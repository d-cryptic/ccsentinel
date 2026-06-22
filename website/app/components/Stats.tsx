const stats = [
  { value: "5×", label: "Faster context switching" },
  { value: "∞", label: "Pipeline stages" },
  { value: "100%", label: "Claude Code compatible" },
];

export function Stats() {
  return (
    <section className="border-y border-line">
      <div className="max-w-6xl mx-auto px-5 sm:px-8">
        <div className="grid grid-cols-1 sm:grid-cols-3 divide-y sm:divide-y-0 sm:divide-x divide-line">
          {stats.map((s) => (
            <div
              key={s.label}
              className="px-6 py-12 text-center flex flex-col items-center"
            >
              <div className="font-serif font-light text-5xl sm:text-6xl text-text leading-none">
                {s.value}
              </div>
              <div className="mt-4 text-sm text-muted">{s.label}</div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
