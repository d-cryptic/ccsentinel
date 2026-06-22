"use client";

import { useEffect, useState } from "react";
import { ShieldIcon } from "./ShieldIcon";

const REPO = "https://github.com/d-cryptic/ccsentinel";

const links = [
  { label: "Features", href: "#features" },
  { label: "How it works", href: "#how" },
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
          ? "bg-ink/85 backdrop-blur-md border-b border-line"
          : "bg-transparent border-b border-transparent"
      }`}
    >
      <div className="max-w-6xl mx-auto px-5 sm:px-8 h-16 flex items-center justify-between">
        <a href="#" className="flex items-center gap-2.5">
          <ShieldIcon className="h-5 w-5 text-accent" />
          <span className="font-serif text-lg tracking-tight text-text">
            Claude Sentinel
          </span>
        </a>

        <div className="hidden md:flex items-center gap-8">
          {links.map((l) => (
            <a
              key={l.label}
              href={l.href}
              target={l.href.startsWith("http") ? "_blank" : undefined}
              rel={l.href.startsWith("http") ? "noopener noreferrer" : undefined}
              className="text-sm text-muted hover:text-text transition-colors"
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
            className="btn-primary"
          >
            Get started
          </a>
        </div>

        <button
          aria-label="Toggle menu"
          aria-expanded={open}
          onClick={() => setOpen((v) => !v)}
          className="md:hidden p-2 text-muted hover:text-text transition-colors"
        >
          <svg
            width="22"
            height="22"
            viewBox="0 0 24 24"
            fill="none"
            aria-hidden="true"
          >
            <path
              d={open ? "M6 6l12 12M6 18L18 6" : "M4 8h16M4 16h16"}
              stroke="currentColor"
              strokeWidth="1.6"
              strokeLinecap="round"
            />
          </svg>
        </button>
      </div>

      {open && (
        <div className="md:hidden border-t border-line bg-ink">
          <div className="px-5 py-5 flex flex-col gap-4">
            {links.map((l) => (
              <a
                key={l.label}
                href={l.href}
                target={l.href.startsWith("http") ? "_blank" : undefined}
                rel={
                  l.href.startsWith("http") ? "noopener noreferrer" : undefined
                }
                className="text-sm text-muted hover:text-text transition-colors"
                onClick={() => setOpen(false)}
              >
                {l.label}
              </a>
            ))}
            <a
              href={REPO}
              target="_blank"
              rel="noopener noreferrer"
              className="btn-primary mt-1 w-full"
            >
              Get started
            </a>
          </div>
        </div>
      )}
    </nav>
  );
}
