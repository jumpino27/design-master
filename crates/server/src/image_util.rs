use crate::error::{AppError, AppResult};
use image::{
    codecs::{png::PngEncoder, webp::WebPEncoder},
    imageops::FilterType,
    ColorType, DynamicImage, GenericImageView, ImageEncoder, ImageFormat, ImageReader, Rgba,
    RgbaImage,
};
use sha2::{Digest, Sha256};
use std::{
    io::Cursor,
    path::{Path, PathBuf},
    sync::OnceLock,
};
use tokio::sync::Semaphore;
use uuid::Uuid;

static IMAGE_SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();

fn image_semaphore() -> &'static Semaphore {
    IMAGE_SEMAPHORE.get_or_init(|| Semaphore::new(4))
}

#[derive(Debug, Clone)]
pub struct ProcessedUpload {
    pub mime_type: String,
    pub extension: String,
    pub width: u32,
    pub height: u32,
    pub byte_size: usize,
    pub checksum_sha256: String,
    pub has_alpha: bool,
    pub original_bytes: Vec<u8>,
    pub model_input_png: Option<Vec<u8>>,
    pub thumbnail_webp: Vec<u8>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ProcessedOutputImage {
    pub mime_type: String,
    pub extension: String,
    pub width: u32,
    pub height: u32,
    pub byte_size: usize,
    pub has_alpha: bool,
    pub thumbnail_webp: Vec<u8>,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

pub fn detect_mime_from_bytes(
    bytes: &[u8],
    filename_hint: Option<&str>,
) -> AppResult<(&'static str, &'static str)> {
    if bytes.len() >= 8 && &bytes[0..8] == b"\x89PNG\r\n\x1a\n" {
        return Ok(("image/png", "png"));
    }
    if bytes.len() >= 3 && &bytes[0..3] == b"\xff\xd8\xff" {
        return Ok(("image/jpeg", "jpg"));
    }
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Ok(("image/webp", "webp"));
    }
    if bytes.len() >= 4 && &bytes[0..4] == b"wOFF" {
        return Ok(("font/woff", "woff"));
    }
    if bytes.len() >= 4 && &bytes[0..4] == b"wOF2" {
        return Ok(("font/woff2", "woff2"));
    }

    // Check if UTF-8 SVG
    let prefix_len = std::cmp::min(bytes.len(), 2048);
    if let Ok(text_prefix) = std::str::from_utf8(&bytes[..prefix_len]) {
        let trimmed = text_prefix.trim_start_matches('\u{feff}').trim_start();
        let lower = trimmed.to_ascii_lowercase();
        if (lower.starts_with("<svg") || (lower.starts_with("<?xml") && lower.contains("<svg")))
            || (filename_hint
                .map(|f| f.to_ascii_lowercase().ends_with(".svg"))
                .unwrap_or(false)
                && lower.contains("<svg"))
        {
            return Ok(("image/svg+xml", "svg"));
        }
    }

    Err(AppError::Validation(
        "Unsupported file signature. Allowed formats: PNG, JPEG, WebP, sanitized SVG, WOFF, WOFF2."
            .into(),
    ))
}

/// Strictly sanitizes SVG text: rejects scripts, event handlers, foreignObject, and external resources.
pub fn sanitize_svg(raw_bytes: &[u8]) -> AppResult<String> {
    let text = std::str::from_utf8(raw_bytes)
        .map_err(|_| AppError::Validation("SVG file must be valid UTF-8 XML.".into()))?;

    let lower = text.to_ascii_lowercase();
    let forbidden_patterns = [
        "<script",
        "javascript:",
        "vbscript:",
        "data:text/html",
        "<foreignobject",
        "<iframe",
        "<object",
        "<embed",
        "onload=",
        "onerror=",
        "onclick=",
        "onmouseover=",
        "onfocus=",
        "onanimation",
        "<!entity",
    ];
    for pat in forbidden_patterns {
        if lower.contains(pat) {
            return Err(AppError::Validation(format!(
                "SVG rejected by sanitizer: contains forbidden script or active pattern '{}'",
                pat
            )));
        }
    }

    // Reject external HTTP/HTTPS references inside href/xlink:href or @import
    if lower.contains("xlink:href=\"http")
        || lower.contains("xlink:href='http")
        || lower.contains("href=\"http")
        || lower.contains("href='http")
        || lower.contains("@import")
    {
        return Err(AppError::Validation(
            "SVG rejected by sanitizer: external network resources are forbidden.".into(),
        ));
    }

    if !lower.contains("<svg") || !lower.contains("</svg>") {
        return Err(AppError::Validation(
            "Invalid SVG structure: missing <svg> root element.".into(),
        ));
    }

    Ok(text.to_string())
}

fn rasterize_svg_sync(sanitized_svg: &str) -> AppResult<(DynamicImage, Vec<u8>)> {
    let opt = resvg::usvg::Options::default();
    let tree = resvg::usvg::Tree::from_str(sanitized_svg, &opt).map_err(|e| {
        AppError::Validation(format!("Failed to parse SVG for rasterization: {}", e))
    })?;

    let size = tree.size().to_int_size();
    let w = size.width().clamp(64, 1024);
    let h = size.height().clamp(64, 1024);

    let mut pixmap = resvg::tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| AppError::Internal("Failed to allocate SVG raster pixmap".into()))?;

