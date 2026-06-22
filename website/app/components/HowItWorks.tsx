const steps = [
  {
    n: "01",
    title: "Install",
    desc: "Get the binary and initialize the data directory.",
    cmd: "cargo install cst && cst init",
  },
  {
    n: "02",
    title: "Add profiles",
    desc: "Create profiles for each account. Mix OAuth and API keys freely.",
    cmd: "cst new work --auth oauth",
  },
  {
    n: "03",
    title: "Build your pipeline",
    desc: "Declare thresholds. The daemon advances to the next account automatically — or run `cst next` yourself.",
    cmd: "cst pipeline configure work",
  },
];

export function HowItWorks() {
  return (
    <section id="how" className="scroll-mt-16 border-t border-line">
      <div className="max-w-6xl mx-auto px-5 sm:px-8 py-24 sm:py-32">
        <div className="max-w-2xl">
          <p className="eyebrow">Workflow</p>
          <h2 className="mt-5 display text-4xl sm:text-5xl">How it works</h2>
          <p className="mt-5 text-base text-muted leading-relaxed">
            From zero to full multi-account isolation in under a minute.
          </p>
        </div>

        <ol className="mt-14 grid grid-cols-1 lg:grid-cols-3 gap-6">
          {steps.map((s) => (
            <li key={s.n} className="card p-7 flex flex-col">
              <div className="font-serif font-light text-3xl text-accent leading-none">
                {s.n}
              </div>
              <h3 className="mt-5 text-lg font-medium text-text">{s.title}</h3>
              <p className="mt-2 text-sm text-muted leading-relaxed flex-1">
                {s.desc}
              </p>
              <pre className="mt-6 rounded-lg border border-line bg-ink/50 p-3 text-[12.5px] font-mono overflow-x-auto">
                <code>
                  <span className="text-faint">$</span>{" "}
                  <span className="text-accent">{s.cmd}</span>
                </code>
              </pre>
            </li>
          ))}
        </ol>
      </div>
    </section>
  );
}
