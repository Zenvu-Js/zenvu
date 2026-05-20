import { defineConfig } from '@zenvu/cli';

/**
 * ðŸ”µ Zenvu.js Zero-Config Paradigm
 * 
 * Notice how simple this is compared to Webpack or Rollup.
 * Out of the box, Zenvu.js handles:
 * - HMR (Hot Module Replacement)
 * - TypeScript compilation
 * - Code Splitting
 * - Minification
 * - SSR & Edge Deployment
 * 
 * You only need to configure edge cases here.
 */
export default defineConfig({
  server: {
    port: 3000,
    open: true, // Auto-opens browser
  },
  build: {
    target: 'es2024',
    minify: true, // Handled natively by Rust multi-threading
  },
  // ðŸ¢ Enterprise Deep Customization (Solves Vite's lack of control)
  plugins: [
    '@zenvu/plugin-enterprise-auth',
    '@zenvu/plugin-graphql-ast-optimizer'
  ],
  hooks: {
    // Exposes raw low-level control to the developer without messy Webpack loaders
    transformAst(node) {
      if (node.tag === 'ProprietaryTag') {
        // Deep AST manipulation available directly in config!
        node.setAttribute('data-enterprise', 'true');
      }
      return node;
    }
  }
});
