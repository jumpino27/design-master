"use client";

import React, { useMemo, useState } from "react";
import {
  Check,
  Eye,
  Layers,
  Search,
  Sparkles,
  Tag,
  X,
} from "lucide-react";
import { LightDismissDialog } from "@/components/LightDismissDialog";
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
    <>
      <LightDismissDialog
        open={open}
        onClose={onClose}
        title="Choose Your Project’s Web Design Style"
        subtitle={`${DESIGN_STYLES.length} design systems and ${DESIGN_STYLE_MODIFIERS.length} optional modifiers. Search, preview, and choose your project’s direction.`}
        maxWidthClass="max-w-6xl"
        bodyClassName="min-h-0 flex-auto overflow-y-auto overscroll-contain p-0"
      >
        <div className="flex h-[76dvh] min-h-0 flex-col">
          {/* Search & Filter Controls */}
          <div className="shrink-0 px-4 sm:px-6 py-3.5 border-b border-stone-border bg-stone-canvas/60 space-y-3">
            <form onSubmit={handleSearchSubmit} className="flex flex-wrap items-center gap-2">
              <div className="relative flex-1">
                <Search className="w-4 h-4 text-stone-muted absolute left-3.5 top-1/2 -translate-y-1/2 pointer-events-none" />
                <input
                  id="style-keyword-search-input"
                  type="text"
                  value={searchInput}
                  onChange={(e) => {
                    setSearchInput(e.target.value);
                    setCommittedQuery(e.target.value);
                  }}
                  placeholder="Search styles by design system keyword (e.g. brutalism, frosted glass, swiss grid, serif, tactile, neon, pastel, 3d, minimal)..."
                  className="w-full pl-10 pr-9 py-2.5 text-sm rounded-xl bg-stone-surface border border-stone-border text-stone-ink focus:outline-none focus:border-crimson-600"
                />
                {searchInput && (
                  <button
                    type="button"
                    onClick={() => {
                      setSearchInput("");
                      setCommittedQuery("");
                    }}
                    aria-label="Clear search"
                    className="absolute right-3 top-1/2 -translate-y-1/2 text-stone-muted hover:text-stone-ink"
                  >
                    <X className="w-4 h-4" />
                  </button>
                )}
              </div>
              <button
                type="submit"
                data-testid="style-search-btn"
                className="inline-flex items-center gap-1.5 px-4 py-2.5 rounded-xl text-xs font-semibold bg-crimson-600 text-white hover:bg-crimson-700 transition-colors shadow-sm shrink-0"
              >
                <Search className="w-3.5 h-3.5" />
                Search
              </button>
              <button
                type="button"
                onClick={() => setShowModifiersPanel((v) => !v)}
                className={`inline-flex items-center gap-1.5 px-3.5 py-2.5 rounded-xl text-xs font-medium border transition-colors shrink-0 ${
                  showModifiersPanel || draftModifiers.length > 0
                    ? "border-crimson-600 bg-crimson-600/10 text-crimson-300"
                    : "border-stone-border bg-stone-surface text-stone-200 hover:text-stone-ink"
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
                          ? "bg-stone-ink text-stone-surface"
                          : "bg-stone-surface text-stone-200 border border-stone-border hover:text-stone-ink"
                      }`}
                    >
                      {cat} ({count})
                    </button>
                  );
                })}
              </div>

              <div className="flex items-center gap-1 flex-wrap">
                <span className="text-[10px] uppercase tracking-wider text-stone-muted mr-1">
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
                          ? "bg-crimson-600 text-white"
                          : "bg-stone-card text-stone-200 hover:text-stone-ink"
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
              <div className="p-3 rounded-xl bg-stone-surface border border-stone-border space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-[11px] font-semibold text-stone-ink">
                    Combinable Layout, Technique & Design System Terms (Optional — combine with your primary style)
                  </span>
                  {draftModifiers.length > 0 && (
                    <button
                      type="button"
                      onClick={() => setDraftModifiers([])}
                      className="text-[11px] text-stone-muted hover:text-stone-ink"
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
                            ? "bg-crimson-600 text-white border-crimson-600"
                            : matchedBySearch
                            ? "bg-crimson-600/15 text-crimson-300 border-crimson-600"
                            : "bg-stone-card/60 text-stone-200 border-stone-border hover:text-stone-ink"
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
          <div className="px-6 py-2 bg-stone-surface border-b border-stone-border flex items-center justify-between text-xs text-stone-200">
            <span>
              Showing <strong className="text-stone-ink">{filteredStyles.length}</strong> of{" "}
              {DESIGN_STYLES.length} web design styles
              {effectiveQuery ? (
                <>
                  {" "}
                  matching keyword <strong className="text-crimson-300">&ldquo;{effectiveQuery}&rdquo;</strong>
                </>
              ) : null}
            </span>
            {activeStyle && (
              <span className="text-[11px] text-stone-200 truncate max-w-md">
                Selected:{" "}
                <strong className="text-stone-ink">
                  #{activeStyle.number} {activeStyle.name}
                </strong>
                {draftModifiers.length > 0 && ` + ${draftModifiers.length} modifier(s)`}
              </span>
            )}
          </div>

          {/* Scrollable Style Cards Grid */}
          <div className="min-h-0 flex-1 overflow-y-auto p-6 bg-stone-canvas/40">
            {filteredStyles.length === 0 ? (
              <div className="py-16 text-center space-y-3">
                <p className="text-sm font-medium text-stone-ink">
                  No design styles matched &ldquo;{effectiveQuery}&rdquo; in {activeCategory}.
                </p>
                <p className="text-xs text-stone-200">
                  Try searching for broader design system terms like &ldquo;minimal&rdquo;, &ldquo;grid&rdquo;, &ldquo;glass&rdquo;, &ldquo;dark&rdquo;, &ldquo;retro&rdquo;, &ldquo;3d&rdquo;, or &ldquo;serif&rdquo;.
                </p>
                <button
                  type="button"
                  onClick={() => {
                    setSearchInput("");
                    setCommittedQuery("");
                    setActiveCategory("All");
                  }}
                  className="px-3.5 py-2 rounded-lg text-xs font-medium bg-crimson-600 text-white"
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
                      className={`group rounded-xl border transition-all cursor-pointer flex flex-col overflow-hidden bg-stone-surface ${
                        isSelected
                          ? "border-2 border-crimson-600 ring-2 ring-[var(--accent-primary)]/20 shadow-md"
                          : "border-stone-border hover:border-[var(--text-muted)] hover:shadow-md"
                      }`}
                    >
                      {/* Hero Page Image Preview (16:9) */}
                      <div className="relative aspect-video w-full bg-stone-card overflow-hidden border-b border-stone-border">
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
                          <div className="absolute bottom-2.5 right-2.5 px-2.5 py-1 rounded-full bg-crimson-600 text-white text-[10px] font-semibold flex items-center gap-1 shadow">
                            <Check className="w-3 h-3" />
                            Selected
                          </div>
                        )}
                      </div>

                      {/* Card Content */}
                      <div className="p-4 flex-1 flex flex-col justify-between space-y-3">
                        <div className="space-y-1.5">
                          <div className="flex items-center justify-between gap-2">
                            <h3 className="text-sm font-semibold text-stone-ink">
                              {style.number}. {style.name}
                            </h3>
                            <span className="text-[10px] font-mono text-stone-muted">
                              {style.recommendedBrand.fontHeadline}
                            </span>
                          </div>
                          <p className="text-xs text-stone-200 leading-relaxed">
                            {style.definition}
                          </p>
                          <p className="text-[11px] text-stone-muted leading-snug line-clamp-2 pt-0.5">
                            <strong className="text-stone-200">System rules:</strong>{" "}
                            {style.visualRules}
                          </p>
                          {style.bestFit && (
                            <p className="text-[11px] text-stone-muted leading-snug line-clamp-1">
                              <strong className="text-stone-200">Best fit:</strong>{" "}
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
                                      ? "bg-crimson-600/20 text-crimson-300 font-semibold"
                                      : "bg-stone-card text-stone-muted"
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
                                  ? "bg-crimson-600 text-white hover:bg-crimson-700"
                                  : "bg-stone-card text-stone-ink hover:bg-crimson-600 hover:text-white"
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
          <div className="shrink-0 px-4 sm:px-6 py-3.5 border-t border-stone-border bg-stone-surface flex flex-col sm:flex-row sm:items-center justify-between gap-3">
            {activeStyle ? (
              <div className="flex items-center gap-3 min-w-0">
                <img
                  src={activeStyle.heroImage}
                  alt={activeStyle.name}
                  className="w-14 h-9 rounded-lg object-cover border border-stone-border shrink-0"
                />
                <div className="min-w-0">
                  <div className="flex flex-wrap items-center gap-2">
                    <span className="text-xs font-semibold text-stone-ink">
                      #{activeStyle.number} {activeStyle.name}
                    </span>
                    {draftModifiers.length > 0 && (
                      <span className="text-[11px] px-2 py-0.5 rounded-full bg-crimson-600/12 text-crimson-300 font-medium">
                        + {draftModifiers.length} modifier(s)
                      </span>
                    )}
                  </div>
                  <p className="text-[11px] text-stone-200 truncate">
                    {activeStyle.definition}
                  </p>
                </div>
              </div>
            ) : (
              <span className="text-xs text-stone-muted">
                Click any style card above to select your project’s primary design direction.
              </span>
            )}

            <div className="flex items-center gap-2 shrink-0">
              <button
                type="button"
                onClick={onClose}
                className="px-4 py-2 rounded-lg text-xs font-medium border border-stone-border text-stone-200 hover:text-stone-ink"
              >
                Cancel
              </button>
              <button
                type="button"
                data-testid="confirm-style-selection-btn"
                disabled={!activeStyle}
                onClick={() => handleConfirmSelection()}
                className="inline-flex items-center gap-1.5 px-5 py-2 rounded-lg text-xs font-semibold bg-crimson-600 text-white hover:bg-crimson-700 disabled:opacity-40 transition-colors"
              >
                <Sparkles className="w-3.5 h-3.5" />
                Apply Style to Project
              </button>
            </div>
          </div>
        </div>
      </LightDismissDialog>

      <LightDismissDialog
        open={Boolean(expandedPreviewStyle)}
        onClose={() => setExpandedPreviewStyle(null)}
        title={expandedPreviewStyle?.name || "Style preview"}
        maxWidthClass="max-w-4xl"
      >
        {expandedPreviewStyle && (
          <div className="space-y-4">
            <img
              src={expandedPreviewStyle.heroImage}
              alt={expandedPreviewStyle.name}
              className="aspect-video w-full rounded-lg bg-black object-contain"
            />
            <p className="text-sm text-stone-ink">{expandedPreviewStyle.definition}</p>
            <p className="text-xs leading-relaxed text-stone-muted">{expandedPreviewStyle.visualRules}</p>
            <button
              type="button"
              onClick={() => {
                const style = expandedPreviewStyle;
                setExpandedPreviewStyle(null);
                setDraftStyleId(style.id);
                handleConfirmSelection(style);
              }}
              className="rounded-lg bg-crimson-600 px-4 py-2.5 text-xs font-semibold text-white hover:bg-crimson-700"
            >
              Choose #{expandedPreviewStyle.number} {expandedPreviewStyle.name}
            </button>
          </div>
        )}
      </LightDismissDialog>
    </>
  );
}
