type Props = { className?: string };

export function TerminalCard({ className = "" }: Props) {
  return (
    <div
      className={`relative w-full max-w-2xl mx-auto card overflow-hidden shadow-2xl shadow-black/40 ${className}`}
    >
      {/* Window chrome */}
      <div className="flex items-center gap-2 px-4 py-3 border-b border-line">
        <span className="h-2.5 w-2.5 rounded-full bg-line" />
        <span className="h-2.5 w-2.5 rounded-full bg-line" />
        <span className="h-2.5 w-2.5 rounded-full bg-line" />
        <span className="ml-3 text-[11px] text-faint font-mono">
          ~/work/backend — cst
        </span>
      </div>

      {/* Body */}
      <div className="p-5 sm:p-6 font-mono text-[13px] leading-relaxed bg-ink/40">
        <div className="text-faint">
          <span className="text-muted">~/work/backend</span>
          <span className="text-accent-dim"> git:(main)</span>
        </div>
        <div className="mt-1">
          <span className="text-accent">$</span>{" "}
          <span className="text-text">cst use work:backend</span>
        </div>

        <div className="mt-3 space-y-1">
          <div className="text-muted">
            <span className="text-accent">✓</span> Switched to profile{" "}
            <span className="text-text">work</span>
            <span className="text-faint">:</span>
            <span className="text-text">backend</span>
          </div>
          <div className="pl-3 text-faint">
            auth: <span className="text-muted">oauth</span> · slot:{" "}
            <span className="text-muted">2/4</span> · usage:{" "}
            <span className="text-muted">23%</span>
          </div>
          <div className="pl-3 text-faint">
            CLAUDE_CONFIG_DIR ={" "}
            <span className="text-muted">~/.claude-sentinel/work/backend</span>
          </div>
        </div>

        <div className="mt-4">
          <span className="text-accent">$</span>{" "}
          <span className="text-text">claude</span>
        </div>
        <div className="mt-1 text-muted">
          <span className="text-accent">●</span> Claude Code{" "}
          <span className="text-faint">v1.0 · ready</span>
        </div>
        <div className="mt-1 text-faint">
          Listening for prompts…
          <span className="inline-block w-1.5 h-3.5 bg-accent/70 align-middle ml-1" />
        </div>
      </div>
    </div>
  );
}
