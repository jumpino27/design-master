"use client";

import React, {
  Suspense,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import {
  AlertTriangle,
  Archive,
  ArrowLeft,
  Check,
  CheckCircle2,
  ChevronDown,
  ChevronLeft,
  ChevronRight,
  Copy,
  Crosshair,
  Download,
  Edit2,
  Eye,
  FileArchive,
  FileCode2,
  FolderKanban,
  Grid,
  GripHorizontal,
  Image as ImageIcon,
  Layers,
  Layout,
  LayoutGrid,
  Lock,
  Loader2,
  Maximize2,
  Menu,
  MessageSquare,
  Monitor,
  Move,
  Palette,
  PanelLeftClose,
  PanelLeftOpen,
  Paperclip,
  Plus,
  RefreshCw,
  RotateCcw,
  Search,
  Send,
  Settings,
  ShieldCheck,
  Smartphone,
  Sparkles,
  Tablet,
  Trash2,
  Upload,
  X,
  Zap,
  ZoomIn,
  ZoomOut,
} from "lucide-react";
import {
  apiFetch,
  AssetDto,
  AttachmentRole,
  BrandSpec,
  HandoffResponse,
  HighlightAnnotation,
  OutputDto,
  ProjectSummaryDto,
  RunDto,
  SelectedReference,
  SetupStatusDto,
  ThreadChatResponse,
  ThreadDto,
  ThreadMessageDto,
} from "@/lib/api";
import { BrandKitView } from "@/components/workspace/BrandKitView";
import { DesignMdView } from "@/components/workspace/DesignMdView";
import { AssetsLibraryView } from "@/components/workspace/AssetsLibraryView";
import { HandoffDialog } from "@/components/workspace/HandoffDialog";
import { DetailsDrawer } from "@/components/workspace/DetailsDrawer";
import { ImageViewerModal } from "@/components/workspace/ImageViewerModal";
import { FullPagePlanReview } from "@/components/workspace/FullPagePlanReview";
import { LightDismissDialog } from "@/components/LightDismissDialog";

interface ProjectDetailResponse {
  project: {
    id: string;
    name: string;
    description: string;
    purpose: string;
    industry: string;
    target_audience: string;
    audience_needs: string;
    desired_user_action: string;
    archive_status: string;
    created_at: string;
    updated_at: string;
    counts: {
      threads: number;
      assets: number;
      runs: number;
      outputs: number;
    };
  };
  brand_version: number;
  brand: BrandSpec;
  brand_history: Array<{
    version: number;
    brand: BrandSpec;
    created_at: string;
  }>;
}

interface CanvasTabGroup {
  tabKey: string;
  tabLabel: string;
  kind: string;
  aspectRatio: string;
  /**
   * Every output belonging to this tab in chronological order:
   * - For a Full Website tab: all pages, buttons, icon sheets, and extracted transparent assets from that run.
   * - For a Component tab (e.g. Navbar): v1, v2, v3, mobile variants, and any extracted assets side-by-side on the board.
   */
  items: OutputDto[];
  /** Primary outputs (non-extracted) */
  versions: OutputDto[];
  /** Extracted transparent child assets */
  extractedAssets: OutputDto[];
  /** Active run currently generating/editing into this tab, if any */
  activeRunForTab: RunDto | null;
  /** Most recent failed/interrupted run for this tab, if any */
  failedRunForTab: RunDto | null;
}

function normalizeTabKeyClient(raw: string): string {
  const slug = raw
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  return slug || "component";
}

function WorkspaceContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const queryClient = useQueryClient();

  const projectIdParam =
    searchParams.get("projectId") || searchParams.get("project");
  const threadIdParam =
    searchParams.get("threadId") || searchParams.get("thread");
  const viewParam = searchParams.get("view") as
    | "generations"
    | "design_md"
    | "brand_kit"
    | "assets"
    | null;

  // Navigation & Collapsible Sidebar state
  const [activeView, setActiveView] = useState<
    "generations" | "design_md" | "brand_kit" | "assets"
  >(viewParam || "generations");
  const [sidebarCollapsed, setSidebarCollapsed] = useState<boolean>(false);
  const [mobileSidebarOpen, setMobileSidebarOpen] = useState<boolean>(false);
  const [projectDropdownOpen, setProjectDropdownOpen] =
    useState<boolean>(false);
  const [threadSearch, setThreadSearch] = useState<string>("");
  const [showArchivedThreads, setShowArchivedThreads] =
    useState<boolean>(false);
  const [editingThreadId, setEditingThreadId] = useState<string | null>(null);
  const [editingThreadTitle, setEditingThreadTitle] = useState<string>("");

  // Sideways Chat & Composer state
  const [chatInput, setChatInput] = useState<string>("");
  const [selectedQuickReplies, setSelectedQuickReplies] = useState<
    Record<string, string[]>
  >({});
  const [selectedRefs, setSelectedRefs] = useState<SelectedReference[]>([]);
  const [quality, setQuality] = useState<string>("low");
  const [sendingChat, setSendingChat] = useState<boolean>(false);
  const [chatError, setChatError] = useState<string | null>(null);
  const [liveStreamMessage, setLiveStreamMessage] = useState<string | null>(
    null
  );
  const [retryingRunId, setRetryingRunId] = useState<string | null>(null);
  const [uploadingFiles, setUploadingFiles] = useState<boolean>(false);
  const [addAssetModalOpen, setAddAssetModalOpen] = useState<boolean>(false);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  // 3/4 Multi-Tab Interactive Canvas Board state
  const [activeCanvasTabKey, setActiveCanvasTabKey] = useState<string | null>(
    null
  );
  const [selectedOutputByTab, setSelectedOutputByTab] = useState<
    Record<string, string>
  >({});
  const [boardScale, setBoardScale] = useState<number>(1);
  const [boardPan, setBoardPan] = useState<{ x: number; y: number }>({
    x: 0,
    y: 0,
  });
  const [panningBoard, setPanningBoard] = useState<{
    startX: number;
    startY: number;
    origX: number;
    origY: number;
  } | null>(null);
  const canvasBoardRef = useRef<HTMLDivElement | null>(null);

  // 2D Drag-and-Drop positions (`{ x, y }` offsets per output card) + custom card ordering
  const [cardOffsets, setCardOffsets] = useState<
    Record<string, { x: number; y: number }>
  >({});
  const [draggingCard, setDraggingCard] = useState<{
    outputId: string;
    startX: number;
    startY: number;
    origX: number;
    origY: number;
  } | null>(null);

  // Highlight & Edit Region state (`Sunburst 2.5 Medium`)
  const [highlightTargetOutputId, setHighlightTargetOutputId] = useState<
    string | null
  >(null);
  const [isDrawingHighlight, setIsDrawingHighlight] = useState<boolean>(false);
  const [dragStartPct, setDragStartPct] = useState<{
    x: number;
    y: number;
  } | null>(null);
  const [draftHighlightBox, setDraftHighlightBox] = useState<{
    outputId: string;
    tabKey: string;
    tabLabel: string;
    x_pct: number;
    y_pct: number;
    w_pct: number;
    h_pct: number;
  } | null>(null);
  const [highlightComment, setHighlightComment] = useState<string>("");
  const [queuedHighlights, setQueuedHighlights] = useState<
    HighlightAnnotation[]
  >([]);

  // Modals & Drawers state
  const [viewerOutput, setViewerOutput] = useState<OutputDto | null>(null);
  const [detailsDrawerOpen, setDetailsDrawerOpen] = useState<boolean>(false);
  const [detailsRunId, setDetailsRunId] = useState<string | null>(null);
  const [handoffModalOpen, setHandoffModalOpen] = useState<boolean>(false);
  const [activeHandoff, setActiveHandoff] = useState<HandoffResponse | null>(
    null
  );
  const [activeHandoffOutputId, setActiveHandoffOutputId] = useState<
    string | null
  >(null);
  const [activeHandoffOutputIds, setActiveHandoffOutputIds] = useState<
    string[] | null
  >(null);
  const [activeHandoffTabLabel, setActiveHandoffTabLabel] = useState<
    string | null
  >(null);
  const [activeHandoffRunId, setActiveHandoffRunId] = useState<string | null>(
    null
  );
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [checkingConsistency, setCheckingConsistency] =
    useState<boolean>(false);
  const [adoptingTokens, setAdoptingTokens] = useState<boolean>(false);
  const [deletingOutputId, setDeletingOutputId] = useState<string | null>(null);
  const [tabDropdownOpen, setTabDropdownOpen] = useState<boolean>(false);

  const handoffCacheRef = useRef<Record<string, HandoffResponse>>({});
  const tabRowScrollRef = useRef<HTMLDivElement | null>(null);
  const fileInputRef = useRef<HTMLInputElement | null>(null);
  const chatInputRef = useRef<HTMLTextAreaElement | null>(null);
  const chatScrollRef = useRef<HTMLDivElement | null>(null);
  const lastAutoSwitchedRunRef = useRef<string | null>(null);

  const showToast = useCallback((msg: string) => {
    setToastMessage(msg);
    setTimeout(() => {
      setToastMessage((prev) => (prev === msg ? null : prev));
    }, 3400);
  }, []);

  // 1. Fetch setup status & capabilities
  const { data: setupStatus } = useQuery<SetupStatusDto>({
    queryKey: ["setup-status"],
    queryFn: () => apiFetch<SetupStatusDto>("/api/setup/status"),
  });

  useEffect(() => {
    if (setupStatus?.settings?.default_quality) {
      setQuality(setupStatus.settings.default_quality);
    }
  }, [setupStatus?.settings?.default_quality]);

  // 2. Fetch projects list
  const { data: projectsData, isLoading: loadingProjects } = useQuery<{
    projects: ProjectSummaryDto[];
  }>({
    queryKey: ["projects"],
    queryFn: () => apiFetch<{ projects: ProjectSummaryDto[] }>("/api/projects"),
  });

  const activeProjectId = useMemo(() => {
    if (projectIdParam) return projectIdParam;
    if (projectsData?.projects && projectsData.projects.length > 0) {
      return projectsData.projects[0].id;
    }
    return null;
  }, [projectIdParam, projectsData]);

  useEffect(() => {
    if (
      !loadingProjects &&
      projectsData?.projects &&
      projectsData.projects.length === 0
    ) {
      router.replace("/projects");
    }
  }, [loadingProjects, projectsData, router]);

  // 3. Fetch active project detail & brand kit
  const { data: projectDetail, refetch: refetchProjectDetail } =
    useQuery<ProjectDetailResponse>({
      queryKey: ["project-detail", activeProjectId],
      queryFn: () =>
        apiFetch<ProjectDetailResponse>(`/api/projects/${activeProjectId}`),
      enabled: Boolean(activeProjectId),
    });

  // 4. Fetch threads for project
  const { data: threadsData, refetch: refetchThreads } = useQuery<{
    threads: ThreadDto[];
  }>({
    queryKey: [
      "project-threads",
      activeProjectId,
      threadSearch,
      showArchivedThreads,
    ],
    queryFn: () => {
      const params = new URLSearchParams();
      if (threadSearch.trim()) params.set("q", threadSearch.trim());
      if (showArchivedThreads) params.set("include_archived", "true");
      return apiFetch<{ threads: ThreadDto[] }>(
        `/api/projects/${activeProjectId}/threads?${params.toString()}`
      );
    },
    enabled: Boolean(activeProjectId),
  });

  const activeThreadId = useMemo(() => {
    if (
      threadIdParam &&
      threadsData?.threads?.some((t) => t.id === threadIdParam)
    ) {
      return threadIdParam;
    }
    if (threadsData?.threads && threadsData.threads.length > 0) {
      return threadsData.threads[0].id;
    }
    return threadIdParam || null;
  }, [threadIdParam, threadsData]);

  // 5. Fetch project assets
  const { data: assetsData, refetch: refetchAssets } = useQuery<{
    assets: AssetDto[];
  }>({
    queryKey: ["project-assets", activeProjectId],
    queryFn: () =>
      apiFetch<{ assets: AssetDto[] }>(
        `/api/projects/${activeProjectId}/assets`
      ),
    enabled: Boolean(activeProjectId),
  });

  const projectAssets = useMemo(
    () => assetsData?.assets || [],
    [assetsData?.assets]
  );

  // 6. Fetch active thread + runs + conversational messages
  const {
    data: threadDetail,
    refetch: refetchThreadDetail,
    isLoading: loadingThread,
  } = useQuery<{
    thread: ThreadDto;
    runs: RunDto[];
    messages?: ThreadMessageDto[];
  }>({
    queryKey: ["thread-detail", activeThreadId],
    queryFn: () =>
      apiFetch<{
        thread: ThreadDto;
        runs: RunDto[];
        messages?: ThreadMessageDto[];
      }>(`/api/threads/${activeThreadId}`),
    enabled: Boolean(activeThreadId),
    refetchInterval: (query) => {
      const rList = query.state.data?.runs || [];
      const hasActive = rList.some((r) =>
        ["queued", "planning", "ready", "rendering", "saving"].includes(
          r.status
        )
      );
      return hasActive ? 1400 : false;
    },
  });

  const runs = useMemo(() => threadDetail?.runs || [], [threadDetail?.runs]);
  const messages = useMemo(
    () => threadDetail?.messages || [],
    [threadDetail?.messages]
  );

  // Active generating run check (locks chat while AI is planning or rendering images)
  const activeGeneratingRun: RunDto | null = useMemo(() => {
    return (
      runs.find((r) =>
        ["queued", "planning", "ready", "rendering", "saving"].includes(
          r.status
        )
      ) || null
    );
  }, [runs]);

  const isChatLocked = sendingChat || Boolean(activeGeneratingRun);

  const latestRun: RunDto | null = useMemo(() => {
    if (runs.length === 0) return null;
    if (detailsRunId) {
      const f = runs.find((r) => r.id === detailsRunId);
      if (f) return f;
    }
    return runs[runs.length - 1];
  }, [runs, detailsRunId]);

  // 7. Build Dynamic Canvas Tabs from all runs & outputs in this thread
  // Rule:
  // - When creating a Full Website (`run.mode === "full_page"` or a run with `run.target_tab_key`),
  //   ALL outputs produced by that run (landing page, buttons, icon sheets, phone mockups, extracted transparent assets)
  //   stay together inside ONE tab!
  // - Separate tabs are for different components/sections (e.g. `Navbar`, `Hero Section`).
  // - Recalling/editing a component or tab adds the newly generated output(s) inside that SAME tab next to existing ones!
  const canvasTabs: CanvasTabGroup[] = useMemo(() => {
    const groupsMap = new Map<string, CanvasTabGroup>();

    for (const run of runs) {
      const isActiveRun = [
        "queued",
        "planning",
        "awaiting_plan_review",
        "ready",
        "rendering",
        "saving",
      ].includes(run.status);

      const isFailedRun = [
        "failed",
        "cancelled",
        "interrupted",
        "outcome_unknown",
        "partial",
      ].includes(run.status);

      const isUnifiedRunTab =
        run.mode === "full_page" || Boolean(run.target_tab_key?.trim());

      const rawUnifiedKey = isUnifiedRunTab
        ? normalizeTabKeyClient(
            run.target_tab_key?.trim() ||
              (run.mode === "full_page"
                ? "full-website"
                : run.outputs[0]?.tab_key ||
                  run.outputs[0]?.title ||
                  "component")
          )
        : null;
      const unifiedKey =
        rawUnifiedKey === "full-website-desktop" ? "full-website" : rawUnifiedKey;

      const unifiedLabel = isUnifiedRunTab
        ? (run.target_tab_label?.trim() === "Full Website (Desktop)"
            ? "Full Website"
            : run.target_tab_label?.trim()) ||
          (run.mode === "full_page"
            ? "Full Website"
            : run.outputs[0]?.tab_label ||
              run.outputs[0]?.title?.replace(/^0\d\s*/, "") ||
              "Component")
        : null;

      if (run.outputs.length === 0 && (isActiveRun || isFailedRun)) {
        const targetKey =
          unifiedKey ||
          normalizeTabKeyClient(
            run.mode === "full_page" ? "full-website" : "component"
          );
        const targetLabel =
          unifiedLabel ||
          (run.mode === "full_page" ? "Full Website" : "New Component");

        let grp = groupsMap.get(targetKey);
        if (!grp) {
          grp = {
            tabKey: targetKey,
            tabLabel: targetLabel,
            kind: run.mode,
            aspectRatio: run.model_settings?.aspect_ratio || "16:9",
            items: [],
            versions: [],
            extractedAssets: [],
            activeRunForTab: isActiveRun ? run : null,
            failedRunForTab: isFailedRun ? run : null,
          };
          groupsMap.set(targetKey, grp);
        } else {
          if (isActiveRun) {
            grp.activeRunForTab = run;
            grp.failedRunForTab = null;
          } else if (isFailedRun) {
            grp.failedRunForTab = run;
          }
        }
        continue;
      }

      let seenPrimaryUiInComponentRun = false;
      for (const out of run.outputs) {
        const isAssetOut =
          Boolean(out.extracted_from_output_id) ||
          out.kind === "asset" ||
          Boolean(out.transparent_background);
        if (run.mode === "component" && !isAssetOut) {
          if (seenPrimaryUiInComponentRun) {
            continue;
          }
          seenPrimaryUiInComponentRun = true;
        }
        const rawOutKey =
          unifiedKey ||
          normalizeTabKeyClient(out.tab_key?.trim() || out.title);
        const tabKey =
          rawOutKey === "full-website-desktop" ? "full-website" : rawOutKey;
        const tabLabel =
          unifiedLabel ||
          (out.tab_label?.trim() === "Full Website (Desktop)"
            ? "Full Website"
            : out.tab_label?.trim()) ||
          out.title.replace(/^0\d\s*/, "");

        let grp = groupsMap.get(tabKey);
        if (!grp) {
          grp = {
            tabKey,
            tabLabel,
            kind: run.mode === "full_page" ? "full_page" : out.kind,
            aspectRatio: out.aspect_ratio,
            items: [],
            versions: [],
            extractedAssets: [],
            activeRunForTab: null,
            failedRunForTab: null,
          };
          groupsMap.set(tabKey, grp);
        }

        if (tabLabel && !grp.tabLabel) {
          grp.tabLabel = tabLabel;
        }
        if (!grp.items.some((existing) => existing.id === out.id)) {
          grp.items.push(out);
        }
        if (isAssetOut) {
          if (!grp.extractedAssets.some((existing) => existing.id === out.id)) {
            grp.extractedAssets.push(out);
          }
        } else {
          if (!grp.versions.some((existing) => existing.id === out.id)) {
            grp.versions.push(out);
          }
        }
        if (isActiveRun) {
          grp.activeRunForTab = run;
          grp.failedRunForTab = null;
        } else if (isFailedRun) {
          grp.failedRunForTab = run;
        } else if (run.status === "completed") {
          grp.failedRunForTab = null;
        }
      }
    }

    const resultGroups = Array.from(groupsMap.values()).filter(
      (grp) =>
        grp.items.length > 0 ||
        Boolean(grp.activeRunForTab) ||
        Boolean(grp.failedRunForTab)
    );

    // Order each tab's items so primary UI views (Desktop first, then Mobile/Tablet right next to it)
    // appear at the top of the tab board, followed by extracted transparent assets.
    for (const grp of resultGroups) {
      grp.items.sort((a, b) => {
        const aIsAsset =
          Boolean(a.extracted_from_output_id) ||
          a.kind === "asset" ||
          Boolean(a.transparent_background);
        const bIsAsset =
          Boolean(b.extracted_from_output_id) ||
          b.kind === "asset" ||
          Boolean(b.transparent_background);
        if (aIsAsset !== bIsAsset) {
          return aIsAsset ? 1 : -1;
        }
        if (!aIsAsset && !bIsAsset) {
          const aIsMobile = a.aspect_ratio === "9:16" || a.aspect_ratio === "3:4";
          const bIsMobile = b.aspect_ratio === "9:16" || b.aspect_ratio === "3:4";
          if (aIsMobile !== bIsMobile) {
            return aIsMobile ? 1 : -1;
          }
        }
        return 0;
      });
    }

    return resultGroups;
  }, [runs]);

  // Global mouse drag listener for 2D Canvas Card dragging & 2D Canvas Panning
  useEffect(() => {
    if (!draggingCard && !panningBoard) return;
    const handleMouseMove = (e: MouseEvent) => {
      if (draggingCard) {
        const dx = (e.clientX - draggingCard.startX) / boardScale;
        const dy = (e.clientY - draggingCard.startY) / boardScale;
        setCardOffsets((prev) => ({
          ...prev,
          [draggingCard.outputId]: {
            x: Math.round(draggingCard.origX + dx),
            y: Math.round(draggingCard.origY + dy),
          },
        }));
      } else if (panningBoard) {
        const dx = e.clientX - panningBoard.startX;
        const dy = e.clientY - panningBoard.startY;
        setBoardPan({
          x: Math.round(panningBoard.origX + dx),
          y: Math.round(panningBoard.origY + dy),
        });
      }
    };
    const handleMouseUp = () => {
      setDraggingCard(null);
      setPanningBoard(null);
    };
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [draggingCard, panningBoard, boardScale]);

  // Ensure activeCanvasTabKey points to a valid tab (or auto-switches when a new run targets a tab)
  useEffect(() => {
    if (canvasTabs.length === 0) {
      if (activeCanvasTabKey !== "__design_md__") {
        setActiveCanvasTabKey(null);
      }
      return;
    }

    const lastRun = runs[runs.length - 1];
    if (lastRun && lastAutoSwitchedRunRef.current !== lastRun.id) {
      lastAutoSwitchedRunRef.current = lastRun.id;
      const runTargetKey = lastRun.target_tab_key
        ? normalizeTabKeyClient(lastRun.target_tab_key)
        : lastRun.outputs[0]?.tab_key
        ? normalizeTabKeyClient(lastRun.outputs[0].tab_key!)
        : null;
      if (runTargetKey && canvasTabs.some((t) => t.tabKey === runTargetKey)) {
        setActiveCanvasTabKey(runTargetKey);
        return;
      }
    }

    if (
      !activeCanvasTabKey ||
      (activeCanvasTabKey !== "__all__" &&
        activeCanvasTabKey !== "__design_md__" &&
        !canvasTabs.some((t) => t.tabKey === activeCanvasTabKey))
    ) {
      setActiveCanvasTabKey(canvasTabs[canvasTabs.length - 1].tabKey);
    }
  }, [canvasTabs, runs, activeCanvasTabKey]);

  const currentTabGroup: CanvasTabGroup | null = useMemo(() => {
    if (!activeCanvasTabKey || activeCanvasTabKey === "__all__") return null;
    return canvasTabs.find((t) => t.tabKey === activeCanvasTabKey) || null;
  }, [canvasTabs, activeCanvasTabKey]);

  // Currently selected card inside the active tab
  const activeTabOutput: OutputDto | null = useMemo(() => {
    if (!currentTabGroup || currentTabGroup.items.length === 0) return null;
    const chosenId = selectedOutputByTab[currentTabGroup.tabKey];
    if (chosenId) {
      const found = currentTabGroup.items.find((v) => v.id === chosenId);
      if (found) return found;
    }
    const completed = currentTabGroup.items.filter(
      (v) => v.status === "completed"
    );
    if (completed.length > 0) {
      return completed[completed.length - 1];
    }
    return currentTabGroup.items[currentTabGroup.items.length - 1];
  }, [currentTabGroup, selectedOutputByTab]);

  // Auto-scroll chat history when messages or generation status update
  useEffect(() => {
    if (chatScrollRef.current) {
      chatScrollRef.current.scrollTop = chatScrollRef.current.scrollHeight;
    }
  }, [messages.length, sendingChat, activeGeneratingRun?.status]);

  // 8. Real-time SSE subscription for any active run in the thread
  useEffect(() => {
    if (!activeGeneratingRun) {
      setLiveStreamMessage(null);
      return;
    }

    const es = new EventSource(`/api/runs/${activeGeneratingRun.id}/events`);
    const handleEvent = (ev?: MessageEvent) => {
      if (ev?.data) {
        try {
          const parsed = JSON.parse(ev.data);
          if (parsed?.message) {
            setLiveStreamMessage(parsed.message);
          }
        } catch {
          // ignore non-JSON keep-alive frames
        }
      }
      refetchThreadDetail();
    };

    es.onmessage = handleEvent;
    es.addEventListener("run_planning", handleEvent as EventListener);
    es.addEventListener("planning_started", handleEvent as EventListener);
    es.addEventListener("planning_progress", handleEvent as EventListener);
    es.addEventListener("planning_failed", handleEvent as EventListener);
    es.addEventListener("plan_awaiting_review", handleEvent as EventListener);
    es.addEventListener("run_ready", handleEvent as EventListener);
    es.addEventListener("rendering_queued", handleEvent as EventListener);
    es.addEventListener("item_rendering", handleEvent as EventListener);
    es.addEventListener("item_progress", handleEvent as EventListener);
    es.addEventListener("item_saving", handleEvent as EventListener);
    es.addEventListener("item_completed", handleEvent as EventListener);
    es.addEventListener("item_failed", handleEvent as EventListener);
    es.addEventListener("run_finished", handleEvent as EventListener);
    es.addEventListener("run_status_changed", handleEvent as EventListener);

    return () => {
      es.close();
    };
  }, [activeGeneratingRun, refetchThreadDetail]);

  // 9. Pre-fetch handoffs for completed outputs so `Copy Prompt` is instant
  useEffect(() => {
    for (const run of runs) {
      for (const out of run.outputs) {
        if (out.status === "completed") {
          const cacheKey = `output:${out.id}`;
          if (!handoffCacheRef.current[cacheKey]) {
            apiFetch<{ handoff: HandoffResponse }>(
              `/api/outputs/${out.id}/handoff`,
              {
                method: "POST",
                body: JSON.stringify({}),
              }
            )
              .then((res) => {
                handoffCacheRef.current[cacheKey] = res.handoff;
              })
              .catch(() => {});
          }
        }
      }
    }
  }, [runs]);

  // 10. Multi-file upload helper for paste (`⌘V`) / drag-drop / file picker
  const uploadAndAttachFiles = useCallback(
    async (
      files: File[],
      preferredRole?: AttachmentRole,
      notes: string = ""
    ) => {
      if (!activeProjectId || files.length === 0 || isChatLocked) return;
      setUploadingFiles(true);
      setChatError(null);

      try {
        const newlyAttached: SelectedReference[] = [];
        const hasCurrentScreenshot = selectedRefs.some(
          (r) => r.role === "current_screenshot"
        );

        for (let i = 0; i < files.length; i++) {
          const file = files[i];
          const roleToUse: AttachmentRole =
            preferredRole ||
            (!hasCurrentScreenshot && i === 0
              ? "current_screenshot"
              : "style_reference");

          const fd = new FormData();
          fd.append("role", roleToUse);
          fd.append("label", file.name || `Pasted reference ${i + 1}`);
          fd.append(
            "placement_notes",
            notes ||
              (roleToUse === "current_screenshot"
                ? "Current UI component screenshot to polish, enhance, and extend"
                : "Visual reference / asset for this thread")
          );
          fd.append(
            "preserve_as_is",
            roleToUse === "brand_logo" || roleToUse === "include_asset"
              ? "true"
              : "false"
          );
          fd.append("file", file);

          const res = await apiFetch<{ asset: AssetDto }>(
            `/api/projects/${activeProjectId}/assets`,
            {
              method: "POST",
              body: fd,
            }
          );

          newlyAttached.push({
            asset_id: res.asset.id,
            role: res.asset.role,
            label: res.asset.label,
            placement_notes: res.asset.placement_notes,
            preserve_as_is: res.asset.preserve_as_is,
          });
        }

        await refetchAssets();

        setSelectedRefs((prev) => {
          const updated = [...prev];
          for (const item of newlyAttached) {
            if (!updated.some((r) => r.asset_id === item.asset_id)) {
              updated.push(item);
            }
          }
          return updated;
        });

        showToast(
          newlyAttached.length === 1
            ? `Attached "${newlyAttached[0].label}" to chat`
            : `Attached ${newlyAttached.length} images to chat`
        );
      } catch (err: unknown) {
        setChatError(
          err instanceof Error ? err.message : "Failed to upload image"
        );
      } finally {
        setUploadingFiles(false);
      }
    },
    [activeProjectId, isChatLocked, selectedRefs, refetchAssets, showToast]
  );

  // Global clipboard paste (`Cmd+V` / `Ctrl+V`) supporting multiple images
  useEffect(() => {
    const handleGlobalPaste = (e: ClipboardEvent) => {
      if (activeView !== "generations" || !activeProjectId || isChatLocked)
        return;
      const items = e.clipboardData?.items;
      if (!items) return;

      const imageFiles: File[] = [];
      for (let i = 0; i < items.length; i++) {
        const item = items[i];
        if (item.kind === "file" && item.type.startsWith("image/")) {
          const f = item.getAsFile();
          if (f) {
            const ext = item.type.split("/")[1] || "png";
            imageFiles.push(
              new File([f], `pasted-${Date.now()}-${i + 1}.${ext}`, {
                type: item.type,
              })
            );
          }
        }
      }

      if (imageFiles.length > 0) {
        e.preventDefault();
        uploadAndAttachFiles(imageFiles);
      }
    };

    window.addEventListener("paste", handleGlobalPaste);
    return () => window.removeEventListener("paste", handleGlobalPaste);
  }, [activeView, activeProjectId, isChatLocked, uploadAndAttachFiles]);

  const handleAttachExistingAsset = useCallback(
    (asset: AssetDto, overrideRole?: AttachmentRole, overrideNotes?: string) => {
      const role = overrideRole || asset.role;
      const notes =
        overrideNotes !== undefined ? overrideNotes : asset.placement_notes;
      setSelectedRefs((prev) => {
        const next = [...prev];
        const idx = next.findIndex((r) => r.asset_id === asset.id);
        const entry: SelectedReference = {
          asset_id: asset.id,
          role,
          label: asset.label,
          placement_notes: notes,
          preserve_as_is: asset.preserve_as_is,
        };
        if (idx >= 0) next[idx] = entry;
        else next.push(entry);
        return next;
      });
      setActiveView("generations");
      showToast(`Attached "${asset.label}" to chat`);
    },
    [showToast]
  );

  // Send a conversational message or trigger generation/edit via `POST /api/threads/:id/chat`
  const sendChatMessage = useCallback(
    async (opts?: {
      overrideText?: string;
      overrideHighlights?: HighlightAnnotation[];
      forceGenerate?: boolean;
    }) => {
      if (!activeThreadId || isChatLocked) return;
      const textToSend = (opts?.overrideText ?? chatInput).trim();
      const highlightsToSend = opts?.overrideHighlights ?? queuedHighlights;

      if (!textToSend && highlightsToSend.length === 0) return;

      setSendingChat(true);
      setChatError(null);

      try {
        const res = await apiFetch<ThreadChatResponse>(
          `/api/threads/${activeThreadId}/chat`,
          {
            method: "POST",
            body: JSON.stringify({
              message: textToSend,
              selected_references: selectedRefs,
              highlight_annotations: highlightsToSend,
              active_tab_key:
                activeCanvasTabKey === "__all__" ? null : activeCanvasTabKey,
              active_output_id: activeTabOutput?.id || null,
              quality_preference: quality,
              force_generate: opts?.forceGenerate || false,
            }),
          }
        );

        if (opts?.overrideText === undefined) {
          setChatInput("");
        }
        setQueuedHighlights([]);
        setDraftHighlightBox(null);
        setHighlightComment("");
        setHighlightTargetOutputId(null);
        setSelectedRefs([]);

        await refetchThreadDetail();
        await refetchProjectDetail();

        // If the AI triggered or edited a tab, automatically navigate the user to that tab!
        if (res.target_tab_key) {
          const normKey = normalizeTabKeyClient(res.target_tab_key);
          setActiveCanvasTabKey(normKey);
          showToast(
            highlightsToSend.length > 0
              ? `Editing highlighted region on "${
                  res.target_tab_label || normKey
                }" with Sunburst 2.5 Medium...`
              : `Generating inside "${res.target_tab_label || normKey}" tab...`
          );
        }
      } catch (err: unknown) {
        setChatError(
          err instanceof Error ? err.message : "Failed to send chat message"
        );
      } finally {
        setSendingChat(false);
      }
    },
    [
      activeThreadId,
      isChatLocked,
      chatInput,
      queuedHighlights,
      selectedRefs,
      activeCanvasTabKey,
      activeTabOutput?.id,
      quality,
      refetchThreadDetail,
      refetchProjectDetail,
      showToast,
    ]
  );

  // Highlight region mouse handlers on a specific image card
  const handleImageMouseDown = (
    e: React.MouseEvent<HTMLDivElement>,
    out: OutputDto,
    tabKey: string,
    tabLabel: string
  ) => {
    if (highlightTargetOutputId !== out.id) return;
    e.preventDefault();
    e.stopPropagation();
    const rect = e.currentTarget.getBoundingClientRect();
    const x = Math.max(
      0,
      Math.min(100, ((e.clientX - rect.left) / rect.width) * 100)
    );
    const y = Math.max(
      0,
      Math.min(100, ((e.clientY - rect.top) / rect.height) * 100)
    );
    setIsDrawingHighlight(true);
    setDragStartPct({ x, y });
    setDraftHighlightBox({
      outputId: out.id,
      tabKey,
      tabLabel,
      x_pct: Math.round(x * 10) / 10,
      y_pct: Math.round(y * 10) / 10,
      w_pct: 14,
      h_pct: 14,
    });
  };

  const handleImageMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!isDrawingHighlight || !dragStartPct || !draftHighlightBox) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const curX = Math.max(
      0,
      Math.min(100, ((e.clientX - rect.left) / rect.width) * 100)
    );
    const curY = Math.max(
      0,
      Math.min(100, ((e.clientY - rect.top) / rect.height) * 100)
    );
    const left = Math.min(dragStartPct.x, curX);
    const top = Math.min(dragStartPct.y, curY);
    const width = Math.max(4, Math.abs(curX - dragStartPct.x));
    const height = Math.max(4, Math.abs(curY - dragStartPct.y));

    setDraftHighlightBox({
      ...draftHighlightBox,
      x_pct: Math.round(left * 10) / 10,
      y_pct: Math.round(top * 10) / 10,
      w_pct: Math.round(width * 10) / 10,
      h_pct: Math.round(height * 10) / 10,
    });
  };

  const handleImageMouseUp = () => {
    if (isDrawingHighlight) {
      setIsDrawingHighlight(false);
    }
  };

  const handleSubmitHighlightToChat = async (immediateEdit: boolean) => {
    if (!draftHighlightBox || !highlightComment.trim()) return;
    const annotation: HighlightAnnotation = {
      output_id: draftHighlightBox.outputId,
      tab_key: draftHighlightBox.tabKey,
      tab_label: draftHighlightBox.tabLabel,
      x_pct: draftHighlightBox.x_pct,
      y_pct: draftHighlightBox.y_pct,
      w_pct: draftHighlightBox.w_pct,
      h_pct: draftHighlightBox.h_pct,
      comment: highlightComment.trim(),
    };

    if (immediateEdit) {
      await sendChatMessage({
        overrideText: `In ${draftHighlightBox.tabLabel}, edit the highlighted region: ${highlightComment.trim()}`,
        overrideHighlights: [annotation],
        forceGenerate: true,
      });
    } else {
      setQueuedHighlights((prev) => [...prev, annotation]);
      setDraftHighlightBox(null);
      setHighlightComment("");
      setHighlightTargetOutputId(null);
      chatInputRef.current?.focus();
      showToast(
        "Highlight attached to chat — add any extra notes or press Enter to edit with Sunburst Medium!"
      );
    }
  };

  // Thread CRUD handlers
  async function handleCreateThread() {
    if (!activeProjectId) return;
    try {
      const res = await apiFetch<{ thread: ThreadDto }>(
        `/api/projects/${activeProjectId}/threads`,
        {
          method: "POST",
          body: JSON.stringify({
            title: "New design thread",
          }),
        }
      );
      await refetchThreads();
      setActiveCanvasTabKey(null);
      router.push(
        `/workspace?projectId=${activeProjectId}&threadId=${res.thread.id}`
      );
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to create new thread"
      );
    }
  }

  async function handleSaveThreadRename(threadId: string) {
    if (!editingThreadTitle.trim()) {
      setEditingThreadId(null);
      return;
    }
    try {
      await apiFetch(`/api/threads/${threadId}`, {
        method: "PUT",
        body: JSON.stringify({ title: editingThreadTitle.trim() }),
      });
      setEditingThreadId(null);
      await refetchThreads();
      await refetchThreadDetail();
    } catch (err: unknown) {
      showToast(err instanceof Error ? err.message : "Failed to rename thread");
    }
  }

  async function handleToggleThreadArchive(thread: ThreadDto) {
    const nextStatus =
      thread.archive_status === "archived" ? "active" : "archived";
    try {
      await apiFetch(`/api/threads/${thread.id}`, {
        method: "PUT",
        body: JSON.stringify({ archive_status: nextStatus }),
      });
      await refetchThreads();
      await refetchThreadDetail();
      showToast(
        nextStatus === "archived" ? "Thread archived" : "Thread reopened"
      );
    } catch (err: unknown) {
      showToast(err instanceof Error ? err.message : "Failed to update thread");
    }
  }

  // Run & Output actions
  async function handleCancelRun(runId: string) {
    try {
      await apiFetch(`/api/runs/${runId}/cancel`, {
        method: "POST",
        body: JSON.stringify({}),
      });
      await refetchThreadDetail();
      showToast("Run cancelled");
    } catch (err: unknown) {
      showToast(err instanceof Error ? err.message : "Failed to cancel run");
    }
  }

  async function handleRetryOutput(outputId: string) {
    try {
      await apiFetch(`/api/outputs/${outputId}/retry`, {
        method: "POST",
        body: JSON.stringify({}),
      });
      await refetchThreadDetail();
      showToast("Retrying output item...");
    } catch (err: unknown) {
      showToast(err instanceof Error ? err.message : "Failed to retry output");
    }
  }

  async function handleExtractMissingAssets(outputId: string) {
    setLiveStreamMessage(
      "Auditing rendered UI image & extracting all visual assets onto transparent backgrounds..."
    );
    try {
      await apiFetch(`/api/outputs/${outputId}/extract-assets`, {
        method: "POST",
        body: JSON.stringify({}),
      });
      await refetchThreadDetail();
      showToast("Extracting missing transparent assets into this tab...");
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to extract visual assets"
      );
    }
  }

  async function handleRetryRun(runId: string) {
    setRetryingRunId(runId);
    setLiveStreamMessage("Retrying AI design plan & image generation...");
    try {
      await apiFetch(`/api/runs/${runId}/retry`, {
        method: "POST",
        body: JSON.stringify({}),
      });
      await refetchThreadDetail();
      showToast("Retrying generation run...");
    } catch (err: unknown) {
      showToast(err instanceof Error ? err.message : "Failed to retry run");
    } finally {
      setRetryingRunId(null);
    }
  }

  // Copy Prompt for a SINGLE Image Card
  async function handleCopySingleOutput(output: OutputDto) {
    const cacheKey = `output:${output.id}`;
    const cached = handoffCacheRef.current[cacheKey];

    if (cached) {
      try {
        await navigator.clipboard.writeText(cached.clipboard_text);
        setCopiedId(output.id);
        setTimeout(() => setCopiedId(null), 2200);
        showToast(`Copied prompt for "${output.title}"!`);
        return;
      } catch {
        setActiveHandoff(cached);
        setActiveHandoffOutputId(output.id);
        setActiveHandoffOutputIds(null);
        setActiveHandoffTabLabel(null);
        setActiveHandoffRunId(output.run_id);
        setHandoffModalOpen(true);
        return;
      }
    }

    try {
      const res = await apiFetch<{ handoff: HandoffResponse }>(
        `/api/outputs/${output.id}/handoff`,
        {
          method: "POST",
          body: JSON.stringify({}),
        }
      );
      handoffCacheRef.current[cacheKey] = res.handoff;
      try {
        await navigator.clipboard.writeText(res.handoff.clipboard_text);
        setCopiedId(output.id);
        setTimeout(() => setCopiedId(null), 2200);
        showToast(`Copied prompt for "${output.title}"!`);
      } catch {
        setActiveHandoff(res.handoff);
        setActiveHandoffOutputId(output.id);
        setActiveHandoffOutputIds(null);
        setActiveHandoffTabLabel(null);
        setActiveHandoffRunId(output.run_id);
        setHandoffModalOpen(true);
      }
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to build handoff packet"
      );
    }
  }

  // Build or Copy Prompt for the WHOLE TAB (all completed images & assets in the tab across any runs)
  async function buildWholeTabHandoff(
    tab: CanvasTabGroup
  ): Promise<{ handoff: HandoffResponse; runId: string; outputIds: string[] }> {
    const completedItems = tab.items.filter((o) => o.status === "completed");
    if (completedItems.length === 0) {
      throw new Error("No completed images in this tab yet.");
    }
    const outputIds = completedItems.map((o) => o.id);
    const latestItem = completedItems[completedItems.length - 1];

    const tabCacheKey = `tab:${tab.tabKey}:${outputIds.join(",")}`;
    if (handoffCacheRef.current[tabCacheKey]) {
      return {
        handoff: handoffCacheRef.current[tabCacheKey],
        runId: latestItem.run_id,
        outputIds,
      };
    }

    const res = await apiFetch<{ handoff: HandoffResponse }>(
      `/api/runs/${latestItem.run_id}/handoff`,
      {
        method: "POST",
        body: JSON.stringify({
          output_ids: outputIds,
        }),
      }
    );
    handoffCacheRef.current[tabCacheKey] = res.handoff;
    return {
      handoff: res.handoff,
      runId: latestItem.run_id,
      outputIds,
    };
  }

  async function handleDeleteOutput(output: OutputDto, tab: CanvasTabGroup) {
    if (deletingOutputId) return;
    setDeletingOutputId(output.id);
    try {
      const res = await apiFetch<{
        deleted_output_id: string;
        tab_key: string;
        remaining_in_tab: number;
        tab_closed: boolean;
      }>(`/api/outputs/${output.id}`, {
        method: "DELETE",
      });

      // Clear cached handoff prompts so the tab prompt immediately updates without the deleted image
      handoffCacheRef.current = {};

      if (viewerOutput?.id === output.id) {
        setViewerOutput(null);
      }
      if (highlightTargetOutputId === output.id) {
        setHighlightTargetOutputId(null);
        setDraftHighlightBox(null);
      }

      const willCloseTab = res.tab_closed || tab.items.length <= 1;
      if (willCloseTab) {
        const remainingTabs = canvasTabs.filter(
          (t) => t.tabKey !== tab.tabKey
        );
        if (activeCanvasTabKey === tab.tabKey) {
          setActiveCanvasTabKey(
            remainingTabs[0]?.tabKey || "__design_md__"
          );
        }
        showToast(
          `Deleted "${output.title}" — tab "${tab.tabLabel}" closed (no assets remaining).`
        );
      } else {
        showToast(
          `Deleted "${output.title}" — "${tab.tabLabel}" tab prompt updated!`
        );
      }

      await refetchThreadDetail();
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to delete output"
      );
    } finally {
      setDeletingOutputId(null);
    }
  }

  async function handleCopyWholeTab(tab: CanvasTabGroup) {
    try {
      const { handoff, runId, outputIds } = await buildWholeTabHandoff(tab);
      try {
        await navigator.clipboard.writeText(handoff.clipboard_text);
        setCopiedId(`tab:${tab.tabKey}`);
        setTimeout(() => setCopiedId(null), 2400);
        showToast(
          `Copied whole-tab prompt for "${tab.tabLabel}" (${outputIds.length} images & assets)!`
        );
      } catch {
        setActiveHandoff(handoff);
        setActiveHandoffOutputId(null);
        setActiveHandoffOutputIds(outputIds);
        setActiveHandoffTabLabel(tab.tabLabel);
        setActiveHandoffRunId(runId);
        setHandoffModalOpen(true);
      }
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to copy whole-tab prompt"
      );
    }
  }

  async function handleInspectTabHandoff(tab: CanvasTabGroup) {
    try {
      const { handoff, runId, outputIds } = await buildWholeTabHandoff(tab);
      setActiveHandoff(handoff);
      setActiveHandoffOutputId(null);
      setActiveHandoffOutputIds(outputIds);
      setActiveHandoffTabLabel(tab.tabLabel);
      setActiveHandoffRunId(runId);
      setHandoffModalOpen(true);
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to open tab handoff"
      );
    }
  }

  async function handleExportTabBundle(tab: CanvasTabGroup) {
    const completedItems = tab.items.filter((o) => o.status === "completed");
    if (completedItems.length === 0) return;
    const latestRunId = completedItems[completedItems.length - 1].run_id;
    const allTabOutputIds = completedItems.map((o) => o.id);

    try {
      const res = await apiFetch<{
        handoff: HandoffResponse;
        download_url: string;
      }>(`/api/runs/${latestRunId}/export-bundle`, {
        method: "POST",
        body: JSON.stringify({
          output_ids: allTabOutputIds,
        }),
      });
      setActiveHandoff(res.handoff);
      setActiveHandoffOutputId(null);
      setActiveHandoffOutputIds(allTabOutputIds);
      setActiveHandoffTabLabel(tab.tabLabel);
      setActiveHandoffRunId(latestRunId);
      setHandoffModalOpen(true);
      window.location.href = res.download_url;
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to export portable bundle"
      );
    }
  }

  async function handlePromoteOutputToReference(output: OutputDto) {
    if (isChatLocked) return;
    try {
      const res = await apiFetch<{ asset: AssetDto }>(
        `/api/outputs/${output.id}/promote-reference`,
        {
          method: "POST",
          body: JSON.stringify({}),
        }
      );
      await refetchAssets();

      setSelectedRefs((prev) => {
        const filtered = prev.filter((r) => r.asset_id !== res.asset.id);
        return [
          ...filtered,
          {
            asset_id: res.asset.id,
            role: "selected_previous_output",
            label: res.asset.label,
            placement_notes: `Maintain visual continuity with ${output.title}`,
            preserve_as_is: false,
            source_output_id: output.id,
          },
        ];
      });
      chatInputRef.current?.focus();
      showToast(`Attached "${output.title}" to chat references`);
    } catch (err: unknown) {
      showToast(
        err instanceof Error
          ? err.message
          : "Failed to attach output as reference"
      );
    }
  }

  async function handleCheckConsistency(runId: string) {
    setCheckingConsistency(true);
    try {
      await apiFetch(`/api/runs/${runId}/check-consistency`, {
        method: "POST",
        body: JSON.stringify({}),
      });
      await refetchThreadDetail();
      showToast("Consistency check completed");
    } catch (err: unknown) {
      showToast(
        err instanceof Error ? err.message : "Failed to run consistency check"
      );
    } finally {
      setCheckingConsistency(false);
    }
  }

  async function handleAdoptRunTokens(runId: string) {
    if (!activeProjectId) return;
    setAdoptingTokens(true);
    try {
      await apiFetch(`/api/projects/${activeProjectId}/adopt-run-tokens`, {
        method: "POST",
        body: JSON.stringify({ run_id: runId }),
      });
      await refetchProjectDetail();
      queryClient.invalidateQueries({ queryKey: ["projects"] });
      showToast("Adopted run design tokens into a new Brand Kit version!");
    } catch (err: unknown) {
      showToast(
        err instanceof Error
          ? err.message
          : "Failed to adopt tokens into Brand Kit"
      );
    } finally {
      setAdoptingTokens(false);
    }
  }

  const currentBrand = projectDetail?.brand;
  const projectName = projectDetail?.project?.name || "Loading...";
  const threadTitle = threadDetail?.thread?.title || "Design exploration";

  // Outputs for Lightbox filmstrip: show all completed outputs in the current tab (or thread)
  const viewerFilmstripOutputs = useMemo(() => {
    if (currentTabGroup && currentTabGroup.items.length > 0) {
      return currentTabGroup.items.filter((o) => o.status === "completed");
    }
    return runs.flatMap((r) =>
      r.outputs.filter((o) => o.status === "completed")
    );
  }, [currentTabGroup, runs]);

  const hasCustomCardPositions = useMemo(() => {
    if (!currentTabGroup) return false;
    return (
      boardPan.x !== 0 ||
      boardPan.y !== 0 ||
      currentTabGroup.items.some((it) => {
        const off = cardOffsets[it.id];
        return off && (off.x !== 0 || off.y !== 0);
      })
    );
  }, [currentTabGroup, cardOffsets, boardPan]);

  function resetTabCardPositions() {
    if (!currentTabGroup) return;
    setCardOffsets((prev) => {
      const next = { ...prev };
      for (const it of currentTabGroup.items) {
        delete next[it.id];
      }
      return next;
    });
    setBoardScale(1);
    setBoardPan({ x: 0, y: 0 });
    showToast("Reset canvas zoom, pan, and card positions");
  }

  return (
    <div className="flex h-screen w-screen flex-col overflow-hidden bg-stone-canvas text-stone-ink select-none sm:select-auto">
      {/* =================================================================== */}
      {/* TOP BAR (`#242321` Warm Charcoal)                                   */}
      {/* =================================================================== */}
      <header className="flex h-11 shrink-0 items-center justify-between border-b border-charcoal-border bg-charcoal px-3 text-charcoal-text z-30">
        <div className="flex items-center gap-2.5 min-w-0">
          <button
            type="button"
            onClick={() => setMobileSidebarOpen((v) => !v)}
            className="md:hidden rounded p-1.5 text-charcoal-muted hover:bg-charcoal-surface hover:text-charcoal-text"
            aria-label="Toggle mobile navigation"
          >
            <Menu className="h-4 w-4" />
          </button>

          <button
            type="button"
            onClick={() => setSidebarCollapsed((v) => !v)}
            data-testid="sidebar-collapse-toggle"
            className="hidden md:inline-flex items-center justify-center rounded-md p-1.5 text-charcoal-muted hover:bg-charcoal-surface hover:text-white transition-colors"
            title={
              sidebarCollapsed
                ? "Expand sidebar (Show labels & threads)"
                : "Collapse sidebar to icon rail"
            }
          >
            {sidebarCollapsed ? (
              <PanelLeftOpen className="h-4 w-4 text-burnt" />
            ) : (
              <PanelLeftClose className="h-4 w-4" />
            )}
          </button>

          <button
            type="button"
            onClick={() => router.push("/projects")}
            className="flex items-center gap-2 focus:outline-none group"
          >
            <img
              src="/dm-logo.png"
              alt="Design Master DM Logo"
              className="h-7 w-7 rounded-md border border-charcoal-border object-cover shadow-sm"
            />
            <span className="font-display text-sm font-bold tracking-tight text-charcoal-text group-hover:text-white">
              Design Master
            </span>
          </button>

          <div className="hidden sm:flex items-center gap-1.5 text-charcoal-muted pl-2 border-l border-charcoal-border text-xs truncate">
            <button
              type="button"
              onClick={() => router.push("/projects")}
              className="rounded p-1 hover:bg-charcoal-surface hover:text-charcoal-text"
              title="Back to All Projects"
            >
              <ArrowLeft className="h-3.5 w-3.5" />
            </button>
            <ChevronRight className="h-3.5 w-3.5 opacity-50" />
            <button
              type="button"
              onClick={() => setActiveView("generations")}
              className="hover:text-charcoal-text truncate max-w-[160px]"
            >
              {projectName}
            </button>
            <span className="opacity-40">/</span>
            <span className="font-medium text-charcoal-text truncate max-w-[220px]">
              {activeView === "brand_kit"
                ? "Brand Kit & Design Style"
                : activeView === "assets"
                ? "Assets Library"
                : threadTitle}
            </span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {currentBrand?.design_style?.style_name && (
            <button
              type="button"
              onClick={() =>
                setActiveView(
                  activeView === "brand_kit" ? "generations" : "brand_kit"
                )
              }
              className="hidden md:inline-flex items-center gap-1.5 rounded-md border border-charcoal-border bg-charcoal-surface px-2.5 py-1 text-[11px] font-medium text-charcoal-text hover:bg-charcoal-hover"
              title="Active Web Design Style — click to inspect or enhance with AI"
            >
              <Palette className="h-3 w-3 text-burnt" />
              <span>
                #{currentBrand.design_style.style_number}{" "}
                {currentBrand.design_style.style_name}
              </span>
            </button>
          )}

          {(projectDetail?.project as { is_example?: boolean } | undefined)
            ?.is_example && (
            <span
              className="hidden sm:inline-flex items-center gap-1.5 rounded-full bg-burnt/20 border border-burnt/40 px-2.5 py-0.5 text-[11px] font-medium text-purple-200"
              title="Built-in Showcase Example Project — cannot be deleted, but you can freely inspect, modify, and continue generating inside it!"
            >
              <Sparkles className="h-3 w-3 text-burnt" />
              Showcase Example
            </span>
          )}

          <button
            type="button"
            onClick={() => router.push("/settings")}
            className="hidden sm:inline-flex items-center gap-1.5 rounded-md border border-charcoal-border bg-charcoal-surface px-2.5 py-1 text-[11px] font-medium text-charcoal-text hover:bg-charcoal-hover"
            title="Open Settings, Quality Pricing & Spending Graph"
          >
            <Settings className="h-3 w-3 text-burnt" />
            <span>Settings & Spend</span>
          </button>

          <span
            className="h-2 w-2 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.6)]"
            title="Local Rust daemon connected"
          />
        </div>
      </header>

      {/* =================================================================== */}
      {/* MAIN BODY: [COLLAPSIBLE SIDEBAR] + [1/4 CHAT] + [3/4 TAB CANVAS]    */}
      {/* =================================================================== */}
      <div className="relative flex flex-1 min-h-0 overflow-hidden">
        {mobileSidebarOpen && (
          <div
            className="fixed inset-0 z-20 bg-black/50 md:hidden"
            onClick={() => setMobileSidebarOpen(false)}
          />
        )}

        {/* ================================================================= */}
        {/* LEFT SIDEBAR: EXPANDED (`w-60`) OR COLLAPSED ICON RAIL (`w-14`)   */}
        {/* ================================================================= */}
        <aside
          data-testid="app-sidebar"
          data-collapsed={sidebarCollapsed ? "true" : "false"}
          className={`${
            mobileSidebarOpen ? "translate-x-0" : "-translate-x-full"
          } fixed inset-y-0 left-0 z-30 flex shrink-0 flex-col border-r border-charcoal-border bg-charcoal text-charcoal-text transition-all duration-200 md:static md:translate-x-0 ${
            sidebarCollapsed ? "md:w-14" : "w-60"
          }`}
        >
          {sidebarCollapsed ? (
            /* COLLAPSED ICON-ONLY RAIL */
            <div className="flex flex-1 flex-col items-center justify-between py-3">
              <div className="flex flex-col items-center gap-2.5 w-full px-1.5">
                <button
                  type="button"
                  onClick={() => setSidebarCollapsed(false)}
                  className="flex h-9 w-9 items-center justify-center rounded-lg text-charcoal-muted hover:bg-charcoal-surface hover:text-white transition-colors"
                  title="Expand Sidebar"
                >
                  <PanelLeftOpen className="h-4 w-4" />
                </button>

                <button
                  type="button"
                  onClick={() => router.push("/projects")}
                  className="flex h-9 w-9 items-center justify-center rounded-lg text-xs font-bold text-white shadow-sm"
                  style={{
                    backgroundColor:
                      currentBrand?.colors?.light?.primary || "#D56A3A",
                  }}
                  title={`Project: ${projectName} (Click to switch projects)`}
                >
                  {currentBrand?.logo?.monogram_text ||
                    projectName.slice(0, 1).toUpperCase()}
                </button>

                <div className="h-px w-7 bg-charcoal-border my-0.5" />

                <button
                  type="button"
                  onClick={() => setActiveView("generations")}
                  className={`flex h-9 w-9 items-center justify-center rounded-lg transition-colors ${
                    activeView === "generations"
                      ? "bg-burnt text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface hover:text-white"
                  }`}
                  title="Thread Chat & Multi-Tab Canvas"
                >
                  <MessageSquare className="h-4 w-4" />
                </button>

                <button
                  type="button"
                  onClick={handleCreateThread}
                  className="flex h-9 w-9 items-center justify-center rounded-lg text-charcoal-muted hover:bg-charcoal-surface hover:text-white transition-colors"
                  title="New Design Thread"
                >
                  <Plus className="h-4 w-4" />
                </button>

                <button
                  type="button"
                  onClick={() => setActiveView("design_md")}
                  className={`flex h-9 w-9 items-center justify-center rounded-lg transition-colors ${
                    activeView === "design_md"
                      ? "bg-burnt text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface hover:text-white"
                  }`}
                  title="DESIGN.md Specification & Export"
                >
                  <FileCode2 className="h-4 w-4" />
                </button>

                <button
                  type="button"
                  onClick={() => setActiveView("brand_kit")}
                  className={`flex h-9 w-9 items-center justify-center rounded-lg transition-colors ${
                    activeView === "brand_kit"
                      ? "bg-burnt text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface hover:text-white"
                  }`}
                  title="Brand Kit & Style Studio"
                >
                  <Palette className="h-4 w-4" />
                </button>

                <button
                  type="button"
                  onClick={() => setActiveView("assets")}
                  className={`relative flex h-9 w-9 items-center justify-center rounded-lg transition-colors ${
                    activeView === "assets"
                      ? "bg-burnt text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface hover:text-white"
                  }`}
                  title={`Project Assets Library (${projectAssets.length})`}
                >
                  <ImageIcon className="h-4 w-4" />
                  {projectAssets.length > 0 && (
                    <span className="absolute -right-0.5 -top-0.5 flex h-4 min-w-[16px] items-center justify-center rounded-full bg-burnt px-1 font-mono text-[9px] font-bold text-white">
                      {projectAssets.length}
                    </span>
                  )}
                </button>
              </div>

              <div className="flex flex-col items-center gap-2 w-full px-1.5">
                <button
                  type="button"
                  onClick={() => router.push("/settings")}
                  className="flex h-9 w-9 items-center justify-center rounded-lg text-charcoal-muted hover:bg-charcoal-surface hover:text-white transition-colors"
                  title="Settings & API Key"
                >
                  <Settings className="h-4 w-4" />
                </button>
              </div>
            </div>
          ) : (
            /* EXPANDED SIDEBAR (`w-60`) */
            <>
              <div className="relative p-2.5 border-b border-charcoal-border">
                <button
                  type="button"
                  onClick={() => setProjectDropdownOpen((v) => !v)}
                  className="flex w-full items-center justify-between rounded-lg bg-charcoal-surface px-2.5 py-2 text-left hover:bg-charcoal-hover transition-colors border border-charcoal-border"
                >
                  <div className="flex items-center gap-2.5 min-w-0">
                    <div
                      className="flex h-7 w-7 shrink-0 items-center justify-center rounded-md text-xs font-bold text-white shadow-sm"
                      style={{
                        backgroundColor:
                          currentBrand?.colors?.light?.primary || "#D56A3A",
                      }}
                    >
                      {currentBrand?.logo?.monogram_text ||
                        projectName.slice(0, 1).toUpperCase()}
                    </div>
                    <div className="min-w-0">
                      <div className="truncate text-xs font-semibold text-charcoal-text">
                        {projectName}
                      </div>
                      <div className="truncate text-[10px] text-charcoal-muted">
                        Brand v{projectDetail?.brand_version || 1}
                      </div>
                    </div>
                  </div>
                  <ChevronDown className="h-3.5 w-3.5 shrink-0 text-charcoal-muted" />
                </button>

                {projectDropdownOpen && (
                  <div className="absolute left-2.5 right-2.5 top-14 z-40 rounded-lg border border-charcoal-border bg-charcoal-surface p-1.5 shadow-modal">
                    <div className="px-2 py-1 text-[10px] font-semibold uppercase tracking-wider text-charcoal-muted">
                      Switch Project
                    </div>
                    <div className="max-h-48 overflow-y-auto space-y-0.5">
                      {projectsData?.projects?.map((p) => (
                        <button
                          key={p.id}
                          type="button"
                          onClick={() => {
                            setProjectDropdownOpen(false);
                            setActiveCanvasTabKey(null);
                            router.push(
                              `/workspace?projectId=${p.id}${
                                p.default_thread_id
                                  ? `&threadId=${p.default_thread_id}`
                                  : ""
                              }`
                            );
                          }}
                          className={`flex w-full items-center justify-between rounded-md px-2 py-1.5 text-xs ${
                            p.id === activeProjectId
                              ? "bg-charcoal-hover font-semibold text-white"
                              : "text-charcoal-muted hover:bg-charcoal-hover hover:text-charcoal-text"
                          }`}
                        >
                          <span className="truncate">{p.name}</span>
                          {p.id === activeProjectId && (
                            <Check className="h-3.5 w-3.5 text-burnt" />
                          )}
                        </button>
                      ))}
                    </div>
                    <div className="mt-1 border-t border-charcoal-border pt-1">
                      <button
                        type="button"
                        onClick={() => {
                          setProjectDropdownOpen(false);
                          router.push("/projects");
                        }}
                        className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-xs font-medium text-burnt hover:bg-charcoal-hover"
                      >
                        <FolderKanban className="h-3.5 w-3.5" />
                        <span>All projects / + New project</span>
                      </button>
                    </div>
                  </div>
                )}
              </div>

              {/* Primary Navigation Views */}
              <div className="px-2.5 py-2 space-y-0.5">
                <button
                  type="button"
                  onClick={() => {
                    setActiveView("generations");
                    setMobileSidebarOpen(false);
                  }}
                  className={`flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs font-medium transition-colors ${
                    activeView === "generations"
                      ? "bg-charcoal-surface text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface/60 hover:text-charcoal-text"
                  }`}
                >
                  <MessageSquare
                    className={`h-3.5 w-3.5 ${
                      activeView === "generations" ? "text-burnt" : "opacity-75"
                    }`}
                  />
                  <span>Chat & Multi-Tab Canvas</span>
                </button>

                <button
                  type="button"
                  data-testid="sidebar-nav-design-md"
                  onClick={() => {
                    setActiveView("design_md");
                    setMobileSidebarOpen(false);
                  }}
                  className={`flex w-full items-center justify-between rounded-md px-2.5 py-1.5 text-xs font-medium transition-colors ${
                    activeView === "design_md"
                      ? "bg-charcoal-surface text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface/60 hover:text-charcoal-text"
                  }`}
                >
                  <span className="flex items-center gap-2.5">
                    <FileCode2
                      className={`h-3.5 w-3.5 ${
                        activeView === "design_md" ? "text-burnt" : "opacity-75"
                      }`}
                    />
                    <span>DESIGN.md Spec</span>
                  </span>
                  <span className="rounded bg-burnt/20 border border-burnt/40 px-1.5 py-0.2 font-mono text-[9px] font-bold text-amber-200">
                    .md
                  </span>
                </button>

                <button
                  type="button"
                  onClick={() => {
                    setActiveView("brand_kit");
                    setMobileSidebarOpen(false);
                  }}
                  className={`flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs font-medium transition-colors ${
                    activeView === "brand_kit"
                      ? "bg-charcoal-surface text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface/60 hover:text-charcoal-text"
                  }`}
                >
                  <Palette
                    className={`h-3.5 w-3.5 ${
                      activeView === "brand_kit" ? "text-burnt" : "opacity-75"
                    }`}
                  />
                  <span>Brand Kit & Style</span>
                </button>

                <button
                  type="button"
                  onClick={() => {
                    setActiveView("assets");
                    setMobileSidebarOpen(false);
                  }}
                  className={`flex w-full items-center justify-between rounded-md px-2.5 py-1.5 text-xs font-medium transition-colors ${
                    activeView === "assets"
                      ? "bg-charcoal-surface text-white"
                      : "text-charcoal-muted hover:bg-charcoal-surface/60 hover:text-charcoal-text"
                  }`}
                >
                  <span className="flex items-center gap-2.5">
                    <ImageIcon
                      className={`h-3.5 w-3.5 ${
                        activeView === "assets" ? "text-burnt" : "opacity-75"
                      }`}
                    />
                    <span>Assets Library</span>
                  </span>
                  <span className="rounded bg-charcoal px-1.5 py-0.2 font-mono text-[10px] text-charcoal-muted">
                    {projectAssets.length}
                  </span>
                </button>
              </div>

              {/* Threads List */}
              <div className="mt-1 flex flex-1 flex-col min-h-0 px-2.5">
                <div className="flex items-center justify-between px-1.5 pb-1.5">
                  <span className="text-[10px] font-bold uppercase tracking-wider text-charcoal-muted">
                    Threads
                  </span>
                  <button
                    type="button"
                    onClick={() => setShowArchivedThreads((v) => !v)}
                    className="text-[10px] text-charcoal-muted hover:text-charcoal-text"
                  >
                    {showArchivedThreads ? "Hide archived" : "Show archived"}
                  </button>
                </div>

                <div className="relative mb-2">
                  <Search className="pointer-events-none absolute left-2 top-1/2 h-3 w-3 -translate-y-1/2 text-charcoal-muted" />
                  <input
                    type="text"
                    value={threadSearch}
                    onChange={(e) => setThreadSearch(e.target.value)}
                    placeholder="Search threads..."
                    className="w-full rounded-md border border-charcoal-border bg-charcoal-surface py-1 pl-6 pr-2 text-[11px] text-charcoal-text placeholder:text-charcoal-muted focus:border-burnt focus:outline-none"
                  />
                </div>

                <div className="flex-1 overflow-y-auto space-y-0.5 pr-0.5">
                  {threadsData?.threads?.map((t) => {
                    const isSelected =
                      t.id === activeThreadId && activeView === "generations";
                    const isEditing = editingThreadId === t.id;

                    return (
                      <div
                        key={t.id}
                        className={`group flex items-center justify-between rounded-md px-2 py-1.5 text-xs transition-colors ${
                          isSelected
                            ? "bg-charcoal-surface font-medium text-white"
                            : "text-charcoal-muted hover:bg-charcoal-surface/50 hover:text-charcoal-text"
                        }`}
                      >
                        {isEditing ? (
                          <input
                            type="text"
                            value={editingThreadTitle}
                            onChange={(e) =>
                              setEditingThreadTitle(e.target.value)
                            }
                            onBlur={() => handleSaveThreadRename(t.id)}
                            onKeyDown={(e) => {
                              if (e.key === "Enter")
                                handleSaveThreadRename(t.id);
                              if (e.key === "Escape") setEditingThreadId(null);
                            }}
                            autoFocus
                            className="w-full rounded bg-charcoal px-1.5 py-0.5 text-xs text-white focus:outline-none border border-burnt"
                          />
                        ) : (
                          <>
                            <button
                              type="button"
                              onClick={() => {
                                setActiveView("generations");
                                setActiveCanvasTabKey(null);
                                setMobileSidebarOpen(false);
                                router.push(
                                  `/workspace?projectId=${activeProjectId}&threadId=${t.id}`
                                );
                              }}
                              className="flex flex-1 items-center gap-2 min-w-0 text-left"
                            >
                              <span
                                className={`h-1.5 w-1.5 shrink-0 rounded-full ${
                                  isSelected
                                    ? "bg-burnt"
                                    : "bg-charcoal-muted/40"
                                }`}
                              />
                              <span className="truncate">{t.title}</span>
                            </button>

                            <div className="hidden group-hover:flex items-center gap-1 pl-1">
                              <button
                                type="button"
                                onClick={() => {
                                  setEditingThreadId(t.id);
                                  setEditingThreadTitle(t.title);
                                }}
                                className="rounded p-0.5 text-charcoal-muted hover:text-white"
                                title="Rename thread"
                              >
                                <Edit2 className="h-3 w-3" />
                              </button>
                              <button
                                type="button"
                                onClick={() => handleToggleThreadArchive(t)}
                                className="rounded p-0.5 text-charcoal-muted hover:text-white"
                                title={
                                  t.archive_status === "archived"
                                    ? "Reopen thread"
                                    : "Archive thread"
                                }
                              >
                                {t.archive_status === "archived" ? (
                                  <RotateCcw className="h-3 w-3" />
                                ) : (
                                  <Archive className="h-3 w-3" />
                                )}
                              </button>
                            </div>
                          </>
                        )}
                      </div>
                    );
                  })}

                  <button
                    type="button"
                    onClick={handleCreateThread}
                    className="mt-1 flex w-full items-center gap-1.5 rounded-md px-2 py-1.5 text-xs text-charcoal-muted hover:bg-charcoal-surface/50 hover:text-charcoal-text"
                  >
                    <Plus className="h-3.5 w-3.5" />
                    <span>New thread</span>
                  </button>
                </div>
              </div>

              {/* Sidebar Footer */}
              <div className="border-t border-charcoal-border p-2 flex items-center justify-between text-xs text-charcoal-muted">
                <button
                  type="button"
                  onClick={() => router.push("/settings")}
                  className="flex items-center gap-2 rounded-md px-2 py-1.5 hover:bg-charcoal-surface hover:text-charcoal-text transition-colors flex-1"
                >
                  <Settings className="h-3.5 w-3.5" />
                  <span>Settings</span>
                </button>
                <button
                  type="button"
                  onClick={() => setSidebarCollapsed(true)}
                  className="hidden md:flex items-center justify-center rounded-md p-1.5 hover:bg-charcoal-surface hover:text-white"
                  title="Collapse sidebar to icon rail"
                >
                  <PanelLeftClose className="h-3.5 w-3.5" />
                </button>
              </div>
            </>
          )}
        </aside>

        {/* ================================================================= */}
        {/* MAIN CONTENT AREA                                                 */}
        {/* ================================================================= */}
        {activeView === "design_md" && projectDetail ? (
          <main className="flex-1 overflow-y-auto p-6 bg-[#090B11]">
            <DesignMdView
              project={projectDetail.project}
              brand={projectDetail.brand}
              brandVersion={projectDetail.brand_version}
              onOpenBrandEditor={() => setActiveView("brand_kit")}
            />
          </main>
        ) : activeView === "brand_kit" && projectDetail ? (
          <main className="flex-1 overflow-y-auto p-6">
            <BrandKitView
              projectId={projectDetail.project.id}
              projectMeta={projectDetail.project}
              brandVersion={projectDetail.brand_version}
              initialBrand={projectDetail.brand}
              brandHistory={projectDetail.brand_history}
              assets={projectAssets}
              onSaved={() => {
                refetchProjectDetail();
                refetchAssets();
                queryClient.invalidateQueries({ queryKey: ["projects"] });
                showToast(
                  "Saved new Brand Kit version! DESIGN.md & future generations are updated."
                );
              }}
            />
          </main>
        ) : activeView === "assets" && activeProjectId ? (
          <main className="flex-1 overflow-y-auto p-6">
            <AssetsLibraryView
              projectId={activeProjectId}
              assets={projectAssets}
              onRefresh={() => {
                refetchAssets();
                refetchProjectDetail();
              }}
              onAttachToComposer={(asset) => handleAttachExistingAsset(asset)}
            />
          </main>
        ) : (
          /* =============================================================== */
          /* THREAD WORKSPACE: [1/4 SIDEWAYS CHAT] + [3/4 MULTI-TAB CANVAS]  */
          /* =============================================================== */
          <div className="flex flex-1 flex-col lg:flex-row min-w-0 min-h-0 overflow-hidden">
            {/* ============================================================= */}
            {/* 1/4 SCREEN SIDEWAYS CONVERSATIONAL CHAT PANEL                 */}
            {/* ============================================================= */}
            <section
              data-testid="sideways-chat-panel"
              className="flex w-full lg:w-[27%] lg:min-w-[335px] lg:max-w-[420px] shrink-0 flex-col border-b lg:border-b-0 lg:border-r border-stone-border bg-stone-surface min-h-0 max-h-[45vh] lg:max-h-none"
            >
              {/* Chat Panel Header */}
              <div className="flex items-center justify-between border-b border-[#252D47] px-3.5 py-2.5 bg-[#111522]">
                <div className="min-w-0">
                  <div className="flex items-center gap-1.5">
                    <Sparkles className="h-3.5 w-3.5 text-burnt shrink-0" />
                    <h2 className="truncate text-xs font-bold text-white">
                      Design Thread Chat
                    </h2>
                  </div>
                  <p className="truncate text-[10px] text-stone-300">
                    GLM 5.3 Flash + Sunburst 2.5 · Paste multiple images (`⌘V`)
                  </p>
                </div>

                {/* Custom Quality Pill Switcher */}
                <div className="flex items-center gap-0.5 rounded-lg border border-[#28304A] bg-[#0B0E17] p-0.5 shrink-0">
                  {(["low", "medium", "high"] as const).map((q) => (
                    <button
                      key={q}
                      type="button"
                      disabled={isChatLocked}
                      onClick={() => setQuality(q)}
                      className={`rounded-md px-2 py-0.5 text-[10px] font-bold capitalize transition-colors ${
                        quality === q
                          ? "bg-burnt text-white shadow-sm"
                          : "text-stone-300 hover:text-white"
                      }`}
                      title={`Default render quality: ${q}`}
                    >
                      {q === "medium" ? "Med" : q}
                    </button>
                  ))}
                </div>
              </div>

              {/* Scrollable Chat Message Feed */}
              <div
                ref={chatScrollRef}
                className="flex-1 overflow-y-auto p-3.5 space-y-3"
              >
                {messages.length === 0 ? (
                  <div className="rounded-xl border border-dashed border-[#28304A] bg-[#111522] p-3.5 text-xs space-y-2.5">
                    <div className="font-bold text-white">
                      Chat with your Design Architect
                    </div>
                    <p className="text-[11px] text-stone-300 leading-relaxed">
                      Describe what you want to build for{" "}
                      <span className="font-semibold text-white">
                        {projectName}
                      </span>
                      . The AI will propose{" "}
                      <span className="font-semibold text-amber-200">
                        3–4 project-tailored questions & selectable options
                      </span>{" "}
                      (or you can chat freely) and start generating as soon as
                      everything is clear!
                    </p>
                    <div className="flex flex-wrap gap-1.5 pt-1">
                      {(projectDetail?.project?.industry
                        ?.toLowerCase()
                        .includes("game") ||
                      projectName.toLowerCase().includes("aetheris")
                        ? [
                            `Design the main PC/Console 16:9 Game HUD & Inventory Panel for ${projectName} with a rotating 3D emblem`,
                            `Create a transparent vectorial SVG UI frame & 6-icon skill sheet for ${projectName}`,
                            `Design the Character Loadout & Telemetry screen for ${projectName}`,
                          ]
                        : [
                            `Create the whole landing page for ${projectName} matching its DESIGN.md and anti-AI-slop rules`,
                            `Design a sticky desktop Navbar + Hero section with an interactive product showcase and 3D accent`,
                            `Generate a transparent PNG icon sheet with 6 vectorial SVG product icons for ${projectName}`,
                          ]
                      ).map((starter) => (
                        <button
                          key={starter}
                          type="button"
                          disabled={isChatLocked}
                          onClick={() => setChatInput(starter)}
                          className="rounded-lg border border-[#2A324D] bg-[#171C2E] px-2.5 py-1.5 text-left text-[10px] font-medium text-stone-100 hover:border-burnt hover:text-white disabled:opacity-50 transition-colors"
                        >
                          {starter}
                        </button>
                      ))}
                    </div>
                  </div>
                ) : (
                  messages.map((msg) => {
                    const isUser = msg.role === "user";
                    const selectedForMsg = selectedQuickReplies[msg.id] || [];
                    return (
                      <div
                        key={msg.id}
                        className={`flex flex-col ${
                          isUser ? "items-end" : "items-start"
                        }`}
                      >
                        <div
                          className={`max-w-[92%] rounded-2xl px-3.5 py-2.5 text-xs leading-relaxed shadow-sm ${
                            isUser
                              ? "bg-[#1F1A3A] border border-burnt/50 text-white rounded-br-sm"
                              : "bg-[#151929] border border-[#28304A] text-stone-50 rounded-bl-sm"
                          }`}
                        >
                          {/* Highlight Annotations Badge inside message */}
                          {msg.highlight_annotations &&
                            msg.highlight_annotations.length > 0 && (
                              <div className="mb-2 rounded-lg bg-burnt/20 border border-burnt/40 px-2.5 py-1.5 text-[10px]">
                                {msg.highlight_annotations.map((hl, idx) => (
                                  <div
                                    key={idx}
                                    className="flex items-center gap-1.5 font-semibold text-amber-200"
                                  >
                                    <Crosshair className="h-3 w-3 shrink-0" />
                                    <span>
                                      Region ({Math.round(hl.x_pct)}%,{" "}
                                      {Math.round(hl.y_pct)}% ·{" "}
                                      {Math.round(hl.w_pct)}×
                                      {Math.round(hl.h_pct)}%) · Sunburst Medium
                                      Edit
                                    </span>
                                  </div>
                                ))}
                              </div>
                            )}

                          {/* Attached Reference Images inside message */}
                          {msg.attachments && msg.attachments.length > 0 && (
                            <div className="mb-2 flex flex-wrap gap-1.5">
                              {msg.attachments.map((att) => (
                                <div
                                  key={att.asset_id}
                                  className="relative overflow-hidden rounded-lg border border-white/20 bg-black/20 p-1 flex items-center gap-1.5"
                                >
                                  <img
                                    src={att.thumbnail_url}
                                    alt={att.label}
                                    className="h-8 w-10 rounded object-cover"
                                  />
                                  <div className="pr-1 text-[10px]">
                                    <div className="truncate max-w-[90px] font-semibold">
                                      {att.label}
                                    </div>
                                    <div className="opacity-75 text-[9px]">
                                      {att.role.replace("_", " ")}
                                    </div>
                                  </div>
                                </div>
                              ))}
                            </div>
                          )}

                          <div className="whitespace-pre-line">
                            {msg.content}
                          </div>

                          {/* Linked Tab Jump Button when assistant triggers/edits a tab */}
                          {msg.target_tab_key && (
                            <div className="mt-2 pt-2 border-t border-stone-border/60 flex items-center justify-between gap-2">
                              <button
                                type="button"
                                onClick={() =>
                                  setActiveCanvasTabKey(
                                    normalizeTabKeyClient(msg.target_tab_key!)
                                  )
                                }
                                className="inline-flex items-center gap-1.5 rounded-md bg-burnt/10 border border-burnt/30 px-2.5 py-1 text-[11px] font-bold text-burnt hover:bg-burnt hover:text-white transition-colors"
                              >
                                <Layout className="h-3 w-3" />
                                <span>
                                  Open {msg.target_tab_label || msg.target_tab_key}{" "}
                                  Tab →
                                </span>
                              </button>
                            </div>
                          )}
                        </div>

                        {/* Model-Generated Multi-Select Options & Instant Generation Trigger */}
                        {!isUser &&
                          msg.quick_replies &&
                          msg.quick_replies.length > 0 && (
                            <div className="mt-2 flex flex-col gap-2 max-w-[96%]">
                              <div className="text-[10px] font-medium text-stone-300 px-1">
                                Click options below to select answers (or type freely in the chat box):
                              </div>
                              <div className="flex flex-wrap gap-1.5">
                                {msg.quick_replies.map((qr) => {
                                  const lowerQr = qr.toLowerCase();
                                  const isDirectGenerate =
                                    lowerQr.includes("all clear") ||
                                    lowerQr.includes("start generating") ||
                                    lowerQr === "generate now";
                                  const isSelected = selectedForMsg.includes(qr);

                                  return (
                                    <button
                                      key={qr}
                                      type="button"
                                      disabled={isChatLocked}
                                      onClick={() => {
                                        if (isDirectGenerate) {
                                          const combined =
                                            selectedForMsg.length > 0
                                              ? `${selectedForMsg.join(" · ")} — ${qr}`
                                              : chatInput.trim() || qr;
                                          sendChatMessage({
                                            overrideText: combined,
                                            forceGenerate: true,
                                          });
                                          return;
                                        }
                                        // Toggle this option in selectedQuickReplies and sync into chatInput
                                        const nextSelected = isSelected
                                          ? selectedForMsg.filter((x) => x !== qr)
                                          : [...selectedForMsg, qr];
                                        setSelectedQuickReplies((prev) => ({
                                          ...prev,
                                          [msg.id]: nextSelected,
                                        }));
                                        setChatInput(nextSelected.join(" · "));
                                      }}
                                      className={`inline-flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-[11px] font-semibold shadow-sm transition-all text-left ${
                                        isDirectGenerate
                                          ? "border-emerald-400/50 bg-emerald-500/20 text-emerald-100 hover:bg-emerald-500 hover:text-white"
                                          : isSelected
                                          ? "border-burnt bg-burnt text-white ring-2 ring-burnt/40"
                                          : "border-[#362F66] bg-[#1B1732] text-purple-100 hover:border-burnt hover:bg-[#262046] hover:text-white"
                                      } disabled:opacity-40`}
                                    >
                                      {isSelected && (
                                        <Check className="h-3 w-3 shrink-0 text-white" />
                                      )}
                                      <span>{qr}</span>
                                    </button>
                                  );
                                })}
                              </div>
                              {selectedForMsg.length > 0 && (
                                <div className="flex items-center gap-2 pt-0.5">
                                  <button
                                    type="button"
                                    disabled={isChatLocked}
                                    onClick={() => {
                                      sendChatMessage({
                                        overrideText:
                                          chatInput.trim() ||
                                          selectedForMsg.join(" · "),
                                        forceGenerate: true,
                                      });
                                    }}
                                    className="inline-flex items-center gap-1.5 rounded-xl bg-gradient-to-r from-[#8B5CF6] to-[#06B6D4] px-3.5 py-1.5 text-xs font-bold text-white shadow-md hover:opacity-95 disabled:opacity-40 transition-opacity"
                                  >
                                    <Sparkles className="h-3.5 w-3.5" />
                                    <span>
                                      Confirm Selected ({selectedForMsg.length}) & Start Generation →
                                    </span>
                                  </button>
                                </div>
                              )}
                            </div>
                          )}
                      </div>
                    );
                  })
                )}

                {/* LIVE AI WORKING / GENERATING INDICATOR IN CHAT */}
                {isChatLocked && (
                  <div
                    data-testid="chat-ai-working-indicator"
                    className="rounded-2xl border-2 border-burnt/40 bg-charcoal p-3.5 text-white shadow-md space-y-2"
                  >
                    <div className="flex items-center justify-between gap-2">
                      <div className="flex items-center gap-2">
                        <Loader2 className="h-4 w-4 animate-spin text-burnt shrink-0" />
                        <span className="text-xs font-bold text-white">
                          {sendingChat
                            ? "GLM 5.3 Flash is thinking..."
                            : activeGeneratingRun?.status === "planning"
                            ? "GLM 5.3 Flash is planning design architecture..."
                            : `Sunburst 2.5 is rendering images (${
                                activeGeneratingRun?.outputs.filter(
                                  (o) => o.status === "completed"
                                ).length || 0
                              }/${
                                activeGeneratingRun?.outputs.length || 1
                              } completed)...`}
                        </span>
                      </div>
                      {activeGeneratingRun && (
                        <button
                          type="button"
                          onClick={() =>
                            handleCancelRun(activeGeneratingRun.id)
                          }
                          className="rounded-md bg-white/10 px-2 py-0.5 text-[10px] font-semibold text-white/80 hover:bg-red-500/30 hover:text-white transition-colors"
                        >
                          Cancel
                        </button>
                      )}
                    </div>
                    {liveStreamMessage && (
                      <div className="rounded-lg bg-white/10 px-2.5 py-1.5 font-mono text-[10px] text-amber-200">
                        {liveStreamMessage}
                      </div>
                    )}
                    <p className="text-[11px] text-white/65 leading-relaxed">
                      Chat input is locked while the AI is actively working so
                      your generation completes cleanly.
                    </p>
                  </div>
                )}

                {/* BACKEND RUN ERROR BANNER IN CHAT WHEN A RUN STOPPED / FAILED */}
                {!isChatLocked &&
                  runs.length > 0 &&
                  ["failed", "outcome_unknown", "interrupted", "partial"].includes(
                    runs[runs.length - 1].status
                  ) && (
                    <div
                      data-testid="chat-run-error-banner"
                      className="rounded-2xl border-2 border-danger-ink/40 bg-danger-bg p-3.5 text-danger-ink shadow-sm space-y-2"
                    >
                      <div className="flex items-start justify-between gap-2">
                        <div className="flex items-center gap-1.5 font-bold text-xs">
                          <AlertTriangle className="h-4 w-4 shrink-0" />
                          <span>
                            Generation stopped (
                            {runs[runs.length - 1].status.replace("_", " ")})
                          </span>
                        </div>
                        <button
                          type="button"
                          disabled={retryingRunId === runs[runs.length - 1].id}
                          onClick={() =>
                            handleRetryRun(runs[runs.length - 1].id)
                          }
                          className="inline-flex items-center gap-1 rounded-lg bg-danger-ink px-2.5 py-1 text-[11px] font-bold text-white hover:opacity-90 disabled:opacity-50 shrink-0"
                        >
                          <RefreshCw
                            className={`h-3 w-3 ${
                              retryingRunId === runs[runs.length - 1].id
                                ? "animate-spin"
                                : ""
                            }`}
                          />
                          <span>Retry Generation</span>
                        </button>
                      </div>
                      <p className="text-[11px] leading-relaxed break-words font-mono bg-white/70 rounded-lg p-2 border border-danger-ink/20">
                        {runs[runs.length - 1].error_message ||
                          "An unexpected backend error stopped the generation before all items finished."}
                      </p>
                    </div>
                  )}
              </div>

              {/* Chat Composer Footer (Locked while generating) */}
              <div
                onDragOver={(e) => e.preventDefault()}
                onDrop={(e) => {
                  e.preventDefault();
                  if (isChatLocked) return;
                  const files = Array.from(e.dataTransfer.files || []).filter(
                    (f) => f.type.startsWith("image/")
                  );
                  if (files.length > 0) {
                    uploadAndAttachFiles(files);
                  }
                }}
                className={`border-t border-stone-border bg-[#0F1320] p-3 space-y-2.5 transition-opacity ${
                  isChatLocked ? "bg-stone-canvas/80" : ""
                }`}
              >
                {/* Hidden Multi-Image File Input */}
                <input
                  ref={fileInputRef}
                  type="file"
                  multiple
                  disabled={isChatLocked}
                  accept="image/png,image/jpeg,image/webp,image/svg+xml"
                  className="hidden"
                  onChange={(e) => {
                    const files = Array.from(e.target.files || []);
                    if (files.length > 0) {
                      uploadAndAttachFiles(files);
                    }
                    e.target.value = "";
                  }}
                />

                {/* Queued Highlight Annotations Banner */}
                {queuedHighlights.length > 0 && (
                  <div className="rounded-lg border border-burnt/40 bg-burnt/10 p-2 text-[11px] space-y-1">
                    {queuedHighlights.map((hl, i) => (
                      <div
                        key={i}
                        className="flex items-center justify-between gap-2"
                      >
                        <div className="flex items-center gap-1.5 font-semibold text-burnt min-w-0">
                          <Crosshair className="h-3.5 w-3.5 shrink-0" />
                          <span className="truncate">
                            [{hl.tab_label || hl.tab_key}] &ldquo;{hl.comment}
                            &rdquo; (Sunburst Medium)
                          </span>
                        </div>
                        <button
                          type="button"
                          disabled={isChatLocked}
                          onClick={() =>
                            setQueuedHighlights((prev) =>
                              prev.filter((_, idx) => idx !== i)
                            )
                          }
                          className="text-stone-muted hover:text-stone-ink"
                        >
                          <X className="h-3.5 w-3.5" />
                        </button>
                      </div>
                    ))}
                  </div>
                )}

                {/* Multi-Image Attachments Tray */}
                {selectedRefs.length > 0 && (
                  <div className="flex items-center gap-2 overflow-x-auto pb-1">
                    {selectedRefs.map((sr) => (
                      <div
                        key={sr.asset_id}
                        className="flex items-center gap-1.5 rounded-lg border border-stone-border bg-stone-canvas p-1 pr-2 shrink-0 text-[10px]"
                      >
                        <img
                          src={`/media/asset/${sr.asset_id}/thumbnail`}
                          alt={sr.label}
                          className="h-8 w-10 rounded object-cover bg-white border border-stone-border"
                        />
                        <div className="min-w-0">
                          <div className="truncate max-w-[85px] font-semibold text-stone-ink">
                            {sr.label}
                          </div>
                          <div className="text-[9px] font-bold text-burnt capitalize">
                            {sr.role.replace("_", " ")}
                          </div>
                        </div>
                        <button
                          type="button"
                          disabled={isChatLocked}
                          onClick={() =>
                            setSelectedRefs((prev) =>
                              prev.filter((r) => r.asset_id !== sr.asset_id)
                            )
                          }
                          className="text-stone-muted hover:text-stone-ink ml-0.5"
                        >
                          <X className="h-3 w-3" />
                        </button>
                      </div>
                    ))}
                  </div>
                )}

                {chatError && (
                  <div className="rounded-lg border border-danger-ink/30 bg-danger-bg px-2.5 py-1.5 text-[11px] text-danger-ink">
                    {chatError}
                  </div>
                )}

                {/* Chat Textarea (Locked while AI is working) */}
                <div className="relative">
                  <textarea
                    ref={chatInputRef}
                    data-testid="chat-composer-input"
                    rows={3}
                    disabled={isChatLocked}
                    readOnly={isChatLocked}
                    value={chatInput}
                    onChange={(e) => setChatInput(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter" && !e.shiftKey) {
                        e.preventDefault();
                        if (!isChatLocked) {
                          sendChatMessage();
                        }
                      }
                    }}
                    placeholder={
                      isChatLocked
                        ? "AI is working on your design — chat is locked until generation completes..."
                        : currentTabGroup
                        ? `Chat with the AI, paste screenshots (⌘V), or ask to refine "${currentTabGroup.tabLabel}"...`
                        : "Chat with the AI about what to design or paste screenshots (⌘V)..."
                    }
                    className={`w-full rounded-xl border p-3 text-xs text-white placeholder:text-stone-300 focus:outline-none resize-none transition-colors ${
                      isChatLocked
                        ? "border-stone-border bg-[#0A0C14] text-stone-400 cursor-not-allowed"
                        : "border-[#2A324D] bg-[#0B0E18] focus:border-burnt focus:bg-[#101524]"
                    }`}
                  />
                </div>

                {/* Clean, Non-Overflowing Chat Action Bar */}
                <div className="flex items-center justify-between gap-2 pt-0.5">
                  <div className="flex items-center gap-1.5 min-w-0">
                    <button
                      type="button"
                      disabled={isChatLocked}
                      onClick={() => fileInputRef.current?.click()}
                      className="inline-flex items-center gap-1.5 rounded-lg border border-[#2D3552] bg-[#161B2C] px-3 py-1.5 text-xs font-semibold text-white hover:bg-[#20273E] hover:border-burnt/60 disabled:opacity-40 disabled:cursor-not-allowed transition-colors whitespace-nowrap shrink-0"
                      title="Upload or paste multiple screenshots & reference images"
                    >
                      <Paperclip className="h-3.5 w-3.5 text-burnt shrink-0" />
                      <span>{uploadingFiles ? "Uploading..." : "Images"}</span>
                    </button>

                    <button
                      type="button"
                      disabled={isChatLocked}
                      onClick={() => setAddAssetModalOpen(true)}
                      className="inline-flex items-center gap-1.5 rounded-lg border border-[#2D3552] bg-[#161B2C] px-3 py-1.5 text-xs font-semibold text-stone-100 hover:bg-[#20273E] hover:text-white hover:border-burnt/60 disabled:opacity-40 disabled:cursor-not-allowed transition-colors whitespace-nowrap shrink-0"
                      title="Pick from Project Assets Library"
                    >
                      <Plus className="h-3.5 w-3.5 text-purple-300 shrink-0" />
                      <span>Library</span>
                    </button>
                  </div>

                  <button
                    type="button"
                    disabled={
                      isChatLocked ||
                      (!chatInput.trim() && queuedHighlights.length === 0)
                    }
                    onClick={() => sendChatMessage()}
                    data-testid="chat-send-btn"
                    className="inline-flex items-center justify-center gap-1.5 rounded-lg bg-burnt px-4 py-1.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-40 disabled:cursor-not-allowed transition-colors whitespace-nowrap shrink-0"
                  >
                    {isChatLocked ? (
                      <>
                        <Loader2 className="h-3.5 w-3.5 animate-spin shrink-0" />
                        <span>AI Working...</span>
                      </>
                    ) : (
                      <>
                        <Send className="h-3.5 w-3.5 shrink-0" />
                        <span>Send</span>
                      </>
                    )}
                  </button>
                </div>
              </div>
            </section>

            {/* ============================================================= */}
            {/* 3/4 SCREEN INTERACTIVE MULTI-TAB CANVAS BOARD                 */}
            {/* ============================================================= */}
            <main
              data-testid="multi-tab-canvas"
              className="flex flex-1 flex-col min-w-0 min-h-0 overflow-hidden bg-stone-canvas"
            >
              {/* TOP DYNAMIC COMPONENT / PAGE / ASSET TAB BAR */}
              <div className="relative flex items-center justify-between gap-2 border-b border-stone-border bg-stone-surface px-4 py-2 shrink-0 z-30">
                <div
                  ref={tabRowScrollRef}
                  className="flex items-center gap-1.5 overflow-x-auto no-scrollbar min-w-0 flex-1 pr-2"
                >
                  {/* Pinned DESIGN.md Specification Tab in every thread */}
                  <button
                    type="button"
                    data-testid="canvas-tab-design-md"
                    onClick={() => {
                      setActiveCanvasTabKey("__design_md__");
                      setDraftHighlightBox(null);
                      setHighlightTargetOutputId(null);
                      setTabDropdownOpen(false);
                    }}
                    className={`group relative flex items-center gap-2 rounded-lg px-3 py-1.5 text-xs font-bold transition-all shrink-0 ${
                      activeCanvasTabKey === "__design_md__"
                        ? "bg-burnt text-white shadow-sm border border-burnt"
                        : "bg-[#141827] text-stone-100 border border-[#28304A] hover:text-white hover:border-burnt/60 hover:bg-[#1B2135]"
                    }`}
                    title="Open DESIGN.md Design System & Anti-AI-Slop Guardrails (exportable as .md)"
                  >
                    <FileCode2 className="h-3.5 w-3.5 shrink-0 text-amber-400" />
                    <span>DESIGN.md</span>
                    <span
                      className={`rounded px-1.5 py-0.2 font-mono text-[9px] ${
                        activeCanvasTabKey === "__design_md__"
                          ? "bg-white/20 text-white"
                          : "bg-burnt/20 text-amber-300 border border-burnt/30"
                      }`}
                    >
                      .md
                    </span>
                  </button>

                  {canvasTabs.length === 0 ? (
                    <span className="text-xs font-semibold text-stone-300 py-1 pl-2">
                      Canvas Tabs (Full Website & Component tabs appear here as
                      you generate)
                    </span>
                  ) : (
                    <>
                      {canvasTabs.map((tab) => {
                        const isSelected = activeCanvasTabKey === tab.tabKey;
                        const isGenerating = Boolean(tab.activeRunForTab);
                        const hasFailedRun = Boolean(
                          tab.failedRunForTab && !tab.activeRunForTab
                        );
                        const isMobile = tab.aspectRatio === "9:16";
                        const isTablet = tab.aspectRatio === "3:4";
                        const isAsset = tab.kind === "asset";

                        return (
                          <button
                            key={tab.tabKey}
                            type="button"
                            data-testid={`canvas-tab-${tab.tabKey}`}
                            onClick={() => {
                              setActiveCanvasTabKey(tab.tabKey);
                              setDraftHighlightBox(null);
                              setHighlightTargetOutputId(null);
                              setTabDropdownOpen(false);
                            }}
                            className={`group relative flex items-center gap-2 rounded-lg px-3 py-1.5 text-xs font-semibold transition-all shrink-0 ${
                              isSelected
                                ? "bg-burnt text-white shadow-sm border border-burnt"
                                : "bg-[#141827] text-stone-100 border border-[#28304A] hover:text-white hover:border-burnt/60 hover:bg-[#1B2135]"
                            }`}
                          >
                            {isGenerating ? (
                              <Loader2 className="h-3.5 w-3.5 animate-spin text-burnt shrink-0" />
                            ) : hasFailedRun ? (
                              <AlertTriangle className="h-3.5 w-3.5 text-amber-500 shrink-0" />
                            ) : isMobile ? (
                              <Smartphone className="h-3.5 w-3.5 shrink-0 opacity-80" />
                            ) : isTablet ? (
                              <Tablet className="h-3.5 w-3.5 shrink-0 opacity-80" />
                            ) : isAsset ? (
                              <Sparkles className="h-3.5 w-3.5 shrink-0 text-burnt" />
                            ) : (
                              <Monitor className="h-3.5 w-3.5 shrink-0 opacity-80" />
                            )}

                            <span>{tab.tabLabel}</span>

                            <span
                              className={`rounded px-1.5 py-0.2 font-mono text-[10px] ${
                                isSelected
                                  ? "bg-white/20 text-white"
                                  : "bg-[#1F2538] text-stone-100"
                              }`}
                              title={`${tab.items.length} visual item(s) on this tab board`}
                            >
                              {tab.items.length}
                            </span>
                          </button>
                        );
                      })}

                      {canvasTabs.length > 1 && (
                        <button
                          type="button"
                          data-testid="canvas-tab-all"
                          onClick={() => {
                            setActiveCanvasTabKey("__all__");
                            setDraftHighlightBox(null);
                            setHighlightTargetOutputId(null);
                            setTabDropdownOpen(false);
                          }}
                          className={`flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs font-semibold transition-all shrink-0 ${
                            activeCanvasTabKey === "__all__"
                              ? "bg-burnt text-white shadow-sm"
                              : "bg-[#141827] text-stone-200 border border-[#28304A] hover:text-white hover:bg-[#1B2135]"
                          }`}
                        >
                          <Grid className="h-3.5 w-3.5" />
                          <span>All Tabs ({canvasTabs.length})</span>
                        </button>
                      )}
                    </>
                  )}
                </div>

                {/* Tab Overflow Arrow Dropdown + Scroll Controls + Run Details */}
                <div className="flex items-center gap-1.5 shrink-0">
                  {canvasTabs.length > 0 && (
                    <div className="relative flex items-center gap-1">
                      <button
                        type="button"
                        onClick={() =>
                          tabRowScrollRef.current?.scrollBy({
                            left: -200,
                            behavior: "smooth",
                          })
                        }
                        className="rounded-lg border border-[#28304A] bg-[#141827] p-1.5 text-stone-200 hover:bg-[#1B2135] hover:text-white hover:border-burnt/60"
                        title="Scroll tabs left"
                      >
                        <ChevronLeft className="h-3.5 w-3.5" />
                      </button>
                      <button
                        type="button"
                        onClick={() =>
                          tabRowScrollRef.current?.scrollBy({
                            left: 200,
                            behavior: "smooth",
                          })
                        }
                        className="rounded-lg border border-[#28304A] bg-[#141827] p-1.5 text-stone-200 hover:bg-[#1B2135] hover:text-white hover:border-burnt/60"
                        title="Scroll tabs right"
                      >
                        <ChevronRight className="h-3.5 w-3.5" />
                      </button>
                      <button
                        type="button"
                        data-testid="canvas-tabs-dropdown-btn"
                        onClick={() => setTabDropdownOpen((prev) => !prev)}
                        className={`inline-flex items-center gap-1.5 rounded-lg border px-2.5 py-1.5 text-xs font-bold transition-colors ${
                          tabDropdownOpen
                            ? "border-burnt bg-burnt text-white"
                            : "border-[#28304A] bg-[#141827] text-stone-100 hover:border-burnt/60 hover:bg-[#1B2135] hover:text-white"
                        }`}
                        title="Open dropdown menu of all tabs in this thread"
                      >
                        <span>Tabs ({canvasTabs.length + 1})</span>
                        <ChevronDown
                          className={`h-3.5 w-3.5 transition-transform ${
                            tabDropdownOpen ? "rotate-180" : ""
                          }`}
                        />
                      </button>

                      {tabDropdownOpen && (
                        <>
                          <div
                            className="fixed inset-0 z-40"
                            onClick={() => setTabDropdownOpen(false)}
                          />
                          <div
                            data-testid="canvas-tabs-dropdown-menu"
                            className="absolute right-0 top-full mt-2 w-72 max-h-96 overflow-y-auto rounded-xl border border-[#2E3856] bg-[#0F1320] p-1.5 shadow-2xl z-50"
                          >
                            <div className="px-2.5 py-1.5 text-[10px] font-bold uppercase tracking-wider text-stone-300 border-b border-[#1E2538] mb-1">
                              Jump to Tab ({canvasTabs.length + 1})
                            </div>

                            <button
                              type="button"
                              onClick={() => {
                                setActiveCanvasTabKey("__design_md__");
                                setDraftHighlightBox(null);
                                setHighlightTargetOutputId(null);
                                setTabDropdownOpen(false);
                              }}
                              className={`w-full flex items-center justify-between gap-2 rounded-lg px-2.5 py-2 text-left text-xs font-semibold transition-colors ${
                                activeCanvasTabKey === "__design_md__"
                                  ? "bg-burnt text-white"
                                  : "text-stone-100 hover:bg-[#181E31]"
                              }`}
                            >
                              <div className="flex items-center gap-2 min-w-0">
                                <FileCode2 className="h-3.5 w-3.5 shrink-0 text-amber-400" />
                                <span className="truncate">DESIGN.md</span>
                              </div>
                              <span className="rounded bg-[#1A2135] px-1.5 py-0.5 font-mono text-[10px] text-amber-300">
                                .md
                              </span>
                            </button>

                            {canvasTabs.map((tab) => {
                              const isSel = activeCanvasTabKey === tab.tabKey;
                              return (
                                <button
                                  key={`dropdown-${tab.tabKey}`}
                                  type="button"
                                  data-testid={`dropdown-tab-${tab.tabKey}`}
                                  onClick={() => {
                                    setActiveCanvasTabKey(tab.tabKey);
                                    setDraftHighlightBox(null);
                                    setHighlightTargetOutputId(null);
                                    setTabDropdownOpen(false);
                                  }}
                                  className={`w-full flex items-center justify-between gap-2 rounded-lg px-2.5 py-2 text-left text-xs font-semibold transition-colors mt-0.5 ${
                                    isSel
                                      ? "bg-burnt text-white"
                                      : "text-stone-100 hover:bg-[#181E31]"
                                  }`}
                                >
                                  <div className="flex items-center gap-2 min-w-0">
                                    {tab.aspectRatio === "9:16" ? (
                                      <Smartphone className="h-3.5 w-3.5 shrink-0 opacity-85" />
                                    ) : tab.kind === "asset" ? (
                                      <Sparkles className="h-3.5 w-3.5 shrink-0 text-amber-400" />
                                    ) : (
                                      <Monitor className="h-3.5 w-3.5 shrink-0 opacity-85" />
                                    )}
                                    <span className="truncate">
                                      {tab.tabLabel}
                                    </span>
                                  </div>
                                  <span
                                    className={`rounded px-1.5 py-0.5 font-mono text-[10px] shrink-0 ${
                                      isSel
                                        ? "bg-white/20 text-white"
                                        : "bg-[#1A2135] text-stone-200"
                                    }`}
                                  >
                                    {tab.items.length}{" "}
                                    {tab.items.length === 1 ? "item" : "items"}
                                  </span>
                                </button>
                              );
                            })}

                            {canvasTabs.length > 1 && (
                              <button
                                type="button"
                                onClick={() => {
                                  setActiveCanvasTabKey("__all__");
                                  setDraftHighlightBox(null);
                                  setHighlightTargetOutputId(null);
                                  setTabDropdownOpen(false);
                                }}
                                className={`w-full flex items-center justify-between gap-2 rounded-lg px-2.5 py-2 text-left text-xs font-semibold transition-colors mt-1 border-t border-[#1E2538] pt-2 ${
                                  activeCanvasTabKey === "__all__"
                                    ? "bg-burnt text-white"
                                    : "text-stone-200 hover:bg-[#181E31]"
                                }`}
                              >
                                <div className="flex items-center gap-2">
                                  <Grid className="h-3.5 w-3.5 shrink-0" />
                                  <span>All Tabs Overview</span>
                                </div>
                                <span className="font-mono text-[10px]">
                                  {canvasTabs.length}
                                </span>
                              </button>
                            )}
                          </div>
                        </>
                      )}
                    </div>
                  )}

                  {latestRun && (
                    <button
                      type="button"
                      onClick={() => {
                        setDetailsRunId(latestRun.id);
                        setDetailsDrawerOpen(true);
                      }}
                      className="rounded-lg border border-[#28304A] bg-[#141827] px-2.5 py-1.5 text-[11px] font-semibold text-stone-100 hover:bg-[#1B2135] hover:border-burnt/60"
                    >
                      Run Details
                    </button>
                  )}
                </div>
              </div>

              {/* =========================================================== */}
              {/* CANVAS STAGE BODY                                           */}
              {/* =========================================================== */}
              <div className="flex-1 overflow-auto p-4 lg:p-5">
                {activeCanvasTabKey === "__design_md__" && projectDetail ? (
                  <div className="h-full overflow-y-auto">
                    <DesignMdView
                      project={projectDetail.project}
                      brand={projectDetail.brand}
                      brandVersion={projectDetail.brand_version}
                      onOpenBrandEditor={() => setActiveView("brand_kit")}
                    />
                  </div>
                ) : activeCanvasTabKey === "__all__" ? (
                  /* ALL TABS OVERVIEW GRID */
                  <div className="space-y-4 max-w-7xl mx-auto">
                    <div className="flex items-center justify-between border-b border-stone-border pb-3">
                      <div>
                        <h2 className="text-base font-bold text-stone-ink">
                          All Thread Tabs ({canvasTabs.length})
                        </h2>
                        <p className="text-xs text-stone-muted">
                          Click any tab card to open its interactive canvas
                          board, drag components near each other, or copy
                          handoff prompts.
                        </p>
                      </div>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
                      {canvasTabs.map((tab) => {
                        const latestOut =
                          tab.items[tab.items.length - 1] || null;
                        return (
                          <div
                            key={tab.tabKey}
                            onClick={() => setActiveCanvasTabKey(tab.tabKey)}
                            className="group cursor-pointer flex flex-col rounded-2xl border border-stone-border bg-stone-surface p-3.5 shadow-card hover:border-burnt transition-all"
                          >
                            <div className="flex items-center justify-between mb-2.5">
                              <span className="text-xs font-bold text-stone-ink">
                                {tab.tabLabel}
                              </span>
                              <span className="rounded-md bg-stone-canvas px-2 py-0.5 font-mono text-[10px] font-semibold text-stone-muted border border-stone-border">
                                {tab.items.length}{" "}
                                {tab.items.length === 1 ? "image" : "images"}
                              </span>
                            </div>

                            <div
                              className={`relative aspect-[16/10] w-full overflow-hidden rounded-xl border border-stone-border bg-[#0C0F1A] ${
                                latestOut?.has_alpha ? "checkerboard-bg" : ""
                              }`}
                            >
                              {latestOut?.status === "completed" &&
                              latestOut.original_url ? (
                                <img
                                  src={
                                    latestOut.thumbnail_url ||
                                    latestOut.original_url
                                  }
                                  alt={tab.tabLabel}
                                  className="h-full w-full object-contain"
                                />
                              ) : (
                                <div className="flex h-full w-full items-center justify-center text-xs text-stone-muted">
                                  <Loader2 className="h-5 w-5 animate-spin text-burnt mr-2" />
                                  <span>Generating...</span>
                                </div>
                              )}
                            </div>
                          </div>
                        );
                      })}
                    </div>
                  </div>
                ) : currentTabGroup ? (
                  /* ========================================================= */
                  /* SINGLE ACTIVE TAB: MULTI-IMAGE DRAG & DROP CANVAS BOARD   */
                  /* ========================================================= */
                  <div className="flex flex-col h-full min-h-0 space-y-4">
                    {/* TAB STUDIO HEADER & WHOLE-TAB HANDOFF TOOLBAR */}
                    <div className="flex flex-wrap items-center justify-between gap-3 rounded-2xl border border-stone-border bg-stone-surface px-4 py-3 shadow-sm shrink-0">
                      <div className="flex flex-wrap items-center gap-3 min-w-0">
                        <div>
                          <div className="flex flex-wrap items-center gap-2">
                            <h1 className="text-base font-bold text-stone-ink">
                              {currentTabGroup.tabLabel}
                            </h1>
                            <span className="rounded-md bg-charcoal px-2 py-0.5 font-mono text-[10px] font-bold text-white">
                              {currentTabGroup.items.length}{" "}
                              {currentTabGroup.items.length === 1
                                ? "Canvas Item"
                                : "Canvas Items"}
                            </span>
                            <span className="hidden sm:inline-flex items-center gap-1.5 rounded-md bg-[#141827] border border-[#28304A] px-2.5 py-0.5 text-[10px] font-semibold text-stone-200">
                              <Move className="h-3 w-3 text-burnt" />
                              Scroll wheel to zoom · Drag canvas background to pan · Drag cards to place anywhere
                            </span>
                            {currentTabGroup.activeRunForTab && (
                              <RunStatusBadge
                                status={currentTabGroup.activeRunForTab.status}
                              />
                            )}
                          </div>
                        </div>
                      </div>

                      {/* Whole-Tab Controls & Handoff Actions */}
                      <div className="flex flex-wrap items-center gap-2">
                        {/* Board Zoom & Auto-Arrange */}
                        <div className="flex items-center gap-1 rounded-xl border border-[#28304A] bg-[#101420] p-1">
                          <button
                            type="button"
                            data-testid="canvas-zoom-out-btn"
                            onClick={() =>
                              setBoardScale((s) =>
                                Math.max(0.35, Math.round((s - 0.15) * 100) / 100)
                              )
                            }
                            className="rounded-lg p-1 text-stone-200 hover:bg-[#1E2538] hover:text-white"
                            title="Zoom Board Out (or use Mouse Scroll Wheel)"
                          >
                            <ZoomOut className="h-3.5 w-3.5" />
                          </button>
                          <button
                            type="button"
                            data-testid="canvas-zoom-reset-btn"
                            onClick={() => {
                              setBoardScale(1);
                              setBoardPan({ x: 0, y: 0 });
                            }}
                            className="px-2 font-mono text-[10px] font-bold text-stone-100 hover:text-amber-300"
                            title="Reset Board Zoom (100%) & Pan Center"
                          >
                            {Math.round(boardScale * 100)}%
                          </button>
                          <button
                            type="button"
                            data-testid="canvas-zoom-in-btn"
                            onClick={() =>
                              setBoardScale((s) =>
                                Math.min(2.4, Math.round((s + 0.15) * 100) / 100)
                              )
                            }
                            className="rounded-lg p-1 text-stone-200 hover:bg-[#1E2538] hover:text-white"
                            title="Zoom Board In (or use Mouse Scroll Wheel)"
                          >
                            <ZoomIn className="h-3.5 w-3.5" />
                          </button>
                          {hasCustomCardPositions && (
                            <button
                              type="button"
                              data-testid="canvas-auto-arrange-btn"
                              onClick={resetTabCardPositions}
                              className="ml-1 inline-flex items-center gap-1 rounded-lg bg-[#1A2033] border border-burnt/50 px-2 py-1 text-[10px] font-bold text-amber-300 shadow-sm hover:bg-burnt hover:text-white transition-colors"
                              title="Reset pan, zoom, and dragged card positions back to aligned grid"
                            >
                              <LayoutGrid className="h-3 w-3" />
                              <span>Reset View & Layout</span>
                            </button>
                          )}
                        </div>

                        {/* Quick Mobile Adaptation into this tab */}
                        {currentTabGroup.items.some(
                          (o) =>
                            o.status === "completed" &&
                            o.aspect_ratio !== "9:16" &&
                            o.kind !== "asset"
                        ) && (
                          <button
                            type="button"
                            disabled={isChatLocked}
                            onClick={() =>
                              sendChatMessage({
                                overrideText: `Inside the ${currentTabGroup.tabLabel} tab, generate a Mobile (9:16) layout version next to the desktop design maintaining strict visual continuity.`,
                                forceGenerate: true,
                              })
                            }
                            className="inline-flex items-center gap-1.5 rounded-xl border border-[#28304A] bg-[#141827] px-3 py-1.5 text-xs font-semibold text-stone-100 hover:border-burnt hover:bg-[#1B2135] disabled:opacity-40 transition-colors"
                            title="Generate a Mobile (9:16) variant right inside this tab next to the current design"
                          >
                            <Smartphone className="h-3.5 w-3.5 text-burnt" />
                            <span>+ Mobile (9:16)</span>
                          </button>
                        )}

                        {/* WHOLE-TAB COPY PROMPT BUTTON */}
                        {currentTabGroup.items.some(
                          (o) => o.status === "completed"
                        ) && (
                          <>
                            <button
                              type="button"
                              data-testid="copy-whole-tab-for-agent"
                              onClick={() =>
                                handleCopyWholeTab(currentTabGroup)
                              }
                              className="inline-flex items-center gap-1.5 rounded-xl bg-burnt px-4 py-1.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover transition-colors"
                              title="Copy unified Coding-Agent Handoff Prompt for ALL images & transparent assets in this tab"
                            >
                              {copiedId === `tab:${currentTabGroup.tabKey}` ? (
                                <>
                                  <Check className="h-3.5 w-3.5" />
                                  <span>Copied Whole Tab Prompt ✓</span>
                                </>
                              ) : (
                                <>
                                  <Copy className="h-3.5 w-3.5" />
                                  <span>
                                    Copy Prompt for Whole Tab (
                                    {
                                      currentTabGroup.items.filter(
                                        (o) => o.status === "completed"
                                      ).length
                                    }
                                    )
                                  </span>
                                </>
                              )}
                            </button>

                            <button
                              type="button"
                              onClick={() =>
                                handleInspectTabHandoff(currentTabGroup)
                              }
                              className="inline-flex items-center gap-1 rounded-xl border border-[#28304A] bg-[#141827] px-2.5 py-1.5 text-xs font-semibold text-stone-100 hover:bg-[#1B2135] hover:border-burnt/60"
                              title="Preview full Coding-Agent Handoff Markdown for this tab"
                            >
                              <Eye className="h-3.5 w-3.5 text-stone-300" />
                              <span>Tab Handoff</span>
                            </button>

                            <button
                              type="button"
                              onClick={() =>
                                handleExportTabBundle(currentTabGroup)
                              }
                              className="inline-flex items-center gap-1 rounded-xl border border-[#28304A] bg-[#141827] px-2.5 py-1.5 text-xs font-semibold text-stone-100 hover:bg-[#1B2135] hover:border-burnt/60"
                              title="Export portable .zip bundle for this tab"
                            >
                              <FileArchive className="h-3.5 w-3.5 text-stone-300" />
                              <span>.zip</span>
                            </button>
                          </>
                        )}
                      </div>
                    </div>

                    {/* Full-Page Plan Review Banner if awaiting approval */}
                    {currentTabGroup.activeRunForTab?.status ===
                      "awaiting_plan_review" && (
                      <FullPagePlanReview
                        run={currentTabGroup.activeRunForTab}
                        onApproved={() => {
                          refetchThreadDetail();
                          showToast(
                            "Plan approved! Rendering coordinated design set..."
                          );
                        }}
                      />
                    )}

                    {/* Highlight Mode Instruction Banner */}
                    {highlightTargetOutputId && (
                      <div className="flex items-center justify-between rounded-xl border border-burnt bg-burnt/10 px-4 py-2.5 text-xs text-stone-ink shrink-0">
                        <div className="flex items-center gap-2">
                          <Crosshair className="h-4 w-4 text-burnt shrink-0" />
                          <span>
                            <strong>Highlight Edit Mode Active:</strong> Click
                            and drag a rectangle directly over the highlighted
                            card below, add a comment, and send to edit that
                            region with{" "}
                            <code className="font-mono font-bold text-burnt">
                              Sunburst 2.5 Medium
                            </code>
                            .
                          </span>
                        </div>
                        <button
                          type="button"
                          onClick={() => {
                            setHighlightTargetOutputId(null);
                            setDraftHighlightBox(null);
                          }}
                          className="text-xs font-bold text-burnt hover:underline"
                        >
                          Exit Highlight Mode
                        </button>
                      </div>
                    )}

                    {/* ===================================================== */}
                    {/* INTERACTIVE 2D DRAGGABLE & ZOOMABLE CANVAS BOARD      */}
                    {/* ===================================================== */}
                    <div
                      data-testid="interactive-tab-canvas-board"
                      onWheel={(e) => {
                        e.preventDefault();
                        const factor = e.deltaY < 0 ? 0.08 : -0.08;
                        setBoardScale((s) =>
                          Math.max(
                            0.35,
                            Math.min(2.4, Math.round((s + factor) * 100) / 100)
                          )
                        );
                      }}
                      onMouseDown={(e) => {
                        const target = e.target as HTMLElement;
                        // Allow middle-mouse pan anywhere or left-mouse pan on canvas background
                        if (
                          e.button === 1 ||
                          !target.closest(
                            "[data-canvas-card], button, a, textarea, input"
                          )
                        ) {
                          e.preventDefault();
                          setPanningBoard({
                            startX: e.clientX,
                            startY: e.clientY,
                            origX: boardPan.x,
                            origY: boardPan.y,
                          });
                        }
                      }}
                      style={{
                        backgroundImage:
                          "radial-gradient(#262E46 1.25px, transparent 1.25px)",
                        backgroundSize: `${Math.max(10, Math.round(20 * boardScale))}px ${Math.max(10, Math.round(20 * boardScale))}px`,
                        backgroundPosition: `${boardPan.x}px ${boardPan.y}px`,
                      }}
                      className={`relative flex-1 min-h-[520px] overflow-hidden rounded-2xl border border-stone-border bg-[#0A0D16] p-5 shadow-inner space-y-4 ${
                        panningBoard
                          ? "cursor-grabbing select-none"
                          : "cursor-grab"
                      }`}
                    >
                      {/* Floating Bottom-Right Interactive Canvas HUD (Zoom + Pan + Reset) */}
                      <div className="pointer-events-auto absolute bottom-3.5 right-3.5 z-50 flex items-center gap-1.5 rounded-xl border border-[#2A3350] bg-[#0F1320]/95 px-2.5 py-1.5 text-[11px] text-stone-200 shadow-lg backdrop-blur-md">
                        <span className="font-mono text-[10px] text-stone-300">
                          Zoom:{" "}
                          <strong className="text-white">
                            {Math.round(boardScale * 100)}%
                          </strong>
                        </span>
                        <span className="text-stone-600">·</span>
                        <span className="font-mono text-[10px] text-stone-300">
                          Pan:{" "}
                          <strong className="text-white">
                            {Math.round(boardPan.x)}, {Math.round(boardPan.y)}
                          </strong>
                        </span>
                        {(boardScale !== 1 ||
                          boardPan.x !== 0 ||
                          boardPan.y !== 0) && (
                          <button
                            type="button"
                            onClick={(e) => {
                              e.stopPropagation();
                              setBoardScale(1);
                              setBoardPan({ x: 0, y: 0 });
                            }}
                            className="ml-1 rounded-md bg-burnt/20 border border-burnt/40 px-1.5 py-0.5 text-[10px] font-bold text-amber-300 hover:bg-burnt hover:text-white transition-colors"
                          >
                            Center
                          </button>
                        )}
                      </div>

                      {/* Live Streaming Progress Bar when tab has existing items and a new run is active */}
                      {currentTabGroup.items.length > 0 &&
                        currentTabGroup.activeRunForTab && (
                          <div className="relative z-40 flex items-center justify-between gap-3 rounded-xl border border-burnt/40 bg-charcoal px-4 py-2.5 text-xs text-white shadow-md">
                            <div className="flex items-center gap-2.5 min-w-0">
                              <Loader2 className="h-4 w-4 animate-spin text-burnt shrink-0" />
                              <span className="font-bold shrink-0">
                                Live Stream:
                              </span>
                              <span className="truncate font-mono text-[11px] text-amber-200">
                                {liveStreamMessage ||
                                  (currentTabGroup.activeRunForTab.status ===
                                  "planning"
                                    ? "GLM 5.3 Flash is planning coordinated visual items..."
                                    : "Sunburst 2.5 is rendering visual items into this tab...")}
                              </span>
                            </div>
                            <RunStatusBadge
                              status={currentTabGroup.activeRunForTab.status}
                            />
                          </div>
                        )}

                      {/* Error Banner inside Tab when a run in this tab failed or partially failed */}
                      {!currentTabGroup.activeRunForTab &&
                        currentTabGroup.failedRunForTab &&
                        currentTabGroup.items.length > 0 && (
                          <div className="relative z-40 flex flex-wrap items-center justify-between gap-3 rounded-xl border-2 border-danger-ink/40 bg-danger-bg px-4 py-3 text-xs text-danger-ink shadow-sm">
                            <div className="flex items-center gap-2 min-w-0">
                              <AlertTriangle className="h-4 w-4 shrink-0" />
                              <div className="min-w-0">
                                <div className="font-bold">
                                  Backend Error During Generation (
                                  {currentTabGroup.failedRunForTab.status})
                                </div>
                                <div className="truncate font-mono text-[11px] opacity-90">
                                  {currentTabGroup.failedRunForTab
                                    .error_message ||
                                    "One or more items stopped unexpectedly."}
                                </div>
                              </div>
                            </div>
                            <button
                              type="button"
                              disabled={
                                retryingRunId ===
                                currentTabGroup.failedRunForTab.id
                              }
                              onClick={() =>
                                handleRetryRun(
                                  currentTabGroup.failedRunForTab!.id
                                )
                              }
                              className="inline-flex items-center gap-1.5 rounded-lg bg-danger-ink px-3 py-1.5 text-xs font-bold text-white hover:opacity-90 disabled:opacity-50"
                            >
                              <RefreshCw
                                className={`h-3.5 w-3.5 ${
                                  retryingRunId ===
                                  currentTabGroup.failedRunForTab.id
                                    ? "animate-spin"
                                    : ""
                                }`}
                              />
                              <span>Retry Failed Items</span>
                            </button>
                          </div>
                        )}

                      {currentTabGroup.items.length === 0 ? (
                        currentTabGroup.failedRunForTab &&
                        !currentTabGroup.activeRunForTab ? (
                          /* Actionable Error Stage when planning failed before outputs were created */
                          <div
                            data-testid="tab-failed-error-stage"
                            className="relative flex min-h-[420px] w-full flex-col items-center justify-center rounded-2xl border-2 border-danger-ink/30 bg-stone-surface p-8 text-center shadow-card"
                          >
                            <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-danger-bg text-danger-ink mb-3 shadow-sm">
                              <AlertTriangle className="h-6 w-6" />
                            </div>
                            <h3 className="text-base font-bold text-stone-ink">
                              Generation Stopped on &ldquo;
                              {currentTabGroup.tabLabel}&rdquo;
                            </h3>
                            <p className="mt-1 max-w-lg text-xs text-stone-muted">
                              The backend encountered an error while preparing
                              or rendering this tab:
                            </p>
                            <div className="mt-3 max-w-xl w-full rounded-xl border border-danger-ink/25 bg-danger-bg/60 p-3.5 font-mono text-xs text-danger-ink text-left break-words">
                              {currentTabGroup.failedRunForTab.error_message ||
                                `Run stopped with status '${currentTabGroup.failedRunForTab.status}'.`}
                            </div>
                            <div className="mt-4 flex items-center gap-3">
                              <button
                                type="button"
                                data-testid="retry-failed-run-btn"
                                disabled={
                                  retryingRunId ===
                                  currentTabGroup.failedRunForTab.id
                                }
                                onClick={() =>
                                  handleRetryRun(
                                    currentTabGroup.failedRunForTab!.id
                                  )
                                }
                                className="inline-flex items-center gap-2 rounded-xl bg-burnt px-5 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-50 transition-colors"
                              >
                                <RefreshCw
                                  className={`h-4 w-4 ${
                                    retryingRunId ===
                                    currentTabGroup.failedRunForTab.id
                                      ? "animate-spin"
                                      : ""
                                  }`}
                                />
                                <span>Retry Generation Now</span>
                              </button>
                            </div>
                          </div>
                        ) : (
                          /* Live Skeleton when run just started and outputs are still being planned */
                          <div
                            data-testid="live-generating-stage"
                            className="relative flex min-h-[420px] w-full flex-col items-center justify-center overflow-hidden rounded-2xl border border-stone-border bg-stone-surface/90 p-8 text-center canvas-shimmer-overlay"
                          >
                            <div className="canvas-scanline-bar" />
                            <div className="flex h-12 w-12 items-center justify-center rounded-2xl bg-burnt/15 text-burnt mb-3 shadow-sm">
                              <Loader2 className="h-6 w-6 animate-spin" />
                            </div>
                            <h3 className="text-sm font-bold text-stone-ink">
                              Planning & Generating &ldquo;
                              {currentTabGroup.tabLabel}&rdquo;...
                            </h3>
                            <p className="mt-1 max-w-md text-xs text-stone-muted">
                              {liveStreamMessage ||
                                "GLM 5.3 Flash + Sunburst 2.5 are preparing your visual assets for this tab board..."}
                            </p>
                          </div>
                        )
                      ) : (
                        <div
                          style={{
                            transform: `translate3d(${boardPan.x}px, ${boardPan.y}px, 0) scale(${boardScale})`,
                            transformOrigin: "top left",
                          }}
                          className={`grid gap-6 ${
                            panningBoard || draggingCard
                              ? ""
                              : "transition-transform duration-75"
                          } ${
                            currentTabGroup.items.length === 1
                              ? "grid-cols-1 max-w-4xl mx-auto"
                              : "grid-cols-1 xl:grid-cols-2"
                          }`}
                        >
                          {currentTabGroup.items.map((itemOut, idx) => {
                            const isCardSelected =
                              activeTabOutput?.id === itemOut.id;
                            const isHighlightActiveOnCard =
                              highlightTargetOutputId === itemOut.id;
                            const offset = cardOffsets[itemOut.id] || {
                              x: 0,
                              y: 0,
                            };
                            const isItemGenerating = [
                              "queued",
                              "rendering",
                              "saving",
                            ].includes(itemOut.status);

                            return (
                              <div
                                key={itemOut.id}
                                data-canvas-card="true"
                                data-testid={`canvas-card-${itemOut.id}`}
                                onClick={() =>
                                  setSelectedOutputByTab((prev) => ({
                                    ...prev,
                                    [currentTabGroup.tabKey]: itemOut.id,
                                  }))
                                }
                                style={{
                                  transform: `translate3d(${offset.x}px, ${offset.y}px, 0)`,
                                  zIndex:
                                    draggingCard?.outputId === itemOut.id
                                      ? 40
                                      : isCardSelected
                                      ? 20
                                      : 10,
                                }}
                                className={`flex flex-col rounded-2xl border bg-[#121624] shadow-card transition-shadow ${
                                  isHighlightActiveOnCard
                                    ? "border-burnt ring-4 ring-burnt/30"
                                    : isCardSelected
                                    ? "border-burnt ring-2 ring-burnt/60"
                                    : "border-[#28304A] hover:border-stone-400"
                                }`}
                              >
                                {/* CARD DRAG HEADER + WELL-PLACED VIEW LARGER BUTTON */}
                                <div
                                  onMouseDown={(e) => {
                                    if (
                                      (e.target as HTMLElement).closest(
                                        "button, a"
                                      )
                                    )
                                      return;
                                    e.preventDefault();
                                    e.stopPropagation();
                                    setSelectedOutputByTab((prev) => ({
                                      ...prev,
                                      [currentTabGroup.tabKey]: itemOut.id,
                                    }));
                                    setDraggingCard({
                                      outputId: itemOut.id,
                                      startX: e.clientX,
                                      startY: e.clientY,
                                      origX: offset.x,
                                      origY: offset.y,
                                    });
                                  }}
                                  className="flex items-center justify-between gap-2 border-b border-[#262E45] bg-[#161B2C] px-3.5 py-2.5 rounded-t-2xl cursor-grab active:cursor-grabbing select-none"
                                  title="Drag card header to move this component anywhere on the canvas board"
                                >
                                  <div className="flex items-center gap-2 min-w-0">
                                    <GripHorizontal className="h-4 w-4 text-stone-300 shrink-0" />
                                    <span className="rounded bg-[#0B0E17] border border-[#2A3350] px-1.5 py-0.5 font-mono text-[10px] font-bold text-white shrink-0">
                                      #{idx + 1}
                                    </span>
                                    <span className="truncate text-xs font-bold text-white">
                                      {itemOut.title}
                                    </span>
                                    {itemOut.kind === "page" &&
                                      (itemOut.page_route ||
                                        itemOut.item_spec?.page_route) && (
                                        <span className="rounded-md bg-burnt/20 border border-burnt/40 px-1.5 py-0.5 font-mono text-[10px] font-bold text-amber-300 shrink-0">
                                          Route:{" "}
                                          {itemOut.page_route ||
                                            itemOut.item_spec?.page_route}
                                        </span>
                                      )}
                                    {itemOut.kind === "page" &&
                                      (itemOut.role === "visual_anchor" ||
                                      itemOut.item_spec?.is_visual_anchor ? (
                                        <span className="rounded-md bg-amber-950/90 border border-amber-500/50 px-1.5 py-0.5 text-[9px] font-bold text-amber-200 shrink-0">
                                          Visual Anchor
                                        </span>
                                      ) : (
                                        <span className="rounded-md bg-indigo-950/90 border border-indigo-500/50 px-1.5 py-0.5 text-[9px] font-bold text-indigo-200 shrink-0">
                                          Img2Img Shared Navbar
                                        </span>
                                      ))}
                                    {itemOut.extracted_from_output_id && (
                                      <span className="rounded-md bg-emerald-950/90 border border-emerald-500/50 px-1.5 py-0.5 text-[9px] font-bold text-emerald-200 shrink-0">
                                        Extracted Transparent Asset
                                      </span>
                                    )}
                                    {!itemOut.extracted_from_output_id &&
                                      itemOut.has_alpha && (
                                        <span className="rounded-md bg-emerald-950/90 border border-emerald-500/50 px-1.5 py-0.5 text-[9px] font-bold text-emerald-200 shrink-0">
                                          Alpha PNG ✓
                                        </span>
                                      )}
                                  </div>

                                  <div className="flex items-center gap-1.5 shrink-0">
                                    {itemOut.item_spec?.viewport?.intent &&
                                      itemOut.item_spec.viewport.intent !==
                                        "isolated" && (
                                        <span className="rounded bg-[#0D101B] px-1.5 py-0.5 font-mono text-[10px] font-semibold text-stone-200 border border-[#28304A] capitalize">
                                          {itemOut.item_spec.viewport.intent}{" "}
                                          {itemOut.item_spec.viewport
                                            .css_width || ""}
                                          px
                                        </span>
                                      )}
                                    <span className="rounded bg-[#0D101B] px-1.5 py-0.5 font-mono text-[10px] text-stone-200 border border-[#28304A]">
                                      {itemOut.aspect_ratio}
                                    </span>
                                    {itemOut.status === "completed" &&
                                      itemOut.original_url && (
                                        <button
                                          type="button"
                                          data-testid={`view-larger-btn-${itemOut.id}`}
                                          onClick={(e) => {
                                            e.stopPropagation();
                                            setViewerOutput(itemOut);
                                          }}
                                          className="inline-flex items-center gap-1 rounded-lg border border-[#2A3350] bg-[#0D101B] px-2.5 py-1 text-[11px] font-semibold text-stone-100 hover:border-burnt hover:bg-burnt hover:text-white transition-colors"
                                          title="Open clean Fullscreen Lightbox Viewer"
                                        >
                                          <Maximize2 className="h-3 w-3" />
                                          <span>View Larger</span>
                                        </button>
                                      )}
                                    <button
                                      type="button"
                                      data-testid={`delete-output-btn-${itemOut.id}`}
                                      disabled={deletingOutputId === itemOut.id}
                                      onClick={(e) => {
                                        e.stopPropagation();
                                        handleDeleteOutput(
                                          itemOut,
                                          currentTabGroup
                                        );
                                      }}
                                      className="inline-flex items-center gap-1 rounded-lg border border-red-500/40 bg-red-950/50 px-2 py-1 text-[11px] font-semibold text-red-200 hover:border-red-400 hover:bg-red-600 hover:text-white disabled:opacity-40 transition-colors"
                                      title="Delete this image/asset from the tab and update the tab's handoff prompt (closes tab if empty)"
                                    >
                                      <Trash2 className="h-3 w-3" />
                                      <span>Delete</span>
                                    </button>
                                  </div>
                                </div>

                                {/* CARD IMAGE VIEWPORT */}
                                <div className="p-3 flex-1 flex flex-col justify-between">
                                  {itemOut.status === "completed" &&
                                  itemOut.original_url ? (
                                    <div
                                      onMouseDown={(e) =>
                                        handleImageMouseDown(
                                          e,
                                          itemOut,
                                          currentTabGroup.tabKey,
                                          currentTabGroup.tabLabel
                                        )
                                      }
                                      onMouseMove={handleImageMouseMove}
                                      onMouseUp={handleImageMouseUp}
                                      onDoubleClick={() =>
                                        setViewerOutput(itemOut)
                                      }
                                      className={`relative w-full overflow-hidden rounded-xl border border-stone-border bg-[#0C0F1A] flex items-center justify-center select-none ${
                                        itemOut.has_alpha
                                          ? "checkerboard-bg"
                                          : ""
                                      } ${
                                        isHighlightActiveOnCard
                                          ? "cursor-crosshair"
                                          : "cursor-pointer"
                                      }`}
                                      title={
                                        isHighlightActiveOnCard
                                          ? "Drag to draw a highlight region on this image"
                                          : "Click to select card · Double-click to open Fullscreen Viewer"
                                      }
                                    >
                                      <img
                                        src={itemOut.original_url}
                                        alt={itemOut.title}
                                        draggable={false}
                                        className={`w-full object-contain mx-auto block ${
                                          ["2:3", "9:16", "3:4"].includes(
                                            itemOut.aspect_ratio
                                          )
                                            ? "max-h-[78vh]"
                                            : "max-h-[54vh]"
                                        }`}
                                      />

                                      {/* DRAWN HIGHLIGHT BOUNDING BOX + COMMENT POPOVER */}
                                      {draftHighlightBox &&
                                        draftHighlightBox.outputId ===
                                          itemOut.id && (
                                          <div
                                            data-testid="highlight-bounding-box"
                                            style={{
                                              left: `${draftHighlightBox.x_pct}%`,
                                              top: `${draftHighlightBox.y_pct}%`,
                                              width: `${draftHighlightBox.w_pct}%`,
                                              height: `${draftHighlightBox.h_pct}%`,
                                            }}
                                            onMouseDown={(e) =>
                                              e.stopPropagation()
                                            }
                                            className="absolute z-30 border-2 border-burnt bg-burnt/20 shadow-[0_0_0_9999px_rgba(24,23,21,0.38)] rounded-md"
                                          >
                                            <span className="absolute -top-6 left-0 rounded bg-burnt px-2 py-0.5 font-mono text-[10px] font-bold text-white shadow">
                                              {Math.round(
                                                draftHighlightBox.x_pct
                                              )}
                                              %,{" "}
                                              {Math.round(
                                                draftHighlightBox.y_pct
                                              )}
                                              % (
                                              {Math.round(
                                                draftHighlightBox.w_pct
                                              )}
                                              ×
                                              {Math.round(
                                                draftHighlightBox.h_pct
                                              )}
                                              %)
                                            </span>

                                            {!isDrawingHighlight && (
                                              <div
                                                onClick={(e) =>
                                                  e.stopPropagation()
                                                }
                                                className="absolute left-0 top-full mt-2 w-80 rounded-xl border border-[#2A3350] bg-[#141827] p-3 shadow-modal text-xs space-y-2.5 z-40"
                                              >
                                                <div className="flex items-center justify-between">
                                                  <span className="font-bold text-white flex items-center gap-1.5">
                                                    <Crosshair className="h-3.5 w-3.5 text-burnt" />
                                                    Edit Highlighted Region
                                                  </span>
                                                  <span className="rounded bg-burnt/20 px-1.5 py-0.5 font-mono text-[9px] font-semibold text-amber-300">
                                                    Sunburst 2.5 Medium
                                                  </span>
                                                </div>

                                                <textarea
                                                  rows={2}
                                                  autoFocus
                                                  data-testid="highlight-comment-input"
                                                  value={highlightComment}
                                                  onChange={(e) =>
                                                    setHighlightComment(
                                                      e.target.value
                                                    )
                                                  }
                                                  placeholder="Comment what to change here (e.g., Add a 'Book Demo' button with an arrow icon)..."
                                                  className="w-full rounded-lg border border-[#2A324D] bg-[#0B0E18] p-2 text-xs text-white placeholder:text-stone-300 focus:border-burnt focus:bg-[#101524] focus:outline-none"
                                                />

                                                <div className="flex items-center justify-end gap-1.5">
                                                  <button
                                                    type="button"
                                                    onClick={() =>
                                                      setDraftHighlightBox(null)
                                                    }
                                                    className="rounded-lg px-2.5 py-1 text-[11px] font-medium text-stone-300 hover:bg-[#1E2538]"
                                                  >
                                                    Cancel
                                                  </button>
                                                  <button
                                                    type="button"
                                                    disabled={
                                                      !highlightComment.trim()
                                                    }
                                                    onClick={() =>
                                                      handleSubmitHighlightToChat(
                                                        false
                                                      )
                                                    }
                                                    className="rounded-lg border border-[#28304A] bg-[#0D101B] px-2.5 py-1 text-[11px] font-semibold text-stone-100 hover:bg-[#1E2538] disabled:opacity-40"
                                                  >
                                                    Queue in Chat
                                                  </button>
                                                  <button
                                                    type="button"
                                                    data-testid="send-highlight-edit-btn"
                                                    disabled={
                                                      !highlightComment.trim() ||
                                                      isChatLocked
                                                    }
                                                    onClick={() =>
                                                      handleSubmitHighlightToChat(
                                                        true
                                                      )
                                                    }
                                                    className="rounded-lg bg-burnt px-3 py-1 text-[11px] font-bold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-40"
                                                  >
                                                    Send & Edit Now
                                                  </button>
                                                </div>
                                              </div>
                                            )}
                                          </div>
                                        )}
                                    </div>
                                  ) : isItemGenerating ? (
                                    <div className="relative flex min-h-[280px] w-full flex-col items-center justify-center overflow-hidden rounded-xl border border-stone-border bg-[#0C0F1A] p-6 text-center canvas-shimmer-overlay">
                                      <div className="canvas-scanline-bar" />
                                      <Loader2 className="h-7 w-7 animate-spin text-burnt mb-2" />
                                      <div className="text-xs font-bold text-stone-ink">
                                        Rendering &ldquo;{itemOut.title}
                                        &rdquo;...
                                      </div>
                                      <div className="text-[11px] text-stone-muted mt-0.5 capitalize">
                                        Status: {itemOut.status}
                                      </div>
                                    </div>
                                  ) : (
                                    <div className="flex min-h-[240px] w-full flex-col items-center justify-center rounded-xl bg-danger-bg/30 p-6 text-center">
                                      <AlertTriangle className="h-6 w-6 text-danger-ink mb-1.5" />
                                      <div className="text-xs font-bold text-danger-ink uppercase">
                                        {itemOut.status.replace("_", " ")}
                                      </div>
                                      <p className="mt-1 text-[11px] text-stone-muted max-w-xs">
                                        {itemOut.error_message ||
                                          "Generation encountered an error."}
                                      </p>
                                      <button
                                        type="button"
                                        onClick={() =>
                                          handleRetryOutput(itemOut.id)
                                        }
                                        className="mt-3 inline-flex items-center gap-1.5 rounded-lg bg-charcoal px-3 py-1.5 text-xs font-semibold text-white hover:bg-charcoal-hover"
                                      >
                                        <RefreshCw className="h-3 w-3" />
                                        <span>Retry Item</span>
                                      </button>
                                    </div>
                                  )}

                                  {/* CARD ACTION BAR (WELL-PLACED PER-IMAGE BUTTONS) */}
                                  {itemOut.status === "completed" && (
                                    <div className="mt-3 pt-2.5 border-t border-[#262E45] flex flex-wrap items-center justify-between gap-2">
                                      <div className="flex flex-wrap items-center gap-1.5">
                                        {/* Copy Prompt for THIS Specific Image */}
                                        <button
                                          type="button"
                                          data-testid={`copy-image-prompt-${itemOut.id}`}
                                          onClick={(e) => {
                                            e.stopPropagation();
                                            handleCopySingleOutput(itemOut);
                                          }}
                                          className="inline-flex items-center gap-1.5 rounded-xl bg-burnt px-3 py-1.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover transition-colors"
                                          title="Copy Coding-Agent Handoff Prompt for this specific image"
                                        >
                                          {copiedId === itemOut.id ? (
                                            <>
                                              <Check className="h-3.5 w-3.5 text-emerald-300" />
                                              <span>Copied Prompt ✓</span>
                                            </>
                                          ) : (
                                            <>
                                              <Copy className="h-3.5 w-3.5 text-amber-300" />
                                              <span>
                                                Copy Prompt (This Image)
                                              </span>
                                            </>
                                          )}
                                        </button>

                                        {/* Highlight & Edit Region on THIS Image */}
                                        {!itemOut.extracted_from_output_id &&
                                          itemOut.kind !== "asset" && (
                                            <>
                                              <button
                                                type="button"
                                                data-testid={`highlight-edit-btn-${itemOut.id}`}
                                                onClick={(e) => {
                                                  e.stopPropagation();
                                                  setSelectedOutputByTab((prev) => ({
                                                    ...prev,
                                                    [currentTabGroup.tabKey]:
                                                      itemOut.id,
                                                  }));
                                                  setHighlightTargetOutputId(
                                                    isHighlightActiveOnCard
                                                      ? null
                                                      : itemOut.id
                                                  );
                                                  setDraftHighlightBox(null);
                                                }}
                                                className={`inline-flex items-center gap-1 rounded-xl border px-2.5 py-1.5 text-xs font-semibold transition-colors ${
                                                  isHighlightActiveOnCard
                                                    ? "border-burnt bg-burnt text-white"
                                                    : "border-[#28304A] bg-[#161B2C] text-stone-100 hover:border-burnt hover:bg-[#1E2538]"
                                                }`}
                                                title="Draw a highlight box on this image to edit with Sunburst 2.5 Medium"
                                              >
                                                <Crosshair className="h-3.5 w-3.5" />
                                                <span>
                                                  {isHighlightActiveOnCard
                                                    ? "Drag on Image..."
                                                    : "Highlight & Edit"}
                                                </span>
                                              </button>

                                              <button
                                                type="button"
                                                disabled={isChatLocked}
                                                onClick={(e) => {
                                                  e.stopPropagation();
                                                  handleExtractMissingAssets(
                                                    itemOut.id
                                                  );
                                                }}
                                                className="inline-flex items-center gap-1 rounded-xl border border-[#28304A] bg-[#161B2C] px-2.5 py-1.5 text-xs font-semibold text-stone-100 hover:border-burnt hover:bg-[#1E2538] disabled:opacity-40 transition-colors"
                                                title="Inspect this rendered design and extract any unextracted visual assets, 3D props, plants, logos, or photos onto transparent backgrounds in this tab"
                                              >
                                                <Sparkles className="h-3.5 w-3.5 text-amber-400" />
                                                <span>Extract Assets</span>
                                              </button>
                                            </>
                                          )}
                                      </div>

                                      <div className="flex items-center gap-1.5">
                                        <button
                                          type="button"
                                          disabled={isChatLocked}
                                          onClick={(e) => {
                                            e.stopPropagation();
                                            handlePromoteOutputToReference(
                                              itemOut
                                            );
                                          }}
                                          className="inline-flex items-center gap-1 rounded-lg border border-[#28304A] bg-[#161B2C] px-2.5 py-1 text-[11px] font-semibold text-stone-100 hover:border-burnt hover:bg-[#1E2538] disabled:opacity-40"
                                          title="Attach this image to the chat composer as a reference"
                                        >
                                          <Layers className="h-3 w-3 text-burnt" />
                                          <span>Ref</span>
                                        </button>

                                        {itemOut.original_url && (
                                          <a
                                            href={itemOut.original_url}
                                            onClick={(e) => e.stopPropagation()}
                                            download={`${itemOut.title
                                              .toLowerCase()
                                              .replace(/[^a-z0-9]+/g, "-")}.png`}
                                            className="inline-flex items-center gap-1 rounded-lg border border-[#28304A] bg-[#161B2C] px-2.5 py-1 text-[11px] font-semibold text-stone-100 hover:border-burnt hover:bg-[#1E2538]"
                                            title="Download original PNG"
                                          >
                                            <Download className="h-3 w-3" />
                                            <span>PNG</span>
                                          </a>
                                        )}
                                      </div>
                                    </div>
                                  )}
                                </div>
                              </div>
                            );
                          })}
                        </div>
                      )}
                    </div>
                  </div>
                ) : (
                  /* EMPTY THREAD CANVAS STATE */
                  <div className="flex h-full min-h-[420px] flex-col items-center justify-center rounded-2xl border border-dashed border-stone-border bg-stone-surface/60 p-8 text-center">
                    <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-burnt/10 text-burnt mb-3">
                      <Layers className="h-6 w-6" />
                    </div>
                    <h2 className="text-base font-bold text-stone-ink">
                      {loadingThread
                        ? "Loading thread..."
                        : `Multi-Tab Interactive Canvas for "${threadTitle}"`}
                    </h2>
                    <p className="mt-1 max-w-lg text-xs text-stone-muted leading-relaxed">
                      Use the sideways chat on the left to discuss your design,
                      paste one or more reference screenshots (`⌘V`), and
                      generate a <strong>Full Website</strong> (all pages,
                      buttons, icon sheets & transparent assets in one tab),
                      individual <strong>Component Tabs</strong> (`Navbar`,
                      `Hero`), or open the <strong>DESIGN.md</strong> tab above!
                    </p>
                  </div>
                )}
              </div>
            </main>
          </div>
        )}
      </div>

      {/* =================================================================== */}
      {/* ADD / PICK REFERENCE ASSET DIALOG                                   */}
      {/* =================================================================== */}
      <LightDismissDialog
        open={addAssetModalOpen}
        onClose={() => setAddAssetModalOpen(false)}
        title="Attach Reference Images & Assets with AI Instructions"
        subtitle="Choose how the AI should use this asset, add an optional comment, and upload or pick from your project library."
        maxWidthClass="max-w-xl"
      >
        <AddAssetDialogBody
          projectAssets={projectAssets}
          onUploadFile={async (file, role, notes) => {
            await uploadAndAttachFiles([file], role, notes);
            setAddAssetModalOpen(false);
          }}
          onSelectExisting={(asset, role, notes) => {
            handleAttachExistingAsset(asset, role, notes);
            setAddAssetModalOpen(false);
          }}
        />
      </LightDismissDialog>

      {/* =================================================================== */}
      {/* FULLSCREEN STUDIO LIGHTBOX MODAL                                    */}
      {/* =================================================================== */}
      <ImageViewerModal
        open={Boolean(viewerOutput)}
        onClose={() => setViewerOutput(null)}
        activeOutput={viewerOutput}
        allOutputs={viewerFilmstripOutputs}
        onSelectOutput={(o) => setViewerOutput(o)}
        onCopyPrompt={(o) => handleCopySingleOutput(o)}
        onStartHighlightEdit={(o) => {
          if (currentTabGroup) {
            setSelectedOutputByTab((prev) => ({
              ...prev,
              [currentTabGroup.tabKey]: o.id,
            }));
          }
          setHighlightTargetOutputId(o.id);
          setDraftHighlightBox(null);
        }}
        copiedId={copiedId}
      />

      {/* =================================================================== */}
      {/* RUN DIAGNOSTICS & DETAILS DRAWER                                    */}
      {/* =================================================================== */}
      <DetailsDrawer
        open={detailsDrawerOpen}
        onClose={() => setDetailsDrawerOpen(false)}
        run={latestRun}
        projectAssets={projectAssets}
        onCheckConsistency={handleCheckConsistency}
        checkingConsistency={checkingConsistency}
        onAdoptTokens={handleAdoptRunTokens}
        adoptingTokens={adoptingTokens}
      />

      {/* =================================================================== */}
      {/* HANDOFF PACKET & PORTABLE BUNDLE DIALOG                             */}
      {/* =================================================================== */}
      {(activeHandoffRunId || latestRun) && (
        <HandoffDialog
          open={handoffModalOpen}
          onClose={() => setHandoffModalOpen(false)}
          handoff={activeHandoff}
          runId={activeHandoffRunId || latestRun!.id}
          outputId={activeHandoffOutputId}
          outputIds={activeHandoffOutputIds}
          tabLabel={activeHandoffTabLabel}
          onHandoffUpdated={(updated) => {
            setActiveHandoff(updated);
            const rId = activeHandoffRunId || latestRun!.id;
            const key = activeHandoffOutputId
              ? `output:${activeHandoffOutputId}`
              : `run:${rId}`;
            handoffCacheRef.current[key] = updated;
          }}
        />
      )}

      {/* =================================================================== */}
      {/* FLOATING TOAST NOTIFICATION                                         */}
      {/* =================================================================== */}
      {toastMessage && (
        <div className="fixed bottom-4 right-4 z-50 flex items-center gap-2 rounded-xl bg-charcoal px-4 py-2.5 text-xs font-semibold text-white shadow-modal border border-charcoal-border">
          <CheckCircle2 className="h-4 w-4 text-emerald-400 shrink-0" />
          <span>{toastMessage}</span>
        </div>
      )}
    </div>
  );
}

