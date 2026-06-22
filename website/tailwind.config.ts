import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        // Cool near-black canvas + layered surfaces
        ink: "#0B0E12",
        surface: "#11151B",
        "surface-2": "#161B22",
        line: "#222933",
        // Text scale
        text: "#ECEFF3",
        muted: "#9AA3AE",
        faint: "#6A727D",
        // Single restrained accent — muted teal
        accent: "#5EC4B6",
        "accent-dim": "#3E8C82",
      },
      fontFamily: {
        sans: ["var(--font-sans)", "system-ui", "sans-serif"],
        serif: ["var(--font-serif)", "Georgia", "serif"],
        mono: ["var(--font-mono)", "ui-monospace", "monospace"],
      },
      letterSpacing: {
        widest: "0.22em",
      },
      maxWidth: {
        prose: "42rem",
      },
      animation: {
        rise: "rise 0.7s cubic-bezier(0.22, 1, 0.36, 1) both",
        "fade-in": "fadeIn 0.9s ease-out both",
      },
      keyframes: {
        rise: {
          from: { transform: "translateY(16px)", opacity: "0" },
          to: { transform: "translateY(0)", opacity: "1" },
        },
        fadeIn: {
          from: { opacity: "0" },
          to: { opacity: "1" },
        },
      },
    },
  },
  plugins: [],
};

export default config;
