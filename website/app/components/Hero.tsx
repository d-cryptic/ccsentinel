import { CopyCommand } from "./CopyCommand";
import { TerminalCard } from "./TerminalCard";

const REPO = "https://github.com/d-cryptic/ccsentinel";

export function Hero() {
  return (
    <section className="relative overflow-hidden -mt-16 pt-16">
      {/* Faint dotted backdrop + soft radial fade */}
      <div className="absolute inset-0 dot-grid opacity-60" aria-hidden="true" />
      <div
        className="absolute inset-x-0 top-0 h-[40rem] bg-[radial-gradient(ellipse_60%_50%_at_50%_0%,rgba(94,196,182,0.08),transparent_70%)]"
        aria-hidden="true"
      />

      <div className="relative z-10 max-w-6xl mx-auto px-5 sm:px-8 py-20 sm:py-28 w-full">
        <div className="text-center max-w-3xl mx-auto animate-rise">
          <p className="eyebrow justify-center">Intelligent account manager</p>

          <h1 className="mt-7 display text-5xl sm:text-6xl md:text-7xl">
            Switch Claude accounts
            <br />
            <span className="italic text-accent">instantly.</span>
          </h1>

          <p className="mt-7 text-base sm:text-lg text-muted max-w-xl mx-auto leading-relaxed">
            Manage every Claude Code account, profile, and session in one place.
            Build a pipeline between accounts — switch context with a single
            command, or let the daemon advance automatically.
          </p>

          <div className="mt-9 flex flex-col sm:flex-row items-stretch sm:items-center justify-center gap-3">
            <a
              href={REPO}
              target="_blank"
              rel="noopener noreferrer"
              className="btn-primary"
            >
              Install now
            </a>
            <a
              href={REPO}
              target="_blank"
              rel="noopener noreferrer"
              className="btn-ghost"
            >
              View on GitHub
            </a>
          </div>

          <div className="mt-7 flex justify-center">
            <CopyCommand command="cargo install cst" />
          </div>
        </div>

        <div className="mt-16 sm:mt-20">
          <TerminalCard />
        </div>
      </div>
    </section>
  );
}
