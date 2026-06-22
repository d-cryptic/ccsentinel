"use client";

import { useState } from "react";

type Props = {
  command: string;
  className?: string;
};

export function CopyCommand({ command, className = "" }: Props) {
  const [copied, setCopied] = useState(false);

  const onCopy = async () => {
    try {
      await navigator.clipboard.writeText(command);
      setCopied(true);
      setTimeout(() => setCopied(false), 1600);
    } catch {
      /* ignore */
    }
  };

  return (
    <div
      className={`inline-flex items-center gap-3 rounded-xl border border-line bg-surface pl-4 pr-2 py-2 font-mono text-sm ${className}`}
    >
      <span className="text-faint select-none" aria-hidden="true">
        $
      </span>
      <code className="text-text">{command}</code>
      <button
        type="button"
        onClick={onCopy}
        aria-label="Copy install command"
        className="ml-1 inline-flex items-center justify-center h-7 w-7 rounded-lg border border-line text-muted hover:text-text hover:border-accent/40 transition-colors"
      >
        {copied ? (
          <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            aria-hidden="true"
          >
            <path
              d="m5 12 4 4 10-10"
              stroke="#5EC4B6"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        ) : (
          <svg
            width="14"
            height="14"
            viewBox="0 0 24 24"
            fill="none"
            aria-hidden="true"
          >
            <rect
              x="8"
              y="8"
              width="12"
              height="12"
              rx="2"
              stroke="currentColor"
              strokeWidth="1.6"
            />
            <path
              d="M16 8V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h2"
              stroke="currentColor"
              strokeWidth="1.6"
            />
          </svg>
        )}
      </button>
    </div>
  );
}
