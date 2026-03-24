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
    rollupOptions: {
      onwarn(warning, warn) {
        // google-protobuf uses eval() internally for transpilation detection;
        // this is dead code in a bundled environment and can be safely ignored.
        if (warning.code === "EVAL" && warning.id?.includes("google-protobuf")) return;
        warn(warning);
      },
      output: {
        manualChunks(id) {
          if (
            id.includes("node_modules/react/") ||
            id.includes("node_modules/react-dom/") ||
            id.includes("node_modules/react-router")
          ) {
            return "vendor-react";
          }
          if (id.includes("node_modules/@ant-design/icons")) {
            return "vendor-antd-icons";
          }
          if (
            id.includes("node_modules/antd/") ||
            id.includes("node_modules/@ant-design/")
          ) {
            return "vendor-antd";
          }
          if (
            id.includes("node_modules/google-protobuf") ||
            id.includes("node_modules/grpc-web")
          ) {
            return "vendor-grpc";
          }
          if (id.includes("node_modules/")) {
            return "vendor-misc";
          }
        },
      },
    },
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
