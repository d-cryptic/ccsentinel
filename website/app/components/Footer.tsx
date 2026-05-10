const REPO = "https://github.com/d-cryptic/ccsentinel";

const links = [
  { label: "GITHUB", href: REPO },
  { label: "DOCS", href: `${REPO}#readme` },
  { label: "ISSUES", href: `${REPO}/issues` },
  { label: "MIT LICENSE", href: `${REPO}/blob/main/LICENSE` },
];

export function Footer() {
  return (
    <footer className="bg-bg border-t-2 border-border">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10 sm:py-12 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-6">
        <div>
          <div className="flex items-center gap-2">
            <span className="font-vt text-3xl text-cyan glow-cyan leading-none">
              [CST]
            </span>
            <span className="font-mono text-xs uppercase tracking-widest text-dim">
              claude_sentinel
            </span>
          </div>
          <p className="mt-3 font-mono text-[12px] text-dim max-w-sm leading-relaxed">
            Intelligent account, profile, and session manager for Claude Code.
          </p>
        </div>

        <div className="flex flex-wrap gap-x-6 gap-y-2">
          {links.map((l) => (
            <a
              key={l.label}
              href={l.href}
              target="_blank"
              rel="noopener noreferrer"
              className="font-mono text-[11px] uppercase tracking-widest text-dim hover:text-cyan hover:[text-shadow:_0_0_8px_#00FFFF] transition-all"
            >
              &gt; {l.label}
            </a>
          ))}
        </div>

        <div className="font-mono text-[11px] uppercase tracking-widest text-neongreen glow-green">
          BUILT WITH RUST{" "}
          <span aria-hidden="true">&hearts;</span>
        </div>
      </div>

      {/* Anthropic disclaimer */}
      <div className="border-t border-border/50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <p className="font-mono text-[10px] text-dim/60 leading-relaxed text-center">
            Claude Sentinel is an independent, open-source project. It is not affiliated with, endorsed by, or associated with Anthropic PBC.
            &ldquo;Claude&rdquo; and &ldquo;Claude Code&rdquo; are trademarks of Anthropic PBC.
            This tool interacts with Claude Code exclusively through officially documented configuration mechanisms.
          </p>
        </div>
      </div>
    </footer>
  );
}
