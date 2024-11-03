import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      "/hls": {
        target: "http://localhost:8000",
        changeOrigin: true,
      },
    },
  },
})