function RunStatusBadge({ status }: { status: RunDto["status"] }) {
  if (status === "completed") {
    return (
      <span className="inline-flex items-center gap-1 rounded-full bg-success-bg px-2 py-0.5 text-[10px] font-semibold text-success-ink">
        <span>Completed</span>
        <Check className="h-3 w-3" />
      </span>
    );
  }
  if (
    ["queued", "planning", "ready", "rendering", "saving"].includes(status)
  ) {
    return (
      <span className="inline-flex items-center gap-1.5 rounded-full bg-warning-bg px-2.5 py-0.5 text-[10px] font-semibold text-warning-ink">
        <Loader2 className="h-3 w-3 animate-spin" />
        <span className="capitalize">{status}...</span>
      </span>
    );
  }
  if (status === "awaiting_plan_review") {
    return (
      <span className="inline-flex items-center gap-1 rounded-full bg-warning-bg px-2.5 py-0.5 text-[10px] font-semibold text-warning-ink">
        <span>Awaiting Plan Review</span>
      </span>
    );
  }
  if (status === "partial") {
    return (
      <span className="inline-flex items-center gap-1 rounded-full bg-warning-bg px-2.5 py-0.5 text-[10px] font-semibold text-warning-ink">
        <span>Partial Completion</span>
      </span>
    );
  }
  return (
    <span className="inline-flex items-center gap-1 rounded-full bg-danger-bg px-2.5 py-0.5 text-[10px] font-semibold text-danger-ink capitalize">
      {status.replace("_", " ")}
    </span>
  );
}