    let sx = w as f32 / size.width() as f32;
    let sy = h as f32 / size.height() as f32;
    let scale = sx.min(sy);
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let rgba_img = RgbaImage::from_raw(w, h, pixmap.data().to_vec())
        .ok_or_else(|| AppError::Internal("Failed to convert SVG pixmap to RgbaImage".into()))?;
    let dyn_img = DynamicImage::ImageRgba8(rgba_img);
    let png_bytes = encode_png(&dyn_img)?;
    Ok((dyn_img, png_bytes))
}

pub fn encode_png(img: &DynamicImage) -> AppResult<Vec<u8>> {
    let mut buf = Vec::new();
    let rgba = img.to_rgba8();
    let encoder = PngEncoder::new(&mut buf);
    encoder
        .write_image(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            ColorType::Rgba8.into(),
        )
        .map_err(|e| AppError::Internal(format!("PNG encode failed: {}", e)))?;
    Ok(buf)
}

pub fn encode_webp_thumbnail(img: &DynamicImage, max_dim: u32) -> AppResult<Vec<u8>> {
    let (w, h) = img.dimensions();
    let thumb = if w > max_dim || h > max_dim {
        img.resize(max_dim, max_dim, FilterType::Lanczos3)
    } else {
        img.clone()
    };
    let rgba = thumb.to_rgba8();
    let mut buf = Vec::new();
    let encoder = WebPEncoder::new_lossless(&mut buf);
    encoder
        .encode(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            ColorType::Rgba8.into(),
        )
        .map_err(|e| AppError::Internal(format!("WebP thumbnail encode failed: {}", e)))?;
    Ok(buf)
}

/// Checks whether the decoded image has genuine alpha transparency (pixels with alpha < 250).
pub fn verify_actual_alpha(img: &DynamicImage) -> bool {
    if !img.color().has_alpha() {
        return false;
    }
    let rgba = img.to_rgba8();
    rgba.pixels().any(|p| p.0[3] < 250)
}

