const REPO = "https://github.com/d-cryptic/ccsentinel";

export function FinalCTA() {
  return (
    <section className="relative overflow-hidden border-t border-line">
      <div
        aria-hidden="true"
        className="absolute inset-0 bg-[radial-gradient(ellipse_55%_60%_at_50%_100%,rgba(94,196,182,0.1),transparent_70%)]"
      />
      <div className="relative max-w-3xl mx-auto px-5 sm:px-8 py-28 sm:py-36 text-center">
        <p className="eyebrow justify-center">Get started</p>
        <h2 className="mt-6 display text-4xl sm:text-6xl">
          Manage Claude
          <br />
          <span className="italic text-accent">like a pro.</span>
        </h2>
        <p className="mt-6 text-base text-muted max-w-md mx-auto leading-relaxed">
          Open source, written in Rust, as fast as the terminal you already
          live in.
        </p>
        <div className="mt-9 flex justify-center">
          <a
            href={REPO}
            target="_blank"
            rel="noopener noreferrer"
            className="btn-primary"
          >
            Get Claude Sentinel
          </a>
        </div>
      </div>
    </section>
  );
}
