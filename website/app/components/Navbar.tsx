"use client";

import { useEffect, useState } from "react";

const REPO = "https://github.com/d-cryptic/ccsentinel";

const links = [
  { label: "Features", href: "#features" },
  { label: "How It Works", href: "#how" },
  { label: "Docs", href: `${REPO}#readme` },
  { label: "GitHub", href: REPO },
];

export function Navbar() {
  const [scrolled, setScrolled] = useState(false);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 8);
    onScroll();
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  return (
    <nav
      className={`sticky top-0 z-40 transition-all duration-300 ${
        scrolled
          ? "bg-bg/95 backdrop-blur-md border-b border-border"
          : "bg-bg/70 border-b border-transparent"
      }`}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <a href="#" className="flex items-center gap-2">
          <span className="font-vt text-3xl text-cyan glow-cyan leading-none">
            [CST]
          </span>
          <span className="hidden sm:inline font-mono text-xs uppercase tracking-widest text-dim">
            claude_sentinel
          </span>
        </a>

        <div className="hidden md:flex items-center gap-7">
          {links.map((l) => (
            <a
              key={l.label}
              href={l.href}
              target={l.href.startsWith("http") ? "_blank" : undefined}
              rel={l.href.startsWith("http") ? "noopener noreferrer" : undefined}
              className="font-mono text-xs uppercase tracking-widest text-dim hover:text-cyan hover:[text-shadow:_0_0_8px_#00FFFF] transition-all"
            >
              {l.label}
            </a>
          ))}
        </div>

        <div className="hidden md:block">
          <a
            href={REPO}
            target="_blank"
            rel="noopener noreferrer"
            className="btn-retro"
          >
            Get Started
            <span aria-hidden="true">&gt;</span>
          </a>
        </div>

        <button
          aria-label="Toggle menu"
          aria-expanded={open}
          onClick={() => setOpen((v) => !v)}
          className="md:hidden p-2 text-cyan border border-cyan"
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            aria-hidden="true"
          >
            <path
              d={open ? "M6 6l12 12M6 18L18 6" : "M4 7h16M4 12h16M4 17h16"}
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
            />
          </svg>
        </button>
      </div>

      {open && (
        <div className="md:hidden border-t border-border bg-bg">
          <div className="px-4 py-4 flex flex-col gap-4">
            {links.map((l) => (
              <a
                key={l.label}
                href={l.href}
                target={l.href.startsWith("http") ? "_blank" : undefined}
                rel={
                  l.href.startsWith("http") ? "noopener noreferrer" : undefined
                }
                className="font-mono text-xs uppercase tracking-widest text-dim hover:text-cyan"
                onClick={() => setOpen(false)}
              >
                &gt; {l.label}
              </a>
            ))}
            <a
              href={REPO}
              target="_blank"
              rel="noopener noreferrer"
              className="btn-retro-fill mt-2 w-full"
            >
              Get Started &gt;
            </a>
          </div>
        </div>
      )}
    </nav>
  );
}
