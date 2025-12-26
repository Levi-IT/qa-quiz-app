import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite'; // Import Tailwind v4

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    tailwindcss(), // Kích hoạt Tailwind v4 ở đây
    sveltekit()
  ],
  // Các cấu hình Tauri giữ nguyên (nếu có)
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: true || false,
    hmr: true ? {
      protocol: 'ws',
      host: 'localhost',
      port: 1421,
    } : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});