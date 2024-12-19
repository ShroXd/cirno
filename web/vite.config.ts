import { defineConfig } from 'vite'
import { visualizer } from 'rollup-plugin-visualizer'
import react from '@vitejs/plugin-react-swc'
import path from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      '@bindings': path.resolve(__dirname, 'src/bindings'),
    },
  },
  plugins: [
    react(),
    visualizer({
      open: true,
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
        target: 'http://localhost:8000',
        changeOrigin: true,
        proxyTimeout: 30000,
      },
      '/library': {
        target: 'http://localhost:8000',
        changeOrigin: true,
      },
      '/ws': {
        target: 'ws://localhost:8080',
        changeOrigin: true,
        ws: true,
      },
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
})
