/**
 * CSP nonce manager — handles nonce injection for inline scripts/styles.
 */

export class CspManager {
  private nonce: string;

  constructor() {
    // Read nonce from meta tag (injected by SSR)
    const meta = document.querySelector('meta[name="csp-nonce"]');
    this.nonce = meta?.getAttribute('content') || '';
  }

  /** Get the current CSP nonce. */
  getNonce(): string {
    return this.nonce;
  }

  /** Create a script element with the correct nonce. */
  createScript(src?: string, content?: string): HTMLScriptElement {
    const script = document.createElement('script');
    if (this.nonce) script.nonce = this.nonce;
    if (src) script.src = src;
    if (content) script.textContent = content;
    script.type = 'module';
    return script;
  }

  /** Create a style element with the correct nonce. */
  createStyle(content: string): HTMLStyleElement {
    const style = document.createElement('style');
    if (this.nonce) style.nonce = this.nonce;
    style.textContent = content;
    return style;
  }
}
