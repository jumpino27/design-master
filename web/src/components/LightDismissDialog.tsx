"use client";

import React, { useEffect, useId, useRef } from "react";
import { X } from "lucide-react";

interface LightDismissDialogProps {
  open: boolean;
  onClose: () => void;
  title: string;
  subtitle?: string;
  maxWidthClass?: string;
  children: React.ReactNode;
}

export function LightDismissDialog({
  open,
  onClose,
  title,
  subtitle,
  maxWidthClass = "max-w-3xl",
  children,
}: LightDismissDialogProps) {
  const dialogRef = useRef<HTMLDialogElement | null>(null);
  const openPropRef = useRef(open);
  openPropRef.current = open;
  const titleId = useId();

  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;

    if (open && !dialog.open) {
      dialog.showModal();
    } else if (!open && dialog.open) {
      dialog.close();
    }
  }, [open]);

  useEffect(() => {
    const dialog = dialogRef.current;
    if (!dialog) return;

    const handleNativeClose = () => {
      if (openPropRef.current) {
        onClose();
      }
    };
    dialog.addEventListener("close", handleNativeClose);

    // Fallback for browsers without <dialog closedby="any"> support
    const supportsClosedBy =
      typeof HTMLDialogElement !== "undefined" &&
      "closedBy" in HTMLDialogElement.prototype;

    const handleBackdropClick = (event: MouseEvent) => {
      if (event.target !== dialog) return;
      const rect = dialog.getBoundingClientRect();
      const isDialogContent =
        rect.top <= event.clientY &&
        event.clientY <= rect.top + rect.height &&
        rect.left <= event.clientX &&
        event.clientX <= rect.left + rect.width;
      if (isDialogContent) return;
      dialog.close();
    };

    if (!supportsClosedBy) {
      dialog.addEventListener("click", handleBackdropClick);
    }

    return () => {
      dialog.removeEventListener("close", handleNativeClose);
      if (!supportsClosedBy) {
        dialog.removeEventListener("click", handleBackdropClick);
      }
    };
  }, [onClose]);

  return (
    <dialog
      ref={dialogRef}
      {...({ closedby: "any" } as Record<string, string>)}
      aria-labelledby={titleId}
      className={`w-full ${maxWidthClass} rounded-xl border border-stone-border bg-stone-surface p-0 text-stone-ink shadow-2xl open:flex open:flex-col max-h-[90vh]`}
    >
      <div className="flex items-center justify-between border-b border-stone-border px-6 py-4 bg-white rounded-t-xl">
        <div>
          <h2 id={titleId} className="text-base font-semibold text-stone-ink">
            {title}
          </h2>
          {subtitle && (
            <p className="text-xs text-stone-muted mt-0.5">{subtitle}</p>
          )}
        </div>
        <button
          type="button"
          onClick={() => dialogRef.current?.close()}
          aria-label="Close dialog"
          className="rounded-lg p-1.5 text-stone-muted hover:bg-stone-canvas hover:text-stone-ink focus-visible:outline focus-visible:outline-2 focus-visible:outline-burnt"
        >
          <X className="h-4 w-4" />
        </button>
      </div>
      <div className="overflow-y-auto p-6 flex-1">{children}</div>
    </dialog>
  );
}
