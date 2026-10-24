"use client";

import { useState } from "react";
import { ArrowUp } from "lucide-react";

interface ClarificationCardProps {
  messageId: string;
  question: string;
  options: string[];
  disabled: boolean;
  onAnswer: (answer: string, messageId: string) => Promise<boolean>;
}

export function ClarificationCard({ messageId, question, options, disabled, onAnswer }: ClarificationCardProps) {
  const [customAnswer, setCustomAnswer] = useState("");

  async function sendCustomAnswer() {
    const answer = customAnswer.trim();
    if (!answer || disabled) return;
    if (await onAnswer(answer, messageId)) setCustomAnswer("");
  }

  return (
    <div className="mt-2 w-full space-y-2" role="group" aria-label={question} data-testid="clarification-card">
      <div className="space-y-1.5">
        {options.map((option) => (
          <button
            key={option}
            type="button"
            disabled={disabled}
            data-testid="clarification-option"
            onClick={() => void onAnswer(option, messageId)}
            className="w-full rounded-xl border border-stone-border bg-stone-card px-3 py-2 text-left text-xs leading-relaxed text-stone-ink transition-colors hover:border-crimson-500 hover:bg-crimson-950 disabled:cursor-wait disabled:opacity-50"
          >
            {option}
          </button>
        ))}
      </div>
      <form
        onSubmit={(event) => { event.preventDefault(); void sendCustomAnswer(); }}
        className="flex items-end gap-2"
      >
        <textarea
          aria-label="Custom answer"
          placeholder="Or write your own answer…"
          rows={2}
          disabled={disabled}
          value={customAnswer}
          onChange={(event) => setCustomAnswer(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter" && !event.shiftKey && !event.nativeEvent.isComposing) {
              event.preventDefault();
              void sendCustomAnswer();
            }
          }}
          className="min-w-0 flex-1 resize-y rounded-xl border border-stone-border px-3 py-2 text-xs leading-relaxed disabled:opacity-50"
        />
        <button
          type="submit"
          aria-label="Send answer"
          disabled={disabled || !customAnswer.trim()}
          className="rounded-xl bg-crimson-600 p-2.5 text-white hover:bg-crimson-700 disabled:cursor-not-allowed disabled:opacity-40"
        >
          <ArrowUp className="h-4 w-4" />
        </button>
      </form>
      <p className="px-1 text-[10px] leading-relaxed text-stone-muted">Choose an option to send it, or write a custom answer.</p>
    </div>
  );
}
