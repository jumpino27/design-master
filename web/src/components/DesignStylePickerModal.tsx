"use client";

import React, { useMemo, useState } from "react";
import {
  Check,
  Eye,
  Layers,
  Palette,
  Search,
  Sparkles,
  Tag,
  X,
} from "lucide-react";
import {
  buildCombinedStyleBrief,
  DESIGN_STYLE_CATEGORIES,
  DESIGN_STYLE_MODIFIERS,
  DESIGN_STYLES,
  DesignStylePreset,
  searchDesignStyles,
} from "@/lib/designStyles";

interface DesignStylePickerModalProps {
  open: boolean;
  onClose: () => void;
  selectedStyleId: string | null;
  selectedModifierIds: string[];
  onSelectStyle: (
    style: DesignStylePreset,
    modifierIds: string[],
    combinedBrief: string
  ) => void;
}

const POPULAR_SEARCH_KEYWORDS = [
  "marketplace",
  "dropshipping",
  "ecommerce",
  "community",
  "anime",
  "movie",
  "game",
  "claymorphism",
  "neobrutalism",
  "glassmorphism",
  "minimalism",
  "swiss grid",
  "editorial",
  "cyberpunk",
  "luxury",
];

export function DesignStylePickerModal({
  open,
  onClose,
  selectedStyleId,
  selectedModifierIds,
  onSelectStyle,
}: DesignStylePickerModalProps) {
  const [searchInput, setSearchInput] = useState("");
  const [committedQuery, setCommittedQuery] = useState("");
  const [activeCategory, setActiveCategory] = useState<string>("All");
  const [draftStyleId, setDraftStyleId] = useState<string | null>(
    selectedStyleId
  );
  const [draftModifiers, setDraftModifiers] =
    useState<string[]>(selectedModifierIds);
  const [expandedPreviewStyle, setExpandedPreviewStyle] =
    useState<DesignStylePreset | null>(null);
  const [showModifiersPanel, setShowModifiersPanel] = useState(false);

  React.useEffect(() => {
    if (open) {
      setDraftStyleId(selectedStyleId);
      setDraftModifiers(selectedModifierIds);
    }
  }, [open, selectedStyleId, selectedModifierIds]);

  const effectiveQuery = (searchInput || committedQuery).trim().toLowerCase();

  const filteredStyles = useMemo(() => {
    return searchDesignStyles(effectiveQuery, activeCategory);
  }, [activeCategory, effectiveQuery]);

  const matchingModifiers = useMemo(() => {
    if (!effectiveQuery) return [];
    return DESIGN_STYLE_MODIFIERS.filter((m) => {
      const hay = `${m.term} ${m.description} ${m.keywords.join(" ")}`.toLowerCase();
      return hay.includes(effectiveQuery);
    });
  }, [effectiveQuery]);

  const activeStyle = useMemo(
    () =>
      DESIGN_STYLES.find((s) => s.id === draftStyleId) ||
      DESIGN_STYLES.find((s) => s.id === selectedStyleId) ||
      null,
    [draftStyleId, selectedStyleId]
  );

  if (!open) return null;

  function handleSearchSubmit(e?: React.FormEvent) {
    if (e) e.preventDefault();
    setCommittedQuery(searchInput.trim());
  }

  function toggleModifier(modId: string) {
    setDraftModifiers((prev) =>
      prev.includes(modId) ? prev.filter((x) => x !== modId) : [...prev, modId]
    );
  }

  function handleConfirmSelection(styleOverride?: DesignStylePreset) {
    const chosen = styleOverride || activeStyle;
    if (!chosen) return;
    const brief = buildCombinedStyleBrief(chosen, draftModifiers);
    onSelectStyle(chosen, draftModifiers, brief);
    onClose();
  }

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-3 sm:p-6 animate-fade-in"
      role="dialog"
      aria-modal="true"
      aria-labelledby="style-picker-modal-title"
    >
      <div className="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-2xl shadow-2xl w-full max-w-6xl max-h-[92vh] flex flex-col overflow-hidden">
        {/* Modal Header */}
        <div className="px-6 py-4 border-b border-[var(--border-subtle)] flex items-start justify-between gap-4 bg-[var(--bg-elevated)]/50">
          <div>
            <div className="flex items-center gap-2">
              <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[11px] font-semibold bg-[var(--accent-primary)]/20 text-purple-200 border border-[var(--accent-primary)]/40">
                <Palette className="w-3.5 h-3.5 text-purple-300" />
                {DESIGN_STYLES.length} Curated Web Design Systems + {DESIGN_STYLE_MODIFIERS.length} Modifiers
              </span>
              <span className="text-[11px] font-medium text-[var(--text-muted)]">
                Required for AI Generation & Handoff
              </span>
            </div>
            <h2
              id="style-picker-modal-title"
              className="text-lg font-semibold tracking-tight text-[var(--text-primary)] mt-1"
            >
              Choose Your Project’s Web Design Style
            </h2>
            <p className="text-xs text-[var(--text-secondary)] mt-0.5">
              Search by design system keyword, inspect the generated hero preview for each style, and lock in the exact definition and visual rules for the AI models.
            </p>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close style picker"
            className="p-2 rounded-lg text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-surface)] transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Search & Filter Controls */}
        <div className="px-6 py-3.5 border-b border-[var(--border-subtle)] bg-[var(--bg-canvas)]/60 space-y-3">
          <form onSubmit={handleSearchSubmit} className="flex items-center gap-2">
            <div className="relative flex-1">
              <Search className="w-4 h-4 text-[var(--text-muted)] absolute left-3.5 top-1/2 -translate-y-1/2 pointer-events-none" />
              <input
                id="style-keyword-search-input"
                type="text"
                value={searchInput}
                onChange={(e) => {
                  setSearchInput(e.target.value);
                  setCommittedQuery(e.target.value);
                }}
                placeholder="Search styles by design system keyword (e.g. brutalism, frosted glass, swiss grid, serif, tactile, neon, pastel, 3d, minimal)..."
                className="w-full pl-10 pr-9 py-2.5 text-sm rounded-xl bg-[var(--bg-surface)] border border-[var(--border-subtle)] text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent-primary)]"
              />
              {searchInput && (
                <button
                  type="button"
                  onClick={() => {
                    setSearchInput("");
                    setCommittedQuery("");
                  }}
                  aria-label="Clear search"
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-[var(--text-muted)] hover:text-[var(--text-primary)]"
                >
                  <X className="w-4 h-4" />
                </button>
              )}
            </div>
            <button
              type="submit"
              data-testid="style-search-btn"
              className="inline-flex items-center gap-1.5 px-4 py-2.5 rounded-xl text-xs font-semibold bg-[var(--accent-primary)] text-white hover:bg-[var(--accent-primary-hover)] transition-colors shadow-sm shrink-0"
            >
              <Search className="w-3.5 h-3.5" />
              Search
            </button>
            <button
              type="button"
              onClick={() => setShowModifiersPanel((v) => !v)}
              className={`inline-flex items-center gap-1.5 px-3.5 py-2.5 rounded-xl text-xs font-medium border transition-colors shrink-0 ${
                showModifiersPanel || draftModifiers.length > 0
                  ? "border-[var(--accent-primary)] bg-[var(--accent-primary)]/10 text-[var(--accent-primary)]"
                  : "border-[var(--border-subtle)] bg-[var(--bg-surface)] text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
              }`}
            >
              <Layers className="w-3.5 h-3.5" />
              Combine Modifiers ({draftModifiers.length})
            </button>
          </form>

          {/* Quick Keyword Chips & Category Tabs */}
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div className="flex items-center gap-1.5 overflow-x-auto pb-0.5">
              {DESIGN_STYLE_CATEGORIES.map((cat) => {
                const active = activeCategory === cat;
                const count =
                  cat === "All"
                    ? DESIGN_STYLES.length
                    : DESIGN_STYLES.filter((s) => s.category === cat).length;
                return (
                  <button
                    key={cat}
                    type="button"
                    onClick={() => setActiveCategory(cat)}
                    className={`px-2.5 py-1 rounded-lg text-[11px] font-medium transition-colors whitespace-nowrap ${
                      active
                        ? "bg-[var(--text-primary)] text-[var(--bg-surface)]"
                        : "bg-[var(--bg-surface)] text-[var(--text-secondary)] border border-[var(--border-subtle)] hover:text-[var(--text-primary)]"
                    }`}
                  >
                    {cat} ({count})
                  </button>
                );
              })}
            </div>

            <div className="flex items-center gap-1 flex-wrap">
              <span className="text-[10px] uppercase tracking-wider text-[var(--text-muted)] mr-1">
                Keywords:
              </span>
              {POPULAR_SEARCH_KEYWORDS.map((kw) => {
                const isCurr = effectiveQuery === kw.toLowerCase();
                return (
                  <button
                    key={kw}
                    type="button"
                    onClick={() => {
                      const next = isCurr ? "" : kw;
                      setSearchInput(next);
                      setCommittedQuery(next);
                    }}
                    className={`px-2 py-0.5 rounded-md text-[10px] font-medium transition-colors ${
                      isCurr
                        ? "bg-[var(--accent-primary)] text-white"
                        : "bg-[var(--bg-elevated)] text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
                    }`}
                  >
                    #{kw}
                  </button>
                );
              })}
            </div>
          </div>

          {/* Matching or Expanded Combinable Modifiers (13 glossary terms) */}
          {(showModifiersPanel || matchingModifiers.length > 0) && (
            <div className="p-3 rounded-xl bg-[var(--bg-surface)] border border-[var(--border-subtle)] space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-[11px] font-semibold text-[var(--text-primary)]">
                  Combinable Layout, Technique & Design System Terms (Optional — combine with your primary style)
                </span>
                {draftModifiers.length > 0 && (
                  <button
                    type="button"
                    onClick={() => setDraftModifiers([])}
                    className="text-[11px] text-[var(--text-muted)] hover:text-[var(--text-primary)]"
                  >
                    Clear modifiers
                  </button>
                )}
              </div>
              <div className="flex flex-wrap gap-1.5">
                {DESIGN_STYLE_MODIFIERS.map((mod) => {
                  const selected = draftModifiers.includes(mod.id);
                  const matchedBySearch =
                    effectiveQuery.length > 0 &&
                    matchingModifiers.some((m) => m.id === mod.id);
                  return (
                    <button
                      key={mod.id}
                      type="button"
                      onClick={() => toggleModifier(mod.id)}
                      title={mod.description}
                      className={`px-2.5 py-1 rounded-lg text-[11px] font-medium border transition-all flex items-center gap-1.5 ${
                        selected
                          ? "bg-[var(--accent-primary)] text-white border-[var(--accent-primary)]"
                          : matchedBySearch
                          ? "bg-[var(--accent-primary)]/15 text-[var(--accent-primary)] border-[var(--accent-primary)]"
                          : "bg-[var(--bg-elevated)]/60 text-[var(--text-secondary)] border-[var(--border-subtle)] hover:text-[var(--text-primary)]"
                      }`}
                    >
                      <span>{mod.term}</span>
                      <span className="text-[9px] opacity-75">({mod.category})</span>
                    </button>
                  );
                })}
              </div>
            </div>
          )}
        </div>

        {/* Results Status Bar */}
        <div className="px-6 py-2 bg-[var(--bg-surface)] border-b border-[var(--border-subtle)] flex items-center justify-between text-xs text-[var(--text-secondary)]">
          <span>
            Showing <strong className="text-[var(--text-primary)]">{filteredStyles.length}</strong> of{" "}
            {DESIGN_STYLES.length} web design styles
            {effectiveQuery ? (
              <>
                {" "}
                matching keyword <strong className="text-[var(--accent-primary)]">&ldquo;{effectiveQuery}&rdquo;</strong>
              </>
            ) : null}
          </span>
          {activeStyle && (
            <span className="text-[11px] text-[var(--text-secondary)] truncate max-w-md">
              Selected:{" "}
              <strong className="text-[var(--text-primary)]">
                #{activeStyle.number} {activeStyle.name}
              </strong>
              {draftModifiers.length > 0 && ` + ${draftModifiers.length} modifier(s)`}
            </span>
          )}
        </div>

        {/* Scrollable Style Cards Grid */}
        <div className="flex-1 overflow-y-auto p-6 bg-[var(--bg-canvas)]/40">
          {filteredStyles.length === 0 ? (
            <div className="py-16 text-center space-y-3">
              <p className="text-sm font-medium text-[var(--text-primary)]">
                No design styles matched &ldquo;{effectiveQuery}&rdquo; in {activeCategory}.
              </p>
              <p className="text-xs text-[var(--text-secondary)]">
                Try searching for broader design system terms like &ldquo;minimal&rdquo;, &ldquo;grid&rdquo;, &ldquo;glass&rdquo;, &ldquo;dark&rdquo;, &ldquo;retro&rdquo;, &ldquo;3d&rdquo;, or &ldquo;serif&rdquo;.
              </p>
              <button
                type="button"
                onClick={() => {
                  setSearchInput("");
                  setCommittedQuery("");
                  setActiveCategory("All");
                }}
                className="px-3.5 py-2 rounded-lg text-xs font-medium bg-[var(--accent-primary)] text-white"
              >
                Reset search & filters
              </button>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {filteredStyles.map((style) => {
                const isSelected = activeStyle?.id === style.id;
                return (
                  <div
                    key={style.id}
                    data-testid={`style-card-${style.number}`}
                    onClick={() => setDraftStyleId(style.id)}
                    className={`group rounded-xl border transition-all cursor-pointer flex flex-col overflow-hidden bg-[var(--bg-surface)] ${
                      isSelected
                        ? "border-2 border-[var(--accent-primary)] ring-2 ring-[var(--accent-primary)]/20 shadow-md"
                        : "border-[var(--border-subtle)] hover:border-[var(--text-muted)] hover:shadow-md"
                    }`}
                  >
                    {/* Hero Page Image Preview (16:9) */}
                    <div className="relative aspect-video w-full bg-[var(--bg-elevated)] overflow-hidden border-b border-[var(--border-subtle)]">
                      <img
                        src={style.heroImage}
                        alt={`${style.name} hero page preview`}
                        loading="lazy"
                        className="w-full h-full object-cover object-top group-hover:scale-[1.03] transition-transform duration-300"
                      />
                      <div className="absolute top-2.5 left-2.5 flex items-center gap-1.5">
                        <span className="px-2 py-0.5 rounded-md text-[10px] font-bold bg-black/75 text-white backdrop-blur-sm">
                          #{style.number}
                        </span>
                        <span className="px-2 py-0.5 rounded-md text-[10px] font-medium bg-black/65 text-white/90 backdrop-blur-sm">
                          {style.category}
                        </span>
                      </div>

                      {/* Palette Swatches + Inspect Button */}
                      <div className="absolute top-2.5 right-2.5 flex items-center gap-1.5">
                        <div className="flex items-center gap-1 px-1.5 py-1 rounded-md bg-black/65 backdrop-blur-sm">
                          <span
                            className="w-2.5 h-2.5 rounded-full border border-white/40"
                            style={{ backgroundColor: style.recommendedBrand.primaryColor }}
                            title={`Primary: ${style.recommendedBrand.primaryColor}`}
                          />
                          <span
                            className="w-2.5 h-2.5 rounded-full border border-white/40"
                            style={{ backgroundColor: style.recommendedBrand.secondaryColor }}
                            title={`Surface: ${style.recommendedBrand.secondaryColor}`}
                          />
                          <span
                            className="w-2.5 h-2.5 rounded-full border border-white/40"
                            style={{ backgroundColor: style.recommendedBrand.accentColor }}
                            title={`Accent: ${style.recommendedBrand.accentColor}`}
                          />
                        </div>
                        <button
                          type="button"
                          onClick={(e) => {
                            e.stopPropagation();
                            setExpandedPreviewStyle(style);
                          }}
                          title="Inspect full hero preview & visual rules"
                          className="p-1.5 rounded-md bg-black/70 text-white hover:bg-black transition-colors"
                        >
                          <Eye className="w-3.5 h-3.5" />
                        </button>
                      </div>

                      {isSelected && (
                        <div className="absolute bottom-2.5 right-2.5 px-2.5 py-1 rounded-full bg-[var(--accent-primary)] text-white text-[10px] font-semibold flex items-center gap-1 shadow">
                          <Check className="w-3 h-3" />
                          Selected
                        </div>
                      )}
                    </div>

                    {/* Card Content */}
                    <div className="p-4 flex-1 flex flex-col justify-between space-y-3">
                      <div className="space-y-1.5">
                        <div className="flex items-center justify-between gap-2">
                          <h3 className="text-sm font-semibold text-[var(--text-primary)]">
                            {style.number}. {style.name}
                          </h3>
                          <span className="text-[10px] font-mono text-[var(--text-muted)]">
                            {style.recommendedBrand.fontHeadline}
                          </span>
                        </div>
                        <p className="text-xs text-[var(--text-secondary)] leading-relaxed">
                          {style.definition}
                        </p>
                        <p className="text-[11px] text-[var(--text-muted)] leading-snug line-clamp-2 pt-0.5">
                          <strong className="text-[var(--text-secondary)]">System rules:</strong>{" "}
                          {style.visualRules}
                        </p>
                        {style.bestFit && (
                          <p className="text-[11px] text-[var(--text-muted)] leading-snug line-clamp-1">
                            <strong className="text-[var(--text-secondary)]">Best fit:</strong>{" "}
                            {style.bestFit}
                          </p>
                        )}
                      </div>

                      <div className="space-y-2.5 pt-1">
                        {/* Keyword Tags */}
                        <div className="flex flex-wrap gap-1">
                          {style.keywords.slice(0, 6).map((kw) => {
                            const matched =
                              effectiveQuery.length > 0 &&
                              kw.toLowerCase().includes(effectiveQuery);
                            return (
                              <span
                                key={kw}
                                className={`inline-flex items-center gap-0.5 px-1.5 py-0.5 rounded text-[10px] ${
                                  matched
                                    ? "bg-[var(--accent-primary)]/20 text-[var(--accent-primary)] font-semibold"
                                    : "bg-[var(--bg-elevated)] text-[var(--text-muted)]"
                                }`}
                              >
                                <Tag className="w-2.5 h-2.5 opacity-60" />
                                {kw}
                              </span>
                            );
                          })}
                        </div>

                        <div className="flex items-center gap-2 pt-1">
                          <button
                            type="button"
                            data-testid={`use-style-btn-${style.number}`}
                            onClick={(e) => {
                              e.stopPropagation();
                              setDraftStyleId(style.id);
                              handleConfirmSelection(style);
                            }}
                            className={`w-full py-2 px-3 rounded-lg text-xs font-semibold transition-colors flex items-center justify-center gap-1.5 ${
                              isSelected
                                ? "bg-[var(--accent-primary)] text-white hover:bg-[var(--accent-primary-hover)]"
                                : "bg-[var(--bg-elevated)] text-[var(--text-primary)] hover:bg-[var(--accent-primary)] hover:text-white"
                            }`}
                          >
                            <Check className="w-3.5 h-3.5" />
                            {isSelected ? "Use This Style" : `Choose ${style.name}`}
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>

        {/* Sticky Modal Footer */}
        <div className="px-6 py-3.5 border-t border-[var(--border-subtle)] bg-[var(--bg-surface)] flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          {activeStyle ? (
            <div className="flex items-center gap-3 min-w-0">
              <img
                src={activeStyle.heroImage}
                alt={activeStyle.name}
                className="w-14 h-9 rounded-lg object-cover border border-[var(--border-subtle)] shrink-0"
              />
              <div className="min-w-0">
                <div className="flex items-center gap-2">
                  <span className="text-xs font-semibold text-[var(--text-primary)]">
                    #{activeStyle.number} {activeStyle.name}
                  </span>
                  {draftModifiers.length > 0 && (
                    <span className="text-[11px] px-2 py-0.5 rounded-full bg-[var(--accent-primary)]/12 text-[var(--accent-primary)] font-medium">
                      + {draftModifiers.length} modifier(s)
                    </span>
                  )}
                </div>
                <p className="text-[11px] text-[var(--text-secondary)] truncate">
                  {activeStyle.definition}
                </p>
              </div>
            </div>
          ) : (
            <span className="text-xs text-[var(--text-muted)]">
              Click any style card above to select your project’s primary design direction.
            </span>
          )}

          <div className="flex items-center gap-2 shrink-0">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 rounded-lg text-xs font-medium border border-[var(--border-subtle)] text-[var(--text-secondary)] hover:text-[var(--text-primary)]"
            >
              Cancel
            </button>
            <button
              type="button"
              data-testid="confirm-style-selection-btn"
              disabled={!activeStyle}
              onClick={() => handleConfirmSelection()}
              className="inline-flex items-center gap-1.5 px-5 py-2 rounded-lg text-xs font-semibold bg-[var(--accent-primary)] text-white hover:bg-[var(--accent-primary-hover)] disabled:opacity-40 transition-colors"
            >
              <Sparkles className="w-3.5 h-3.5" />
              Apply Style to Project
            </button>
          </div>
        </div>
      </div>

      {/* Fullscreen Lightbox for Hero Preview */}
      {expandedPreviewStyle && (
        <div
          className="fixed inset-0 z-60 bg-black/80 backdrop-blur-md flex items-center justify-center p-4 sm:p-8"
          onClick={() => setExpandedPreviewStyle(null)}
        >
          <div
            className="bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-2xl max-w-4xl w-full overflow-hidden shadow-2xl"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="relative aspect-video w-full bg-black">
              <img
                src={expandedPreviewStyle.heroImage}
                alt={expandedPreviewStyle.name}
                className="w-full h-full object-contain"
              />
              <button
                type="button"
                onClick={() => setExpandedPreviewStyle(null)}
                className="absolute top-3 right-3 p-2 rounded-full bg-black/70 text-white hover:bg-black"
              >
                <X className="w-4 h-4" />
              </button>
            </div>
            <div className="p-5 flex flex-col sm:flex-row sm:items-center justify-between gap-4">
              <div className="space-y-1">
                <div className="flex items-center gap-2">
                  <span className="px-2 py-0.5 rounded text-xs font-bold bg-[var(--accent-primary)] text-white">
                    #{expandedPreviewStyle.number}
                  </span>
                  <h3 className="text-base font-semibold text-[var(--text-primary)]">
                    {expandedPreviewStyle.name}
                  </h3>
                  <span className="text-xs text-[var(--text-muted)]">
                    ({expandedPreviewStyle.category})
                  </span>
                </div>
                <p className="text-xs text-[var(--text-secondary)]">
                  {expandedPreviewStyle.definition}
                </p>
                <p className="text-[11px] text-[var(--text-muted)]">
                  <strong>Visual System Rules:</strong> {expandedPreviewStyle.visualRules}
                </p>
              </div>
              <button
                type="button"
                onClick={() => {
                  const s = expandedPreviewStyle;
                  setExpandedPreviewStyle(null);
                  setDraftStyleId(s.id);
                  handleConfirmSelection(s);
                }}
                className="px-4 py-2.5 rounded-xl text-xs font-semibold bg-[var(--accent-primary)] text-white hover:bg-[var(--accent-primary-hover)] shrink-0"
              >
                Choose #{expandedPreviewStyle.number} {expandedPreviewStyle.name}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
