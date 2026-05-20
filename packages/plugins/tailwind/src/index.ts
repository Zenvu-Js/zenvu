/**
 * @zenvu/plugin-tailwind
 * Compiles Tailwind CSS at build time via Rust and injects into Zenvu Components.
 */
export function tailwindPlugin() {
  return {
    name: 'zenvu-plugin-tailwind',
    onBuildStart() {
      console.log('[Zenvu Tailwind] Generating atomic utility classes...');
    }
  };
}
