export function AnnouncementBanner() {
  return (
    <div className="bg-surface border-b border-line py-2 px-4 text-center">
      <a
        href="https://github.com/d-cryptic/ccsentinel"
        target="_blank"
        rel="noopener noreferrer"
        className="inline-flex items-center gap-2.5 text-xs text-muted hover:text-text transition-colors"
      >
        <span className="inline-flex items-center gap-1.5 rounded-full border border-accent/30 bg-accent/10 px-2 py-0.5 text-[10px] font-medium uppercase tracking-widest text-accent">
          New
        </span>
        <span className="hidden sm:inline">
          Account Pipeline is live — star the project on GitHub
        </span>
        <span className="sm:hidden">Star on GitHub</span>
        <span aria-hidden="true" className="text-faint">
          &rarr;
        </span>
      </a>
    </div>
  );
}
