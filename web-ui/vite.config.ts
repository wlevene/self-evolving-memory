import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/memories': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/stats': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/health': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/links': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/explicit': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
      '/implicit': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
    },
  },
})