pub async fn process_upload_bytes(
    bytes: Vec<u8>,
    filename_hint: Option<String>,
    max_bytes: usize,
    max_megapixels: u64,
) -> AppResult<ProcessedUpload> {
    if bytes.is_empty() {
        return Err(AppError::Validation("Uploaded file is empty.".into()));
    }
    if bytes.len() > max_bytes {
        return Err(AppError::Validation(format!(
            "Upload size ({} bytes) exceeds limit of {} MiB.",
            bytes.len(),
            max_bytes / (1024 * 1024)
        )));
    }

    let _permit = image_semaphore()
        .acquire()
        .await
        .map_err(|_| AppError::Internal("Image worker semaphore closed".into()))?;

    tokio::task::spawn_blocking(move || {
        let (mime, ext) = detect_mime_from_bytes(&bytes, filename_hint.as_deref())?;
        let checksum = sha256_hex(&bytes);

        if mime == "font/woff" || mime == "font/woff2" {
            // Create a small placeholder thumbnail for font files
            let mut placeholder = RgbaImage::from_pixel(160, 120, Rgba([244, 241, 234, 255]));
            for x in 20..140 {
                placeholder.put_pixel(x, 60, Rgba([213, 106, 58, 255]));
            }
            let thumb = encode_webp_thumbnail(&DynamicImage::ImageRgba8(placeholder), 160)?;
            return Ok(ProcessedUpload {
                mime_type: mime.to_string(),
                extension: ext.to_string(),
                width: 0,
                height: 0,
                byte_size: bytes.len(),
                checksum_sha256: checksum,
                has_alpha: false,
                original_bytes: bytes,
                model_input_png: None,
                thumbnail_webp: thumb,
                metadata: serde_json::json!({
                    "kind": "font",
                    "resized_for_model": false
                }),
            });
        }

        if mime == "image/svg+xml" {
            let sanitized = sanitize_svg(&bytes)?;
            let sanitized_bytes = sanitized.as_bytes().to_vec();
            let (dyn_img, png_bytes) = rasterize_svg_sync(&sanitized)?;
            let (w, h) = dyn_img.dimensions();
            let thumb = encode_webp_thumbnail(&dyn_img, 480)?;
            let has_alpha = verify_actual_alpha(&dyn_img);
            return Ok(ProcessedUpload {
                mime_type: mime.to_string(),
                extension: ext.to_string(),
                width: w,
                height: h,
                byte_size: sanitized_bytes.len(),
                checksum_sha256: sha256_hex(&sanitized_bytes),
                has_alpha,
                original_bytes: sanitized_bytes,
                model_input_png: Some(png_bytes),
                thumbnail_webp: thumb,
                metadata: serde_json::json!({
                    "svg_sanitized": true,
                    "rasterized_for_model": true,
                    "model_input_dimensions": { "width": w, "height": h }
                }),
            });
        }

        // Raster decode with dimension verification first
        let reader = ImageReader::new(Cursor::new(&bytes))
            .with_guessed_format()
            .map_err(|e| AppError::Validation(format!("Failed to read image header: {}", e)))?;

        let (w, h) = reader.into_dimensions().map_err(|e| {
            AppError::Validation(format!("Corrupt or unreadable image dimensions: {}", e))
        })?;

        let total_pixels = (w as u64) * (h as u64);
        if w == 0 || h == 0 || total_pixels > max_megapixels {
            return Err(AppError::Validation(format!(
                "Image dimensions {}x{} ({:.1} MP) exceed limit of {} MP.",
                w,
                h,
                total_pixels as f64 / 1_000_000.0,
                max_megapixels / 1_000_000
            )));
        }

        let dyn_img = image::load_from_memory(&bytes).map_err(|e| {
            AppError::Validation(format!("Failed to decode raster image bytes: {}", e))
        })?;

        let has_alpha = verify_actual_alpha(&dyn_img);
        let thumb = encode_webp_thumbnail(&dyn_img, 512)?;

        // Bound model-input derivative to max 2048px on longest axis
        let max_model_dim = 2048;
        let (model_png, resized_meta) = if w > max_model_dim || h > max_model_dim {
            let resized = dyn_img.resize(max_model_dim, max_model_dim, FilterType::Lanczos3);
            let (mw, mh) = resized.dimensions();
            (
                encode_png(&resized)?,
                serde_json::json!({
                    "resized_for_model": true,
                    "original_dimensions": { "width": w, "height": h },
                    "model_input_dimensions": { "width": mw, "height": mh }
                }),
            )
        } else {
            let png = if mime == "image/png" {
                bytes.clone()
            } else {
                encode_png(&dyn_img)?
            };
            (
                png,
                serde_json::json!({
                    "resized_for_model": false,
                    "original_dimensions": { "width": w, "height": h },
                    "model_input_dimensions": { "width": w, "height": h }
                }),
            )
        };

        Ok(ProcessedUpload {
            mime_type: mime.to_string(),
            extension: ext.to_string(),
            width: w,
            height: h,
            byte_size: bytes.len(),
            checksum_sha256: checksum,
            has_alpha,
            original_bytes: bytes,
            model_input_png: Some(model_png),
            thumbnail_webp: thumb,
            metadata: resized_meta,
        })
    })
    .await
    .map_err(|e| AppError::Internal(format!("Blocking image task failed: {}", e)))?
}

