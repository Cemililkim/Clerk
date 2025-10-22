#!/usr/bin/env node
import sharp from 'sharp';
import fs from 'fs';
import path from 'path';
import pngToIco from 'png-to-ico';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

try {
  const svgPath = path.resolve(__dirname, '..', 'icons', 'clerkIcon.svg');
  const outDir = path.resolve(__dirname, '..', 'icons', 'win');
  if (!fs.existsSync(outDir)) fs.mkdirSync(outDir, { recursive: true });

  const sizes = [16, 32, 48, 64, 128, 256, 512];

  for (const s of sizes) {
    const outPath = path.join(outDir, `icon-${s}.png`);
    await sharp(svgPath)
      .resize(s, s)
      .png()
      .toFile(outPath);
    console.log(`Written ${outPath}`);
  }

  // Create icon.ico from a subset of sizes
  const icoPath = path.join(outDir, 'icon.ico');
  // Use common icon sizes for ICO containers (16,32,48,256)
  const icoPngs = [
    path.join(outDir, 'icon-16.png'),
    path.join(outDir, 'icon-32.png'),
    path.join(outDir, 'icon-48.png'),
    path.join(outDir, 'icon-256.png')
  ].filter(p => fs.existsSync(p));

  if (icoPngs.length > 0) {
    const buffer = await pngToIco(icoPngs);
    fs.writeFileSync(icoPath, buffer);
    console.log(`Written ${icoPath}`);
  } else {
    console.warn('No PNGs available to create ICO');
  }

  console.log('Windows icon generation completed.');
} catch (err) {
  console.error('Icon generation failed:', err);
  process.exit(1);
}
