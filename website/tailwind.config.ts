import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        bg: "#080808",
        card: "#111111",
        cyan: "#00FFFF",
        neongreen: "#39FF14",
        magenta: "#FF00FF",
        yellow: "#FFE600",
        blue: "#0066FF",
        dim: "#888888",
        border: "#222222",
      },
      fontFamily: {
        vt: ["var(--font-vt)", "monospace"],
        mono: ["var(--font-mono)", "monospace"],
        ibm: ["var(--font-mono)", "monospace"],
      },
      animation: {
        blink: "blink 1s step-end infinite",
        flicker: "flicker 0.15s infinite",
        "slide-up": "slideUp 0.6s ease-out forwards",
        "fade-in": "fadeIn 0.8s ease-out forwards",
        float: "float 3s ease-in-out infinite",
      },
      keyframes: {
        blink: {
          "0%, 100%": { opacity: "1" },
          "50%": { opacity: "0" },
        },
        flicker: {
          "0%, 100%": { opacity: "1" },
          "50%": { opacity: "0.85" },
        },
        slideUp: {
          from: { transform: "translateY(30px)", opacity: "0" },
          to: { transform: "translateY(0)", opacity: "1" },
        },
        fadeIn: {
          from: { opacity: "0" },
          to: { opacity: "1" },
        },
        float: {
          "0%, 100%": { transform: "translateY(0)" },
          "50%": { transform: "translateY(-10px)" },
        },
      },
    },
  },
  plugins: [],
};

export default config;
