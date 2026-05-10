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
  if (s === "active") return "text-neongreen";
  if (s === "limited") return "text-magenta";
  return "text-gray-500";
}

function usageColor(u: number) {
  if (u >= 85) return "text-magenta";
  if (u >= 60) return "text-yellow";
  return "text-neongreen";
}

export function TerminalDemo() {
  return (
    <section className="bg-bg relative overflow-hidden border-t border-border">
      <div
        aria-hidden="true"
        className="absolute -top-32 left-1/2 -translate-x-1/2 h-72 w-[40rem] rounded-full bg-magenta/15 blur-3xl"
      />
      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 sm:py-28">
        <div className="max-w-2xl">
          <p className="eyebrow text-cyan">&gt; LIVE VIEW_</p>
          <h2 className="mt-4 font-vt text-5xl sm:text-6xl lg:text-7xl uppercase text-yellow glow-yellow leading-none">
            {"// LIVE DASHBOARD"}
          </h2>
          <p className="mt-5 font-mono text-sm sm:text-base text-gray-400 leading-relaxed">
            <code className="text-cyan">cst top</code> gives you an htop-style
            dashboard with active profiles, auth types, token usage, and session
            activity at a glance.
          </p>
        </div>

        {/* CRT Monitor frame */}
        <div className="mt-12 p-2 sm:p-3 neon-border-magenta bg-black">
          <div className="bg-black border border-magenta/40 crt overflow-hidden">
            <div className="flex items-center gap-2 px-4 py-2 border-b border-magenta/40">
              <span className="h-3 w-3 rounded-full bg-magenta" />
              <span className="h-3 w-3 rounded-full bg-yellow" />
              <span className="h-3 w-3 rounded-full bg-neongreen" />
              <span className="ml-3 text-[11px] text-cyan font-mono uppercase tracking-widest">
                cst top &mdash; 5 profiles
              </span>
            </div>

            <div className="p-4 sm:p-6 font-mono text-[12px] sm:text-[12.5px] leading-relaxed overflow-x-auto phosphor">
              <div className="mb-3">
                <span className="text-cyan">claude-sentinel</span>{" "}
                <span className="text-gray-500">v0.4.2</span>{" "}
                <span className="text-gray-500">&middot;</span>{" "}
                <span className="text-white">5 profiles</span>{" "}
                <span className="text-gray-500">&middot;</span>{" "}
                <span className="text-neongreen">4 healthy</span>{" "}
                <span className="text-gray-500">&middot;</span>{" "}
                <span className="text-magenta">1 limited</span>
              </div>

              <div className="grid grid-cols-[minmax(120px,1.5fr)_70px_1fr_70px_70px] gap-3 sm:gap-4 text-cyan border-b border-cyan/30 pb-2 mb-2 uppercase text-[11px] tracking-widest">
                <div>PROFILE</div>
                <div>AUTH</div>
                <div>USAGE</div>
                <div>STATUS</div>
                <div className="text-right">RESET</div>
              </div>

              {rows.map((r) => (
                <div
                  key={r.name}
                  className="grid grid-cols-[minmax(120px,1.5fr)_70px_1fr_70px_70px] gap-3 sm:gap-4 py-1 items-center"
                >
                  <div className="text-white truncate">{r.name}</div>
                  <div className="text-cyan">{r.auth}</div>
                  <div className="flex items-center gap-2">
                    <span className={usageColor(r.usage)}>{bar(r.usage)}</span>
                    <span className="text-gray-500">{r.usage}%</span>
                  </div>
                  <div className={statusColor(r.status)}>{r.status}</div>
                  <div className="text-gray-500 text-right">{r.reset}</div>
                </div>
              ))}

              <div className="mt-5 pt-3 border-t border-cyan/30 flex flex-wrap gap-3 sm:gap-4 text-gray-500">
                <span>
                  <span className="text-yellow">[u]</span> use
                </span>
                <span>
                  <span className="text-yellow">[r]</span> rotate
                </span>
                <span>
                  <span className="text-yellow">[d]</span> daemon
                </span>
                <span>
                  <span className="text-yellow">[q]</span> quit
                </span>
                <span className="ml-auto">
                  <span className="text-neongreen animate-blink">&#9679;</span>{" "}
                  auto-switch on
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
