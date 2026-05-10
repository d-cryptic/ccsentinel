import type { Metadata, Viewport } from "next";
import { VT323, Space_Mono } from "next/font/google";
import "./globals.css";

const vt323 = VT323({
  weight: "400",
  subsets: ["latin"],
  variable: "--font-vt",
  display: "swap",
});

const spaceMono = Space_Mono({
  weight: ["400", "700"],
  subsets: ["latin"],
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
  themeColor: "#080808",
};

export default function RootLayout({
  children,
}: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en" className={`${vt323.variable} ${spaceMono.variable}`}>
      <body className="bg-bg text-gray-200 font-mono antialiased">
        {children}
      </body>
    </html>
  );
}
