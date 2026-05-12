#!/usr/bin/env node
/**
 * Visual regression screenshots for the portfolio site.
 * Requires: trunk serve running on port 8080
 * Usage: NODE_PATH=$(npx --yes playwright 2>/dev/null; find ~/.npm/_npx -name playwright -type d 2>/dev/null | head -1)/.. \
 *        node scripts/screenshot.js [output-dir]
 * Quick run: scripts/screenshot.sh
 */

const { chromium } = require("playwright");
const fs = require("fs");
const path = require("path");

const BASE_URL = "http://localhost:8080";
const OUT_DIR = process.argv[2] || path.join(__dirname, "../screenshots");

const VIEWPORTS = [
  { name: "360-phone",   width: 360,  height: 780 },
  { name: "360-flip-cover", width: 360, height: 420 }, // Z Flip cover screen (short)
  { name: "480-small",   width: 480,  height: 850 },
  { name: "768-tablet",  width: 768,  height: 1024 },
  { name: "1024-tablet-l", width: 1024, height: 1366 },
  { name: "1440-desktop", width: 1440, height: 900 },
  { name: "1920-wide",   width: 1920, height: 1080 },
];

const PAGES = [
  { name: "home",    path: "/" },
  { name: "blog",    path: "/blog" },
  { name: "veille",  path: "/veille" },
];

async function shoot(page, url, file) {
  await page.goto(url, { waitUntil: "networkidle" });
  // Let Leptos/WASM hydrate
  await page.waitForTimeout(1200);
  await page.screenshot({ path: file, fullPage: true });
  console.log("  ✓", path.basename(file));
}

(async () => {
  fs.mkdirSync(OUT_DIR, { recursive: true });

  const browser = await chromium.launch({
    executablePath: "/usr/bin/google-chrome-stable",
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });

  for (const vp of VIEWPORTS) {
    const ctx = await browser.newContext({ viewport: { width: vp.width, height: vp.height } });
    const page = await ctx.newPage();
    console.log(`\n[${vp.name} — ${vp.width}×${vp.height}]`);

    for (const pg of PAGES) {
      const file = path.join(OUT_DIR, `${vp.name}__${pg.name}.png`);
      await shoot(page, BASE_URL + pg.path, file);
    }

    // Blog article — find first article link
    await page.goto(BASE_URL + "/blog", { waitUntil: "networkidle" });
    await page.waitForTimeout(800);
    const articleLink = await page.$(".blog-article-list a, .blog-card a, article a");
    if (articleLink) {
      const href = await articleLink.getAttribute("href");
      if (href) {
        const file = path.join(OUT_DIR, `${vp.name}__blog-article.png`);
        await shoot(page, BASE_URL + href, file);
      }
    }

    await ctx.close();
  }

  await browser.close();
  console.log(`\nScreenshots saved to: ${OUT_DIR}`);
})();
