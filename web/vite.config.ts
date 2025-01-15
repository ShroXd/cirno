import react from '@vitejs/plugin-react-swc'
import path from 'path'
import { visualizer } from 'rollup-plugin-visualizer'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig(() => {
  const devProxy = {
    target: process.env.VITE_API_URL || '/',
    changeOrigin: true,
  }

  return {
    resolve: {
      alias: {
        '~': path.resolve(__dirname, 'src'),
      },
    },
    plugins: [
      react(),
      visualizer({
        open: !process.env.CI,
        filename: 'dist/stats.html',
        gzipSize: true,
        brotliSize: true,
      }),
    ],
    server: {
      host: true,
      port: 5173,
      proxy: {
        '/hls': {
          ...devProxy,
          proxyTimeout: 30000,
        },
        '/library': devProxy,
        '/video-player': devProxy,
        '/ws': {
          ...devProxy,
          ws: true,
        },
      },
    },
    test: {
      silent: true,
      environment: 'jsdom',
      setupFiles: [path.resolve(__dirname, 'vitest-setup.ts')],
      coverage: {
        provider: 'v8',
        reporter: ['html', 'text', 'json'],
        exclude: [
          '**/node_modules/**',
          '**/test/**',
          '**/*.d.ts',
          '**/*.{test,config}.{js,ts}',
          '**/App.tsx',
          '**/i18n.ts',
          '**/main.tsx',
          '**/types.ts',
          '**/bindings/**',
        ],
      },
    },
    build: {
      outDir: 'dist',
      sourcemap: false,
      chunkSizeWarningLimit: 1500,
      rollupOptions: {
        output: {
          manualChunks(id) {
            if (id.includes('react') || id.includes('react-dom')) {
              return 'react-vendor'
            }
            if (id.includes('video.js')) {
              return 'video-vendor'
            }
            // TODO: split business logic into chunks if needed
          },
        },
      },
      minify: 'terser',
      terserOptions: {
        compress: {
          drop_console: true,
          drop_debugger: true,
        },
        format: {
          comments: false,
        },
      },
    },
    optimizeDeps: {
      include: ['react', 'react-dom'],
    },
  }
})
