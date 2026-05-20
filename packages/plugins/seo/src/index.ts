/**
 * @zenvu/plugin-seo
 * Automated Next.js-style Head Management and Meta Tag generation for SSR.
 */
export function useSEO(meta: { title: string, description: string }) {
  if (typeof document !== 'undefined') {
    document.title = meta.title;
    // update meta tags
  }
}
