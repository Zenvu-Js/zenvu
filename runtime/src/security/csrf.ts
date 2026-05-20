/**
 * CSRF token management — double-submit cookie pattern.
 */

export class CsrfManager {
  private tokenKey: string;
  private headerName: string;

  constructor(options: { tokenKey?: string; headerName?: string } = {}) {
    this.tokenKey = options.tokenKey || '_csrf';
    this.headerName = options.headerName || 'X-CSRF-Token';
  }

  /** Get the current CSRF token from the meta tag or cookie. */
  getToken(): string {
    // Try meta tag first (SSR-injected)
    const meta = document.querySelector('meta[name="csrf-token"]');
    if (meta) return meta.getAttribute('content') || '';

    // Fall back to cookie
    const match = document.cookie.match(new RegExp(`${this.tokenKey}=([^;]+)`));
    return match ? match[1] : '';
  }

  /** Inject CSRF token into a fetch request. */
  injectHeaders(headers: HeadersInit = {}): Headers {
    const h = new Headers(headers);
    h.set(this.headerName, this.getToken());
    return h;
  }

  /** Create a secure fetch wrapper that auto-injects CSRF tokens. */
  secureFetch(url: string, options: RequestInit = {}): Promise<Response> {
    const method = (options.method || 'GET').toUpperCase();

    // Only inject for state-changing methods
    if (['POST', 'PUT', 'PATCH', 'DELETE'].includes(method)) {
      options.headers = this.injectHeaders(options.headers);
    }

    // Always include credentials for cookie-based CSRF
    options.credentials = options.credentials || 'same-origin';

    return fetch(url, options);
  }

  /** Generate a hidden CSRF input element for forms. */
  createHiddenInput(): HTMLInputElement {
    const input = document.createElement('input');
    input.type = 'hidden';
    input.name = this.tokenKey;
    input.value = this.getToken();
    return input;
  }
}

/** Global CSRF manager instance. */
export const csrf = new CsrfManager();
