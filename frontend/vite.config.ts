import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import { readFileSync } from 'fs'

export default defineConfig({
  plugins: [
    vue(),
    // Serve converter.json from project root as /converter.json
    {
      name: 'serve-converter-config',
      configureServer(server) {
        server.middlewares.use('/converter.json', (_req, res) => {
          const configPath = resolve(__dirname, '../converter.json')
          try {
            const content = readFileSync(configPath, 'utf-8')
            res.setHeader('Content-Type', 'application/json')
            res.end(content)
          } catch {
            res.statusCode = 404
            res.end('Not found')
          }
        })
      },
    },
  ],
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      '@assets': resolve(__dirname, '../ui-pack-rpg/PNG'),
    },
  },
  server: {
    fs: {
      // Allow access to core frontend package and parent directories
      allow: [resolve(__dirname, '..'), resolve(__dirname, '../../xiangxue-core/frontend')],
    },
  },
})
