type IconProps = { className?: string };

const icons = {
  profile: (p: IconProps) => (
    <svg viewBox="0 0 24 24" fill="none" className={p.className} aria-hidden="true">
      <rect x="3" y="5" width="18" height="14" rx="2" stroke="currentColor" strokeWidth="1.5" />
      <circle cx="9" cy="11" r="2.2" stroke="currentColor" strokeWidth="1.5" />
      <path d="M5.5 17c.8-2 2.2-3 3.5-3s2.7 1 3.5 3M14 9h5M14 13h4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
    </svg>
  ),
  daemon: (p: IconProps) => (
    <svg viewBox="0 0 24 24" fill="none" className={p.className} aria-hidden="true">
      <path d="M12 3v3M12 18v3M3 12h3M18 12h3M5.6 5.6l2.1 2.1M16.3 16.3l2.1 2.1M5.6 18.4l2.1-2.1M16.3 7.7l2.1-2.1" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
      <circle cx="12" cy="12" r="3.5" stroke="currentColor" strokeWidth="1.5" />
    </svg>
  ),
  session: (p: IconProps) => (
    <svg viewBox="0 0 24 24" fill="none" className={p.className} aria-hidden="true">
      <rect x="3" y="4" width="18" height="16" rx="2" stroke="currentColor" strokeWidth="1.5" />
      <path d="M3 9h18M7 14l2 2 4-4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  ),
  team: (p: IconProps) => (
    <svg viewBox="0 0 24 24" fill="none" className={p.className} aria-hidden="true">
      <circle cx="9" cy="9" r="3" stroke="currentColor" strokeWidth="1.5" />
      <circle cx="17" cy="11" r="2.4" stroke="currentColor" strokeWidth="1.5" />
      <path d="M3.5 19c.8-3 3-4.5 5.5-4.5s4.7 1.5 5.5 4.5M14.5 19c.6-2 2-3 3.5-3s2.9 1 3.5 3" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
    </svg>
  ),
  dashboard: (p: IconProps) => (
    <svg viewBox="0 0 24 24" fill="none" className={p.className} aria-hidden="true">
      <rect x="3" y="4" width="18" height="16" rx="2" stroke="currentColor" strokeWidth="1.5" />
      <path d="M7 16V12M12 16V8M17 16v-6" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" />
    </svg>
  ),
  shell: (p: IconProps) => (
    <svg viewBox="0 0 24 24" fill="none" className={p.className} aria-hidden="true">
      <rect x="3" y="4" width="18" height="16" rx="2" stroke="currentColor" strokeWidth="1.5" />
      <path d="m7 10 3 2-3 2M13 14h4" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  ),
};

const features: {
  title: string;
  desc: string;
  icon: keyof typeof icons;
}[] = [
  {
    title: "Profile management",
    desc: "Separate OAuth/API accounts per project. Switch instantly with `cst use work`.",
    icon: "profile",
  },
  {
    title: "Account pipeline",
    desc: "Declare a sequence of profiles with your own usage thresholds. Advance automatically or with `cst next`.",
    icon: "daemon",
  },
  {
    title: "Session isolation",
    desc: "Each session gets its own CLAUDE_CONFIG_DIR, project history, and settings.",
    icon: "session",
  },
  {
    title: "Team sync",
    desc: "Share profile configs via a shared git remote. Onboard teammates in seconds.",
    icon: "team",
  },
  {
    title: "Live dashboard",
    desc: "`cst top` gives you an htop-style real-time usage view across all profiles.",
    icon: "dashboard",
  },
  {
    title: "Shell integration",
    desc: "Starship module, tmux segment, Zsh/Fish/Bash hooks. Fits your existing workflow.",
    icon: "shell",
  },
];

function renderDesc(text: string) {
  const parts = text.split(/(`[^`]+`)/g);
  return parts.map((part, i) =>
    part.startsWith("`") && part.endsWith("`") ? (
      <code key={i} className="code-chip">
        {part.slice(1, -1)}
      </code>
    ) : (
      <span key={i}>{part}</span>
    ),
  );
}

export function Features() {
  return (
    <section id="features" className="scroll-mt-16">
      <div className="max-w-6xl mx-auto px-5 sm:px-8 py-24 sm:py-32">
        <div className="max-w-2xl">
          <p className="eyebrow">Capabilities</p>
          <h2 className="mt-5 display text-4xl sm:text-5xl">
            Everything you need to run Claude Code at scale.
          </h2>
          <p className="mt-5 text-base text-muted leading-relaxed">
            A complete, considered toolkit for developers who switch between
            accounts all day long.
          </p>
        </div>

        <div className="mt-14 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-px bg-line border border-line rounded-2xl overflow-hidden">
          {features.map((f) => {
            const Icon = icons[f.icon];
            return (
              <article
                key={f.title}
                className="group bg-surface p-7 transition-colors duration-300 hover:bg-surface-2"
              >
                <div className="h-10 w-10 rounded-lg border border-line text-accent flex items-center justify-center">
                  <Icon className="h-5 w-5" />
                </div>
                <h3 className="mt-5 text-base font-medium text-text">
                  {f.title}
                </h3>
                <p className="mt-2 text-sm text-muted leading-relaxed">
                  {renderDesc(f.desc)}
                </p>
              </article>
            );
          })}
        </div>
      </div>
    </section>
  );
}
