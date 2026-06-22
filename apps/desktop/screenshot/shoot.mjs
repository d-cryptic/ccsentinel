import { createServer } from "node:http";
import { readFile, mkdir } from "node:fs/promises";
import { join, extname, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, "..", "dist-screenshot");
const OUT = process.argv[2] || join(__dirname, "..", "shots");
const PORT = 4319;

const MIME = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".css": "text/css",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".json": "application/json",
};

const server = createServer(async (req, res) => {
  try {
    let p = decodeURIComponent(req.url.split("?")[0]);
    if (p === "/") p = "/index.html";
    const file = join(ROOT, p);
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
await mkdir(OUT, { recursive: true });

// Desktop window size (Tauri default-ish)
const page = await browser.newPage({ viewport: { width: 1040, height: 720 }, deviceScaleFactor: 2 });
await page.goto(`http://localhost:${PORT}/`, { waitUntil: "networkidle" });
await page.waitForTimeout(600);

const tabs = ["Profiles", "Sessions", "Auto-Switch", "Stats"];
for (const label of tabs) {
  await page.getByRole("tab", { name: label }).click();
  await page.waitForTimeout(500);
  await page.screenshot({ path: join(OUT, `desktop-${label.toLowerCase().replace(/[^a-z]/g, "-")}.png`) });
}

await browser.close();
server.close();
console.log("desktop screenshots written to", OUT);
