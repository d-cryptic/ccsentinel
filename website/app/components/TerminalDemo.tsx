type Row = {
  name: string;
  auth: string;
  usage: number;
  status: "active" | "idle" | "limited";
  reset: string;
};

const rows: Row[] = [
  { name: "work:backend", auth: "oauth", usage: 23, status: "active", reset: "—" },
  { name: "work:frontend", auth: "oauth", usage: 67, status: "idle", reset: "1h 12m" },
  { name: "personal", auth: "api", usage: 41, status: "idle", reset: "—" },
  { name: "team-shared", auth: "api", usage: 92, status: "limited", reset: "23m" },
  { name: "research", auth: "vertex", usage: 8, status: "idle", reset: "—" },
];

function bar(usage: number) {
  const total = 20;
  const filled = Math.round((usage / 100) * total);
  return "█".repeat(filled) + "░".repeat(total - filled);
}

function statusColor(s: Row["status"]) {
  if (s === "active") return "text-accent";
  if (s === "limited") return "text-rose-300/80";
  return "text-faint";
}

function usageColor(u: number) {
  if (u >= 85) return "text-rose-300/70";
  if (u >= 60) return "text-amber-200/70";
  return "text-accent/80";
}

export function TerminalDemo() {
  return (
    <section className="border-t border-line">
      <div className="max-w-6xl mx-auto px-5 sm:px-8 py-24 sm:py-32">
        <div className="max-w-2xl">
          <p className="eyebrow">Live view</p>
          <h2 className="mt-5 display text-4xl sm:text-5xl">Live dashboard</h2>
          <p className="mt-5 text-base text-muted leading-relaxed">
            <code className="code-chip">cst top</code> gives you an htop-style
            dashboard with active profiles, auth types, token usage, and session
            activity at a glance.
          </p>
        </div>

        <div className="mt-12 card overflow-hidden shadow-2xl shadow-black/40">
          <div className="flex items-center gap-2 px-4 py-3 border-b border-line">
            <span className="h-2.5 w-2.5 rounded-full bg-line" />
            <span className="h-2.5 w-2.5 rounded-full bg-line" />
            <span className="h-2.5 w-2.5 rounded-full bg-line" />
            <span className="ml-3 text-[11px] text-faint font-mono">
              cst top — 5 profiles
            </span>
          </div>

          <div className="p-4 sm:p-6 font-mono text-[12px] sm:text-[12.5px] leading-relaxed overflow-x-auto bg-ink/40">
            <div className="mb-4 text-faint">
              <span className="text-muted">claude-sentinel</span> v0.4.2{" "}
              <span className="text-line">·</span>{" "}
              <span className="text-text">5 profiles</span>{" "}
              <span className="text-line">·</span>{" "}
              <span className="text-accent">4 healthy</span>{" "}
              <span className="text-line">·</span>{" "}
              <span className="text-rose-300/80">1 limited</span>
            </div>

            <div className="grid grid-cols-[minmax(120px,1.5fr)_70px_1fr_70px_70px] gap-3 sm:gap-4 text-faint border-b border-line pb-2 mb-2 uppercase text-[10.5px] tracking-widest">
              <div>Profile</div>
              <div>Auth</div>
              <div>Usage</div>
              <div>Status</div>
              <div className="text-right">Reset</div>
            </div>

            {rows.map((r) => (
              <div
                key={r.name}
                className="grid grid-cols-[minmax(120px,1.5fr)_70px_1fr_70px_70px] gap-3 sm:gap-4 py-1 items-center"
              >
                <div className="text-text truncate">{r.name}</div>
                <div className="text-muted">{r.auth}</div>
                <div className="flex items-center gap-2">
                  <span className={usageColor(r.usage)}>{bar(r.usage)}</span>
                  <span className="text-faint">{r.usage}%</span>
                </div>
                <div className={statusColor(r.status)}>{r.status}</div>
                <div className="text-faint text-right">{r.reset}</div>
              </div>
            ))}

            <div className="mt-5 pt-3 border-t border-line flex flex-wrap gap-4 text-faint">
              <span>
                <span className="text-muted">u</span> use
              </span>
              <span>
                <span className="text-muted">r</span> rotate
              </span>
              <span>
                <span className="text-muted">d</span> daemon
              </span>
              <span>
                <span className="text-muted">q</span> quit
              </span>
              <span className="ml-auto inline-flex items-center gap-1.5">
                <span className="text-accent">●</span> auto-switch on
              </span>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
