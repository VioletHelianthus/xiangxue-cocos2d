/**
 * SSR render script: renders .vue files to HTML.
 *
 * Usage:
 *   node scripts/render.js <input.vue>                      # single file → stdout
 *   node scripts/render.js <a.vue> <b.vue> -o <outdir>      # multiple files → outdir/
 *   node scripts/render.js <dir> -o <outdir>                 # all .vue in dir → outdir/
 *
 * Requires: npm install (uses Vite SSR API internally)
 */

import { createServer } from 'vite'
import { renderToString } from 'vue/server-renderer'
import { createSSRApp } from 'vue'
import { resolve, dirname, basename, extname } from 'path'
import { fileURLToPath } from 'url'
import { writeFileSync, mkdirSync, readdirSync, statSync } from 'fs'

const __dirname = dirname(fileURLToPath(import.meta.url))

function wrapHtml(body) {
  return `<!DOCTYPE html>\n<html>\n<head><meta charset="utf-8"></head>\n<body>\n${body}\n</body>\n</html>`
}

async function main() {
  const args = process.argv.slice(2)
  const outIdx = args.indexOf('-o')
  const outDir = outIdx !== -1 ? args[outIdx + 1] : null
  const skipIndices = new Set(outIdx !== -1 ? [outIdx, outIdx + 1] : [])
  const inputs = args.filter((a, i) => !skipIndices.has(i) && !a.startsWith('-'))

  if (inputs.length === 0) {
    console.error('Usage: node scripts/render.js <input.vue|dir> [-o outdir]')
    process.exit(1)
  }

  // Expand directory inputs to .vue files
  const vueFiles = []
  for (const input of inputs) {
    const fullPath = resolve(process.cwd(), input)
    try {
      if (statSync(fullPath).isDirectory()) {
        const files = readdirSync(fullPath)
          .filter(f => f.endsWith('.vue'))
          .map(f => resolve(fullPath, f))
        vueFiles.push(...files)
      } else {
        vueFiles.push(fullPath)
      }
    } catch {
      vueFiles.push(fullPath)
    }
  }

  if (vueFiles.length === 0) {
    console.error('No .vue files found')
    process.exit(1)
  }

  // Single file without -o: output to stdout (backward compatible)
  if (vueFiles.length === 1 && !outDir) {
    const html = await renderSingle(vueFiles[0])
    process.stdout.write(wrapHtml(html))
    return
  }

  // Multiple files or -o specified: write to outDir
  if (!outDir) {
    console.error('Multiple files require -o <outdir>')
    process.exit(1)
  }

  try { mkdirSync(outDir, { recursive: true }) } catch {}

  const frontendRoot = resolve(__dirname, '..')
  const vite = await createServer({
    root: frontendRoot,
    server: { middlewareMode: true },
    appType: 'custom',
    logLevel: 'warn',
  })

  try {
    const { registerComponents } = await vite.ssrLoadModule(
      resolve(frontendRoot, 'src/components/index.ts')
    )

    for (const vuePath of vueFiles) {
      const mod = await vite.ssrLoadModule(vuePath)
      const Component = mod.default
      const app = createSSRApp(Component)
      registerComponents(app)
      const html = await renderToString(app)

      const outName = basename(vuePath, extname(vuePath)) + '.html'
      const outPath = resolve(outDir, outName)
      writeFileSync(outPath, wrapHtml(html))
      console.error(`  ${basename(vuePath)} → ${outName}`)
    }
  } finally {
    await vite.close()
  }
}

async function renderSingle(vuePath) {
  const frontendRoot = resolve(__dirname, '..')
  const vite = await createServer({
    root: frontendRoot,
    server: { middlewareMode: true },
    appType: 'custom',
    logLevel: 'warn',
  })

  try {
    const mod = await vite.ssrLoadModule(vuePath)
    const Component = mod.default
    const { registerComponents } = await vite.ssrLoadModule(
      resolve(frontendRoot, 'src/components/index.ts')
    )
    const app = createSSRApp(Component)
    registerComponents(app)
    return await renderToString(app)
  } finally {
    await vite.close()
  }
}

main().catch((err) => {
  console.error(err)
  process.exit(1)
})