const DIALOG_ROLE_PILLS: Array<{
  role: AttachmentRole;
  label: string;
}> = [
  { role: "include_asset", label: "Include in Design" },
  { role: "brand_logo", label: "Brand Logo" },
  { role: "current_screenshot", label: "Current UI Screenshot" },
  { role: "style_reference", label: "Style Inspiration" },
  { role: "content_reference", label: "Content Reference" },
];

function AddAssetDialogBody({
  projectAssets,
  onUploadFile,
  onSelectExisting,
}: {
  projectAssets: AssetDto[];
  onUploadFile: (
    file: File,
    role: AttachmentRole,
    notes: string
  ) => Promise<void>;
  onSelectExisting: (
    asset: AssetDto,
    role: AttachmentRole,
    notes?: string
  ) => void;
}) {
  const [role, setRole] = useState<AttachmentRole>("include_asset");
  const [notes, setNotes] = useState("");
  const [uploading, setUploading] = useState(false);

  return (
    <div className="space-y-4 text-xs">
      <div className="space-y-1.5">
        <label className="block font-bold text-stone-ink">
          1. Select Role for AI
        </label>
        <div className="flex flex-wrap gap-1.5">
          {DIALOG_ROLE_PILLS.map((rp) => (
            <button
              key={rp.role}
              type="button"
              onClick={() => setRole(rp.role)}
              className={`rounded-xl border px-3 py-1.5 text-xs font-semibold transition-all ${
                role === rp.role
                  ? "border-burnt bg-burnt text-white"
                  : "border-[#28304A] bg-[#141827] text-stone-100 hover:border-burnt"
              }`}
            >
              {rp.label}
            </button>
          ))}
        </div>
      </div>

      <div>
        <label className="block font-bold text-stone-ink mb-1">
          2. Comment / Instructions for AI (Optional)
        </label>
        <textarea
          rows={2}
          value={notes}
          onChange={(e) => setNotes(e.target.value)}
          placeholder="e.g., Place this logo in the top-left of the navbar, or use this screenshot as the current UI and add a 'Book Demo' button..."
          className="w-full rounded-xl border border-[#2A3350] bg-[#0B0E17] p-2.5 text-xs text-white placeholder:text-stone-400 focus:border-burnt focus:outline-none"
        />
      </div>

      <div className="rounded-xl border-2 border-dashed border-[#2A3350] bg-[#0F1320] p-4 text-center">
        <label className="inline-flex cursor-pointer items-center gap-2 rounded-xl bg-burnt px-4 py-2 text-xs font-bold text-white hover:bg-burnt-hover shadow-sm">
          <Upload className="h-3.5 w-3.5" />
          <span>{uploading ? "Uploading..." : "Upload New Image or SVG"}</span>
          <input
            type="file"
            accept="image/png,image/jpeg,image/webp,image/svg+xml"
            className="hidden"
            onChange={async (e) => {
              const file = e.target.files?.[0];
              if (!file) return;
              setUploading(true);
              try {
                await onUploadFile(file, role, notes);
              } finally {
                setUploading(false);
              }
            }}
          />
        </label>
      </div>

      {projectAssets.length > 0 && (
        <div>
          <div className="font-bold text-stone-ink mb-2">
            Or pick from your Project Assets Library ({projectAssets.length})
          </div>
          <div className="grid grid-cols-2 sm:grid-cols-3 gap-2.5 max-h-60 overflow-y-auto pr-1">
            {projectAssets.map((a) => (
              <button
                key={a.id}
                type="button"
                onClick={() =>
                  onSelectExisting(a, role, notes.trim() || a.placement_notes)
                }
                className="flex flex-col items-start rounded-xl border border-[#28304A] bg-[#141827] p-2 text-left hover:border-burnt transition-colors"
              >
                <img
                  src={a.thumbnail_url}
                  alt={a.label}
                  className="h-20 w-full rounded-lg object-contain checkerboard-bg mb-1.5 p-1 border border-stone-border/60"
                />
                <div className="truncate w-full font-bold text-white">
                  {a.label}
                </div>
                {a.placement_notes && (
                  <div className="truncate w-full text-[10px] text-amber-300">
                    {a.placement_notes}
                  </div>
                )}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

export default function WorkspacePage() {
  return (
    <Suspense
      fallback={
        <div className="flex h-screen w-screen items-center justify-center bg-stone-canvas text-stone-muted text-sm">
          Loading Design Gen workbench...
        </div>
      }
    >
      <WorkspaceContent />
    </Suspense>
  );
}
