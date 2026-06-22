import { createServer } from "node:http";
import { readFile } from "node:fs/promises";
import { join, extname } from "node:path";
import { chromium } from "playwright";

const ROOT = join(process.cwd(), "out");
const OUT = process.argv[2] || join(process.cwd(), "shots");
const PORT = 4317;

const MIME = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".css": "text/css",
  ".svg": "image/svg+xml",
  ".woff2": "font/woff2",
  ".json": "application/json",
  ".ico": "image/x-icon",
  ".png": "image/png",
};

const server = createServer(async (req, res) => {
  try {
    let p = decodeURIComponent(req.url.split("?")[0]);
    if (p === "/") p = "/index.html";
    let file = join(ROOT, p);
    if (!extname(file)) file = join(ROOT, p, "index.html");
    const data = await readFile(file);
    res.writeHead(200, { "Content-Type": MIME[extname(file)] || "application/octet-stream" });
    res.end(data);
  } catch {
    res.writeHead(404);
    res.end("404");
  }
});

await new Promise((r) => server.listen(PORT, r));

const browser = await chromium.launch();
const { mkdir } = await import("node:fs/promises");
await mkdir(OUT, { recursive: true });

// Desktop full page
const desktop = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 2 });
await desktop.goto(`http://localhost:${PORT}/`, { waitUntil: "networkidle" });
await desktop.waitForTimeout(800);
await desktop.screenshot({ path: join(OUT, "web-desktop-full.png"), fullPage: true });
await desktop.screenshot({ path: join(OUT, "web-desktop-hero.png"), fullPage: false });

// Mobile full page
const mobile = await browser.newPage({ viewport: { width: 390, height: 844 }, deviceScaleFactor: 2 });
await mobile.goto(`http://localhost:${PORT}/`, { waitUntil: "networkidle" });
await mobile.waitForTimeout(800);
await mobile.screenshot({ path: join(OUT, "web-mobile-full.png"), fullPage: true });

await browser.close();
server.close();
console.log("screenshots written to", OUT);
