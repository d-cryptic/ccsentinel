import { ShieldIcon } from "./ShieldIcon";

const REPO = "https://github.com/d-cryptic/ccsentinel";

const links = [
  { label: "GitHub", href: REPO },
  { label: "Docs", href: `${REPO}#readme` },
  { label: "Issues", href: `${REPO}/issues` },
  { label: "MIT License", href: `${REPO}/blob/main/LICENSE` },
];

export function Footer() {
  return (
    <footer className="border-t border-line">
      <div className="max-w-6xl mx-auto px-5 sm:px-8 py-12 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-8">
        <div>
          <div className="flex items-center gap-2.5">
            <ShieldIcon className="h-5 w-5 text-accent" />
            <span className="font-serif text-lg tracking-tight text-text">
              Claude Sentinel
            </span>
          </div>
          <p className="mt-3 text-sm text-faint max-w-sm leading-relaxed">
            Intelligent account, profile, and session manager for Claude Code.
          </p>
        </div>

        <div className="flex flex-wrap gap-x-7 gap-y-2">
          {links.map((l) => (
            <a
              key={l.label}
              href={l.href}
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm text-muted hover:text-text transition-colors"
            >
              {l.label}
            </a>
          ))}
        </div>

        <div className="text-sm text-faint">Built with Rust</div>
      </div>

      <div className="border-t border-line">
        <div className="max-w-6xl mx-auto px-5 sm:px-8 py-5">
          <p className="text-[11px] text-faint/70 leading-relaxed text-center max-w-3xl mx-auto">
            Claude Sentinel is an independent, open-source project. It is not
            affiliated with, endorsed by, or associated with Anthropic PBC.
            &ldquo;Claude&rdquo; and &ldquo;Claude Code&rdquo; are trademarks of
            Anthropic PBC. This tool interacts with Claude Code exclusively
            through officially documented configuration mechanisms.
          </p>
        </div>
      </div>
    </footer>
  );
}
