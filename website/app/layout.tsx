import type { Metadata, Viewport } from "next";
import { Inter, Newsreader, JetBrains_Mono } from "next/font/google";
import "./globals.css";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-sans",
  display: "swap",
});

const newsreader = Newsreader({
  subsets: ["latin"],
  weight: ["300", "400", "500"],
  style: ["normal", "italic"],
  variable: "--font-serif",
  display: "swap",
});

const jetbrainsMono = JetBrains_Mono({
  subsets: ["latin"],
  weight: ["400", "500"],
  variable: "--font-mono",
  display: "swap",
});

export const metadata: Metadata = {
  title: "Claude Sentinel — Intelligent Claude Code Account Manager",
  description:
    "Switch between Claude accounts instantly. Manage profiles, sessions, and API keys for Claude Code.",
  keywords: [
    "claude",
    "claude code",
    "account manager",
    "profile switching",
    "api key rotation",
    "developer tools",
    "rust",
    "cli",
  ],
  icons: {
    icon: [{ url: "/icon.svg", type: "image/svg+xml" }],
    shortcut: "/icon.svg",
  },
  openGraph: {
    title: "Claude Sentinel — Switch Claude accounts instantly",
    description:
      "Manage all your Claude accounts, profiles, and sessions in one place.",
    type: "website",
  },
};

export const viewport: Viewport = {
  themeColor: "#0B0E12",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html
      lang="en"
      className={`${inter.variable} ${newsreader.variable} ${jetbrainsMono.variable}`}
    >
      <body className="bg-ink text-text font-sans antialiased selection:bg-accent/25 selection:text-text">
        {children}
      </body>
    </html>
  );
}
