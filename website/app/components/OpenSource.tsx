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
    <section className="border-t border-line">
      <div className="max-w-6xl mx-auto px-5 sm:px-8 py-24 sm:py-32">
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">
          <div>
            <p className="eyebrow">Open source</p>
            <h2 className="mt-5 display text-4xl sm:text-5xl">
              MIT licensed, community-driven.
            </h2>
            <p className="mt-5 text-base text-muted leading-relaxed max-w-lg">
              Claude Sentinel is fully open source. Audit the code, file an
              issue, or send a pull request — it&rsquo;s built in the open.
            </p>

            <div className="mt-8 flex flex-wrap items-center gap-5">
              <a
                href={REPO}
                target="_blank"
                rel="noopener noreferrer"
                className="btn-ghost"
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
                <span className="inline-flex items-baseline gap-2 text-muted">
                  <span className="font-serif font-light text-2xl text-text">
                    {stars.toLocaleString()}
                  </span>
                  <span className="text-sm">stars</span>
                </span>
              )}
            </div>
          </div>

          <div className="card p-6 sm:p-7">
            <div className="flex items-center gap-3 pb-4 border-b border-line">
              <div className="h-10 w-10 rounded-lg border border-line text-accent flex items-center justify-center font-mono text-sm">
                d/
              </div>
              <div>
                <div className="text-sm text-text font-medium">
                  d-cryptic/ccsentinel
                </div>
                <div className="text-[12px] text-faint mt-0.5">
                  Public · MIT · Rust
                </div>
              </div>
            </div>

            <div className="mt-5 grid grid-cols-3 gap-3 text-center">
              {[
                { v: stars.toLocaleString(), l: "Stars" },
                { v: forks.toLocaleString(), l: "Forks" },
                { v: "238", l: "Tests" },
              ].map((m) => (
                <div
                  key={m.l}
                  className="rounded-xl border border-line bg-ink/40 py-4"
                >
                  <div className="font-serif font-light text-2xl text-text leading-none">
                    {m.v}
                  </div>
                  <div className="text-[11px] text-faint mt-1.5">{m.l}</div>
                </div>
              ))}
            </div>

            <div className="mt-5 font-mono text-[12.5px] space-y-1.5">
              {latestCommit && (
                <div className="flex justify-between">
                  <span className="text-faint">updated</span>
                  <span className="text-muted">{latestCommit}</span>
                </div>
              )}
              <div className="flex justify-between">
                <span className="text-faint">build</span>
                <span className="text-accent">passing</span>
              </div>
              <div className="flex justify-between">
                <span className="text-faint">tests</span>
                <span className="text-accent">238 passing</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
