const REPO = "https://github.com/d-cryptic/ccsentinel";
const API = "https://api.github.com/repos/d-cryptic/ccsentinel";

async function getGitHubStats() {
  try {
    const res = await fetch(API, {
      headers: { Accept: "application/vnd.github+json" },
      cache: "force-cache",
    });
    if (!res.ok) return null;
    const data = await res.json();
    return {
      stars: data.stargazers_count as number,
      forks: data.forks_count as number,
      latestCommit: (data.pushed_at as string | null)
        ? new Date(data.pushed_at).toLocaleDateString("en-US", {
            month: "short",
            day: "numeric",
            year: "numeric",
          })
        : null,
    };
  } catch {
    return null;
  }
}

export async function OpenSource() {
  const stats = await getGitHubStats();
  const stars = stats?.stars ?? 0;
  const forks = stats?.forks ?? 0;
  const latestCommit = stats?.latestCommit ?? null;

  return (
    <section className="bg-bg border-y-2 border-yellow/30 relative overflow-hidden">
      <div className="absolute inset-0 hero-grid opacity-30" aria-hidden="true" />
      <div className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 sm:py-28">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <div>
            <p className="eyebrow text-cyan">&gt; OPEN SOURCE_</p>
            <h2 className="mt-4 font-vt text-5xl sm:text-6xl lg:text-7xl uppercase text-yellow glow-yellow leading-none">
              {"// OPEN SOURCE"}
            </h2>
            <p className="mt-5 font-mono text-sm sm:text-base text-gray-400 leading-relaxed max-w-lg">
              Claude Sentinel is{" "}
              <span className="text-neongreen font-bold">MIT licensed</span> and
              community-driven. Audit the code, file an issue, or send a pull
              request.
            </p>

            <div className="mt-7 flex flex-wrap items-center gap-4">
              <a
                href={REPO}
                target="_blank"
                rel="noopener noreferrer"
                className="btn-retro"
              >
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="currentColor"
                  aria-hidden="true"
                >
                  <path d="M12 .3a12 12 0 0 0-3.8 23.4c.6.1.8-.3.8-.6v-2c-3.3.7-4-1.6-4-1.6-.6-1.4-1.4-1.8-1.4-1.8-1.1-.8.1-.8.1-.8 1.2.1 1.9 1.3 1.9 1.3 1.1 1.9 2.9 1.4 3.6 1 .1-.8.4-1.4.8-1.7-2.7-.3-5.5-1.3-5.5-6 0-1.3.5-2.4 1.3-3.2-.1-.4-.6-1.6.1-3.3 0 0 1-.3 3.3 1.2a11.4 11.4 0 0 1 6 0c2.3-1.5 3.3-1.2 3.3-1.2.7 1.7.2 2.9.1 3.3.8.8 1.3 1.9 1.3 3.2 0 4.7-2.8 5.7-5.5 6 .4.4.8 1.1.8 2.2v3.3c0 .3.2.7.8.6A12 12 0 0 0 12 .3" />
                </svg>
                Star on GitHub
              </a>
              {stars > 0 && (
                <span className="inline-flex items-center gap-2 font-mono text-sm text-dim">
                  <span className="font-vt text-3xl text-magenta glow-magenta">
                    &#9733; {stars.toLocaleString()}
                  </span>
                  <span className="uppercase tracking-widest text-[11px]">
                    stars
                  </span>
                </span>
              )}
            </div>
          </div>

          <div className="relative">
            <div className="bg-black border-2 border-cyan box-glow-cyan p-6 sm:p-7 crt">
              <div className="flex items-center gap-3 pb-3 border-b border-cyan/30">
                <div className="h-10 w-10 border-2 border-magenta text-magenta flex items-center justify-center font-vt text-xl">
                  d/
                </div>
                <div>
                  <div className="font-mono text-sm text-cyan glow-cyan">
                    d-cryptic/ccsentinel
                  </div>
                  <div className="font-mono text-[11px] text-dim uppercase tracking-widest mt-0.5">
                    Public &middot; MIT &middot; Rust
                  </div>
                </div>
              </div>

              <div className="mt-5 grid grid-cols-3 gap-3 text-center">
                {[
                  { v: stars.toLocaleString(), l: "STARS" },
                  { v: forks.toLocaleString(), l: "FORKS" },
                  { v: "238", l: "TESTS" },
                ].map((m) => (
                  <div
                    key={m.l}
                    className="border border-magenta/50 bg-card py-3"
                  >
                    <div className="font-vt text-3xl text-magenta glow-magenta leading-none">
                      {m.v}
                    </div>
                    <div className="text-[10px] uppercase tracking-widest text-dim mt-1 font-mono">
                      {m.l}
                    </div>
                  </div>
                ))}
              </div>

              <div className="mt-5 font-mono text-[12px] space-y-1">
                {latestCommit && (
                  <div className="text-dim">
                    <span className="text-yellow">updated:</span>{" "}
                    <span className="text-gray-300">{latestCommit}</span>
                  </div>
                )}
                <div className="text-dim">
                  <span className="text-yellow">build:</span>{" "}
                  <span className="text-neongreen phosphor">passing</span>
                </div>
                <div className="text-dim">
                  <span className="text-yellow">tests:</span>{" "}
                  <span className="text-neongreen phosphor">238 passing</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