pub async fn validate_and_thumbnail_output_bytes(
    bytes: Vec<u8>,
) -> AppResult<ProcessedOutputImage> {
    if bytes.is_empty() {
        return Err(AppError::ProviderError(
            "Provider returned empty image payload.".into(),
        ));
    }

    let _permit = image_semaphore()
        .acquire()
        .await
        .map_err(|_| AppError::Internal("Image worker semaphore closed".into()))?;

    tokio::task::spawn_blocking(move || {
        let (mime, ext) = detect_mime_from_bytes(&bytes, None).map_err(|_| {
            AppError::ProviderError(
                "Returned image bytes do not match a valid image signature.".into(),
            )
        })?;
        let dyn_img = image::load_from_memory(&bytes).map_err(|e| {
            AppError::ProviderError(format!("Failed to decode provider image bytes: {}", e))
        })?;
        let (w, h) = dyn_img.dimensions();
        if w == 0 || h == 0 {
            return Err(AppError::ProviderError(
                "Decoded image has zero dimensions.".into(),
            ));
        }
        let has_alpha = verify_actual_alpha(&dyn_img);
        let thumb = encode_webp_thumbnail(&dyn_img, 640)?;

        let format_check = match mime {
            "image/png" => ImageFormat::Png,
            "image/jpeg" => ImageFormat::Jpeg,
            "image/webp" => ImageFormat::WebP,
            _ => ImageFormat::Png,
        };
        let _ = format_check;

        Ok(ProcessedOutputImage {
            mime_type: mime.to_string(),
            extension: ext.to_string(),
            width: w,
            height: h,
            byte_size: bytes.len(),
            has_alpha,
            thumbnail_webp: thumb,
        })
    })
    .await
    .map_err(|e| AppError::Internal(format!("Output image task panicked: {}", e)))?
}

/// Atomically writes bytes to a target file via a temporary file rename in the same directory.
pub fn atomic_write_file(target: &Path, bytes: &[u8]) -> AppResult<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp_name = format!(
        ".tmp-{}-{}",
        target
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file"),
        Uuid::new_v4()
    );
    let tmp_path: PathBuf = target
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(tmp_name);
    std::fs::write(&tmp_path, bytes)?;
    std::fs::rename(&tmp_path, target)?;
    Ok(())
}

fn parse_hex_rgb(hex: &str, fallback: [u8; 3]) -> [u8; 3] {
    let s = hex.trim().trim_start_matches('#');
    if s.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&s[0..2], 16),
            u8::from_str_radix(&s[2..4], 16),
            u8::from_str_radix(&s[4..6], 16),
        ) {
            return [r, g, b];
        }
    }
    fallback
}

