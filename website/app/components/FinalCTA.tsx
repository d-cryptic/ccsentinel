const REPO = "https://github.com/d-cryptic/ccsentinel";

export function FinalCTA() {
  return (
    <section className="bg-black relative overflow-hidden border-t-2 border-magenta/40">
      <div
        aria-hidden="true"
        className="absolute inset-0 perspective-grid opacity-40"
      />
      <div
        aria-hidden="true"
        className="absolute inset-0 horizon-glow"
      />
      <div className="relative max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-24 sm:py-32 text-center">
        <p className="eyebrow text-cyan">&gt; GET STARTED_</p>
        <h2 className="mt-6 font-vt text-5xl sm:text-7xl lg:text-8xl tracking-tight leading-[0.95] uppercase text-white glow-magenta">
          Start managing Claude
          <br />
          <span className="text-magenta glow-magenta">like a pro</span>
        </h2>
        <p className="mt-6 font-mono text-sm sm:text-base text-gray-400 max-w-xl mx-auto leading-relaxed">
          Open source, written in Rust, fast as the terminals you already love.
        </p>
        <div className="mt-10 flex justify-center">
          <a
            href={REPO}
            target="_blank"
            rel="noopener noreferrer"
            className="btn-retro-magenta"
          >
            Get Claude Sentinel &gt;
          </a>
        </div>
      </div>
    </section>
  );
}
