import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  root: "./",
  base: "/",
  publicDir: "public",
  build: {
    outDir: "dist",
    assetsDir: "assets",
  },
  resolve: {
    // This is require in order to have grpc-web-client-gen working
    preserveSymlinks: true,
  },
  server: {
    port: 5174,
    // host: "0.0.0.0",
    // Enable if you need to access from LAN: host: true
    proxy: {
      // Forward REST-ish UI config calls to the Rocket backend
      // Example: http://localhost:5174/api/uiconfig -> http://localhost:8081/api/uiconfig
      "/api": {
        target: process.env.VITE_BACKEND_URL || "http://localhost:8081",
        changeOrigin: true,
        // If backend were mounted under a sub-path, you could rewrite here.
        // rewrite: (path) => path.replace(/^\/api/, "")
      },
    },
  },
});
