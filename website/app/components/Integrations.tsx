const tools = [
  "Claude Code",
  "Zsh",
  "Fish",
  "Bash",
  "Tmux",
  "Starship",
  "Alfred",
  "Raycast",
];

export function Integrations() {
  return (
    <section className="border-t border-line">
      <div className="max-w-6xl mx-auto px-5 sm:px-8 py-24 sm:py-28">
        <div className="text-center max-w-2xl mx-auto">
          <p className="eyebrow justify-center">Ecosystem</p>
          <h2 className="mt-5 display text-4xl sm:text-5xl">
            Fits the tools you already use.
          </h2>
          <p className="mt-5 text-base text-muted leading-relaxed">
            Drop-in integrations for the shells, multiplexers, and launchers in
            your everyday workflow.
          </p>
        </div>

        <div className="mt-12 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
          {tools.map((t) => (
            <div
              key={t}
              className="inline-flex items-center justify-center gap-2.5 rounded-xl border border-line bg-surface px-5 py-4 text-sm text-muted hover:text-text hover:border-accent/30 transition-colors"
            >
              <span
                className="h-1.5 w-1.5 rounded-full bg-accent"
                aria-hidden="true"
              />
              {t}
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
