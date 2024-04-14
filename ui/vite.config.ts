import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    // This is require in order to have grpc-web-client-gen working
    preserveSymlinks: true,
  }
})
