import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import type { UserConfig } from "vite";

const host = process.env["TAURI_DEV_HOST"];

const server: NonNullable<UserConfig["server"]> = {
  // Tauri expects a fixed port; fail if it is unavailable.
  port: 1420,
  strictPort: true,
  host: host ?? false,
  watch: {
    // Ignore the Rust crates while the frontend dev server watches.
    ignored: ["**/crates/**"],
  },
};
if (host) {
  server.hmr = { protocol: "ws", host, port: 1421 };
}

// Vite options tailored for Tauri development and only applied in `tauri dev`
// and `tauri build`.
export default defineConfig({
  plugins: [react()],
  // Prevent Vite from obscuring rust errors.
  clearScreen: false,
  server,
});
