type Props = { className?: string };

export function TerminalCard({ className = "" }: Props) {
  return (
    <div
      className={`relative w-full max-w-2xl mx-auto bg-black neon-border-green crt overflow-hidden ${className}`}
    >
      {/* Window chrome */}
      <div className="flex items-center gap-2 px-4 py-2 border-b border-neongreen/40 bg-black">
        <span className="h-3 w-3 rounded-full bg-magenta" />
        <span className="h-3 w-3 rounded-full bg-yellow" />
        <span className="h-3 w-3 rounded-full bg-neongreen" />
        <span className="ml-3 text-[11px] text-neongreen/70 font-mono uppercase tracking-widest">
          ~/work/backend &mdash; cst
        </span>
      </div>

      {/* Body — phosphor green-on-black */}
      <div className="p-5 sm:p-6 font-mono text-[13px] leading-relaxed phosphor">
        <div>
          <span>&gt;</span> ~/work/backend
          <span className="text-magenta"> git:(main)</span>
        </div>
        <div className="mt-1">
          <span className="text-cyan">$</span>{" "}
          <span>cst use work:backend</span>
        </div>

        <div className="mt-3 space-y-1">
          <div>
            <span className="text-cyan">[OK]</span> Switched to profile{" "}
            <span className="text-yellow">work</span>
            <span>:</span>
            <span className="text-yellow">backend</span>
          </div>
          <div className="pl-3 opacity-90">
            auth: <span className="text-cyan">oauth</span> &middot; slot:{" "}
            <span className="text-cyan">2/4</span> &middot; usage:{" "}
            <span>23%</span>
          </div>
          <div className="pl-3 opacity-90">
            CLAUDE_CONFIG_DIR ={" "}
            <span className="text-cyan">~/.claude-sentinel/work/backend</span>
          </div>
        </div>

        <div className="mt-4">
          <span className="text-cyan">$</span> <span>claude</span>
        </div>
        <div className="mt-1">
          <span className="text-magenta">&#9679;</span> Claude Code{" "}
          <span className="opacity-70">v1.0 &middot; ready</span>
        </div>
        <div className="mt-1 opacity-80">
          Listening for prompts&hellip;
          <span className="inline-block w-2 h-4 bg-neongreen align-middle ml-1 animate-blink" />
        </div>
      </div>
    </div>
  );
}
