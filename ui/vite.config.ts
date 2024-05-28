import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  root: "./",
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
    // host: "0.0.0.0",
    // cors: true,
  },
});
