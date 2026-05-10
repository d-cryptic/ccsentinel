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
    <section className="bg-[#0A0A0A] border-t border-border relative overflow-hidden">
      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 sm:py-24">
        <div className="text-center max-w-2xl mx-auto">
          <p className="eyebrow text-magenta">&gt; ECOSYSTEM_</p>
          <h2 className="mt-4 font-vt text-5xl sm:text-6xl lg:text-7xl uppercase text-cyan glow-cyan leading-none">
            {"// COMPATIBLE WITH"}
          </h2>
          <p className="mt-5 font-mono text-sm sm:text-base text-gray-400 leading-relaxed">
            Drop-in integrations for the shells, multiplexers, and launchers
            you already use.
          </p>
        </div>

        <div className="mt-12 grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
          {tools.map((t) => (
            <div
              key={t}
              className="inline-flex items-center justify-center gap-2 px-5 py-3 border-2 border-cyan/25 text-gray-300 hover:border-cyan hover:text-cyan hover:box-glow-cyan transition-all font-mono text-xs uppercase tracking-widest bg-card"
            >
              <span
                className="h-1.5 w-1.5 rounded-full bg-magenta"
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
