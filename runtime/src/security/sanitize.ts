/**
 * Client-side input sanitization.
 */

import { escapeHtml, escapeUrl } from './escape';

const DANGEROUS_TAGS = [
  'script', 'iframe', 'object', 'embed', 'form', 'style',
  'link', 'meta', 'base', 'applet', 'math', 'svg',
];

const DANGEROUS_ATTRS = [
  'onclick', 'onerror', 'onload', 'onmouseover', 'onfocus',
  'onZenvur', 'onsubmit', 'onchange', 'onkeydown', 'onkeyup',
  'onmousedown', 'onmouseup', 'oncontextmenu', 'ondblclick',
  'formaction', 'xlink:href', 'data-bind',
];

/** Sanitize any input value â€” type-aware. */
export function sanitize(input: unknown): string {
  if (input === null || input === undefined) return '';
  if (typeof input === 'number' || typeof input === 'boolean') return String(input);
  return escapeHtml(String(input));
}

/** Sanitize HTML â€” strip dangerous tags and attributes, keep safe formatting. */
export function sanitizeHtml(html: string): string {
  const div = document.createElement('div');
  div.innerHTML = html;

  // Remove dangerous elements
  for (const tag of DANGEROUS_TAGS) {
    const elements = div.querySelectorAll(tag);
    elements.forEach(el => el.remove());
  }

  // Remove dangerous attributes from all elements
  const allElements = div.querySelectorAll('*');
  allElements.forEach(el => {
    const attrs = Array.from(el.attributes);
    for (const attr of attrs) {
      const name = attr.name.toLowerCase();
      // Remove event handlers
      if (name.startsWith('on') || DANGEROUS_ATTRS.includes(name)) {
        el.removeAttribute(attr.name);
      }
      // Sanitize href/src URLs
      if (name === 'href' || name === 'src' || name === 'action') {
        const sanitized = escapeUrl(attr.value);
        if (sanitized === 'about:blank') {
          el.removeAttribute(attr.name);
        } else {
          el.setAttribute(attr.name, sanitized);
        }
      }
      // Remove style attributes with expressions
      if (name === 'style') {
        const lower = attr.value.toLowerCase();
        if (lower.includes('expression(') || lower.includes('javascript:') || lower.includes('url(')) {
          el.removeAttribute(attr.name);
        }
      }
    }
  });

  return div.innerHTML;
}

/** Sanitize a URL for safe usage. */
export function sanitizeUrl(url: string): string {
  return escapeUrl(url);
}
