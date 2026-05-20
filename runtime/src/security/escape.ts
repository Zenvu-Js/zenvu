/**
 * Output escaping functions — the primary XSS defense layer.
 *
 * All template interpolations `{{ }}` are passed through `escapeHtml`
 * at compile time. These functions are inlined by the compiler.
 */

const HTML_ESCAPE_MAP: Record<string, string> = {
  '&': '&amp;',
  '<': '&lt;',
  '>': '&gt;',
  '"': '&quot;',
  "'": '&#x27;',
  '/': '&#x2F;',
  '`': '&#96;',
};

const HTML_ESCAPE_RE = /[&<>"'\/`]/g;

/** Escape a string for safe insertion into HTML content. */
export function escapeHtml(str: unknown): string {
  const s = String(str ?? '');
  return s.replace(HTML_ESCAPE_RE, (ch) => HTML_ESCAPE_MAP[ch] || ch);
}

/** Escape a string for safe insertion into an HTML attribute value. */
export function escapeAttr(str: unknown): string {
  const s = String(str ?? '');
  return s.replace(/[&"'<>`]/g, (ch) => HTML_ESCAPE_MAP[ch] || `&#${ch.charCodeAt(0)};`);
}

/** Sanitize a URL — block dangerous schemes. */
export function escapeUrl(url: unknown): string {
  const s = String(url ?? '').trim();
  const lower = s.toLowerCase();

  // Block dangerous URL schemes
  if (lower.startsWith('javascript:') ||
      lower.startsWith('data:') ||
      lower.startsWith('vbscript:') ||
      lower.startsWith('blob:')) {
    return 'about:blank';
  }

  return s;
}

/** Escape a string for safe insertion into a JavaScript string literal. */
export function escapeJs(str: unknown): string {
  const s = String(str ?? '');
  return s
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/"/g, '\\"')
    .replace(/\n/g, '\\n')
    .replace(/\r/g, '\\r')
    .replace(/\u2028/g, '\\u2028')
    .replace(/\u2029/g, '\\u2029');
}