/// Generates a deterministic, clean, high-resolution fixture PNG for automated testing.
/// Marked clearly in metadata as a Test Fixture output.
pub fn generate_deterministic_fixture_png(
    aspect_ratio: &str,
    variant_index: u32,
    primary_hex: &str,
    bg_hex: &str,
    surface_hex: &str,
    transparent: bool,
    seamless: bool,
) -> AppResult<Vec<u8>> {
    let (w, h) = match aspect_ratio {
        "1:1" => (640, 640),
        "16:9" => (800, 450),
        "9:16" => (450, 800),
        "3:2" => (750, 500),
        "2:3" => (500, 750),
        "3:4" => (540, 720),
        "21:9" => (840, 360),
        _ => (720, 540), // 4:3 default
    };

    let primary = parse_hex_rgb(primary_hex, [213, 106, 58]);
    let bg = parse_hex_rgb(bg_hex, [244, 241, 234]);
    let surface = parse_hex_rgb(surface_hex, [250, 249, 246]);

    let mut img = RgbaImage::new(w, h);

    if seamless {
        // Render a genuinely seamless periodic geometric pattern
        for y in 0..h {
            for x in 0..w {
                let u = (x as f32 / w as f32) * std::f32::consts::TAU * 4.0;
                let v = (y as f32 / h as f32) * std::f32::consts::TAU * 4.0;
                let wave = ((u.sin() + v.cos()) * 0.25 + 0.5).clamp(0.0, 1.0);
                let r =
                    (bg[0] as f32 * (1.0 - wave * 0.25) + primary[0] as f32 * (wave * 0.25)) as u8;
                let g =
                    (bg[1] as f32 * (1.0 - wave * 0.25) + primary[1] as f32 * (wave * 0.25)) as u8;
                let b =
                    (bg[2] as f32 * (1.0 - wave * 0.25) + primary[2] as f32 * (wave * 0.25)) as u8;
                let a = if transparent && wave < 0.3 { 0 } else { 255 };
                img.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }
    } else {
        // Fill background (transparent if requested)
        let base_alpha = if transparent { 0 } else { 255 };
        let canvas_color = if variant_index == 4 {
            Rgba([30, 29, 27, base_alpha]) // Bold dark variant
        } else {
            Rgba([bg[0], bg[1], bg[2], base_alpha])
        };

        for y in 0..h {
            for x in 0..w {
                img.put_pixel(x, y, canvas_color);
            }
        }

        // Draw card container in center
        let pad_x = w / 10;
        let pad_y = h / 10;
        let card_color = if variant_index == 4 {
            Rgba([42, 40, 37, 255])
        } else {
            Rgba([surface[0], surface[1], surface[2], 255])
        };

        for y in pad_y..(h - pad_y) {
            for x in pad_x..(w - pad_x) {
                img.put_pixel(x, y, card_color);
            }
        }

        // Draw variant-specific accent bars and CTA button so all 4 directions are visually distinct
        let cta_y_start = h - pad_y - (h / 7);
        let cta_y_end = h - pad_y - (h / 18);
        let cta_x_start = pad_x + (w / 14);
        let cta_x_end = match variant_index {
            1 => w / 2,                // Editorial split layout
            2 => w - pad_x - (w / 14), // Structured full-width CTA
            3 => w - pad_x - (w / 6),  // Minimal centered
            _ => w / 2 + (w / 10),     // Bold dark editorial
        };

        for y in cta_y_start..cta_y_end {
            for x in cta_x_start..cta_x_end {
                img.put_pixel(x, y, Rgba([primary[0], primary[1], primary[2], 255]));
            }
        }

        // Draw editorial right-column visual block for variants 1 and 4
        if variant_index == 1 || variant_index == 4 {
            let split_x = (w * 56) / 100;
            for y in (pad_y + 12)..(h - pad_y - 12) {
                for x in split_x..(w - pad_x - 12) {
                    let shade = ((x + y) % 64) as u8;
                    img.put_pixel(
                        x,
                        y,
                        Rgba([
                            primary[0].saturating_sub(shade / 3),
                            primary[1].saturating_sub(shade / 3),
                            primary[2].saturating_sub(shade / 3),
                            255,
                        ]),
                    );
                }
            }
        }
    }

    encode_png(&DynamicImage::ImageRgba8(img))
}
