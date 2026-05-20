import { defineConfig } from '@zenvu/cli';

export default defineConfig({
  mode: 'blog',
  typescript: true,
  plugins: [],
  server: {
    port: 3000,
    open: true,
  },
  build: {
    target: 'es2022',
    minify: true,
    sourcemap: true,
  },
});
