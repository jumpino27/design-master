import { ArrowRight, Palette, Sparkles, Trash2 } from "lucide-react";
import type { ProjectSummaryDto } from "@/lib/api";
import { findDesignStyleById } from "@/lib/designStyles";

interface ProjectCardProps {
  project: ProjectSummaryDto;
  onOpen: () => void;
  onDelete: () => void;
}

export function ProjectCard({ project, onOpen, onDelete }: ProjectCardProps) {
  const style = project.brand?.design_style;
  const preset = findDesignStyleById(style?.style_id);
  const colors = project.brand?.colors?.light;
  const preview = project.recent_output_id
    ? `/media/output/${project.recent_output_id}/thumbnail`
    : preset?.heroImage;
  const updated = new Date(project.updated_at);

  return (
    <article
      data-testid="project-card"
      aria-label={project.name}
      className="group min-w-0 overflow-hidden rounded-2xl border border-stone-border bg-stone-surface transition-colors hover:border-stone-muted/60 lg:grid lg:grid-cols-[42%_minmax(0,1fr)]"
    >
      <button
        type="button"
        onClick={onOpen}
        aria-label={`Open ${project.name}`}
        className="relative block h-52 w-full overflow-hidden border-b border-stone-border bg-stone-canvas text-left lg:h-full lg:min-h-[252px] lg:border-b-0 lg:border-r"
      >
        {preview ? (
          <img
            src={preview}
            alt={`${project.name} ${project.recent_output_id ? "latest design" : "style preview"}`}
            className="absolute inset-0 h-full w-full object-cover object-top transition-transform duration-300 motion-safe:group-hover:scale-[1.025]"
          />
        ) : (
          <div className="absolute inset-0 flex items-center justify-center p-6">
            <span
              className="flex h-20 w-20 items-center justify-center rounded-2xl border border-white/15 text-3xl font-bold text-white"
              style={{ backgroundColor: colors?.primary || "#C92746" }}
            >
              {project.brand?.logo?.monogram_text ||
                project.name.charAt(0).toUpperCase()}
            </span>
          </div>
        )}
        <span className="absolute inset-x-0 bottom-0 h-16 bg-gradient-to-t from-black/55 to-transparent" />
        <span className="absolute bottom-3 left-3 rounded-md border border-white/15 bg-black/70 px-2 py-1 text-[10px] font-medium text-white backdrop-blur-sm">
          {project.recent_output_id ? "Latest design" : "Style preview"}
        </span>
        <span className="absolute right-3 top-3 rounded-md border border-white/15 bg-black/70 px-2 py-1 text-[10px] font-medium text-white backdrop-blur-sm">
          Brand v{project.brand_version}
        </span>
      </button>

      <div className="flex min-w-0 flex-col p-4 lg:p-5">
        <h2 className="font-display text-lg font-semibold leading-tight tracking-tight text-stone-ink">
          <button
            type="button"
            onClick={onOpen}
            className="text-left hover:text-crimson-300"
          >
            {project.name}
          </button>
        </h2>
        <p
          className="mt-2 line-clamp-2 text-xs leading-5 text-stone-muted"
          title={project.description || project.purpose}
        >
          {project.description || project.purpose || "Local design project"}
        </p>

        <div className="mt-3 flex min-w-0 items-center justify-between gap-3">
          <span
            className="inline-flex min-w-0 items-center gap-1.5 text-[11px] text-stone-100"
            title={style?.style_name || "Custom style"}
          >
            <Palette className="h-3.5 w-3.5 shrink-0 text-stone-muted" />
            <span className="truncate">
              {style?.style_name || "Custom style"}
            </span>
          </span>
          <span
            className="flex shrink-0 -space-x-1"
            role="img"
            aria-label="Project color palette"
          >
            {[
              colors?.primary || "#C92746",
              colors?.accent || "#1E1D1B",
              colors?.background || "#F4F1EA",
            ].map((hex, index) => (
              <span
                key={index}
                title={hex}
                className="h-4 w-4 rounded-full border-2 border-stone-surface"
                style={{ backgroundColor: hex }}
              />
            ))}
          </span>
        </div>

        <div className="mt-3 flex flex-wrap items-center gap-x-3 gap-y-1 text-[10px] text-stone-muted">
          <span>
            {project.thread_count}{" "}
            {project.thread_count === 1 ? "thread" : "threads"} ·{" "}
            {project.run_count} {project.run_count === 1 ? "run" : "runs"}
          </span>
          <time dateTime={project.updated_at} title={updated.toLocaleString()}>
            {updated.toLocaleDateString(undefined, {
              month: "short",
              day: "numeric",
              year: "numeric",
            })}
          </time>
        </div>

        <div className="mt-auto flex items-center justify-between gap-3 pt-4">
          {project.is_example ? (
            <span className="inline-flex items-center gap-1 text-[10px] text-stone-muted">
              <Sparkles className="h-3 w-3" /> Showcase
            </span>
          ) : (
            <button
              type="button"
              onClick={onDelete}
              aria-label={`Delete project ${project.name}`}
              className="-ml-1.5 rounded-lg p-1.5 text-stone-muted transition-colors hover:bg-red-50 hover:text-red-700"
            >
              <Trash2 className="h-3.5 w-3.5" />
            </button>
          )}
          <button
            type="button"
            onClick={onOpen}
            className="inline-flex items-center gap-2 rounded-lg border border-stone-border bg-stone-card px-3 py-2 text-xs font-semibold text-stone-ink transition-colors hover:border-crimson-500/60 hover:bg-crimson-600 hover:text-white"
          >
            Open workbench <ArrowRight className="h-3.5 w-3.5" />
          </button>
        </div>
      </div>
    </article>
  );
}
