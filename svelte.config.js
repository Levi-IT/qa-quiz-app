import adapter from '@sveltejs/adapter-static'; // Đổi từ adapter-auto sang adapter-static
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      pages: 'build',  // Thư mục đầu ra phải khớp với "frontendDist" trong tauri.conf.json
      assets: 'build',
      fallback: 'index.html', // Quan trọng cho chế độ SPA (Single Page App)
      precompress: false,
      strict: false
    })
  }
};

export default config;