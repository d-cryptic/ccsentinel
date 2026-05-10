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
      className={`inline-flex items-center gap-3 bg-black neon-border-cyan rounded-none pl-4 pr-2 py-2 font-mono text-sm ${className}`}
    >
      <span className="text-neongreen select-none" aria-hidden="true">
        $
      </span>
      <code className="text-cyan glow-cyan">{command}</code>
      <button
        type="button"
        onClick={onCopy}
        aria-label="Copy install command"
        className="ml-1 inline-flex items-center justify-center h-7 w-7 border border-cyan text-cyan hover:bg-cyan hover:text-bg transition-colors"
      >
        {copied ? (
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <path
              d="m5 12 4 4 10-10"
              stroke="#39FF14"
              strokeWidth="2.4"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        ) : (
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" aria-hidden="true">
            <rect
              x="8"
              y="8"
              width="12"
              height="12"
              rx="1"
              stroke="currentColor"
              strokeWidth="1.8"
            />
            <path
              d="M16 8V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h2"
              stroke="currentColor"
              strokeWidth="1.8"
            />
          </svg>
        )}
      </button>
    </div>
  );
}
