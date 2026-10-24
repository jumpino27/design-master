"use client";

import React, { useEffect, useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

export function Providers({ children }: { children: React.ReactNode }) {
  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            staleTime: 3000,
            retry: 1,
            refetchOnWindowFocus: false,
          },
        },
      })
  );

  // Sync aria-invalid with :user-invalid and provide WeakMap fallback when needed
  useEffect(() => {
    const syncAria = (el: EventTarget | null) => {
      if (!(el instanceof HTMLElement)) return;
      if (
        el instanceof HTMLInputElement ||
        el instanceof HTMLTextAreaElement ||
        el instanceof HTMLSelectElement
      ) {
        try {
          if (CSS.supports("selector(:user-invalid)")) {
            el.setAttribute(
              "aria-invalid",
              el.matches(":user-invalid") ? "true" : "false"
            );
          } else {
            const valid = el.checkValidity();
            el.classList.toggle("user-invalid-fallback", !valid);
            el.classList.toggle("user-valid-fallback", valid);
            el.setAttribute("aria-invalid", !valid ? "true" : "false");
          }
        } catch {
          // ignore
        }
      }
    };

    const onBlur = (e: Event) => syncAria(e.target);
    const onInput = (e: Event) => {
      if (
        e.target instanceof HTMLElement &&
        e.target.hasAttribute("aria-invalid")
      ) {
        syncAria(e.target);
      }
    };

    document.addEventListener("blur", onBlur, true);
    document.addEventListener("input", onInput, true);
    return () => {
      document.removeEventListener("blur", onBlur, true);
      document.removeEventListener("input", onInput, true);
    };
  }, []);

  return (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
}
