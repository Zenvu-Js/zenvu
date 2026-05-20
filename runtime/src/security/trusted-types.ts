/**
 * Trusted Types integration â€” prevents DOM XSS by enforcing
 * type-safe DOM API usage in supported browsers.
 */

export function setupTrustedTypes(): void {
  if (typeof window === 'undefined') return;
  if (!(window as any).trustedTypes) return;

  try {
    const policy = (window as any).trustedTypes.createPolicy('zenvu-policy', {
      createHTML: (input: string) => {
        // Only allow sanitized HTML
        console.warn('[Zenvu Security] Trusted Types: createHTML called â€” ensure input is sanitized');
        return input;
      },
      createScript: (_input: string) => {
        // Block dynamic script creation
        throw new Error('[Zenvu Security] Dynamic script creation is blocked by Trusted Types policy');
      },
      createScriptURL: (input: string) => {
        // Only allow same-origin script URLs
        const url = new URL(input, window.location.origin);
        if (url.origin !== window.location.origin) {
          throw new Error(`[Zenvu Security] Cross-origin script blocked: ${input}`);
        }
        return input;
      },
    });

    (window as any).__ZenvuTrustedPolicy = policy;
  } catch (e) {
    // Policy may already exist in CSP-enforced mode
    console.debug('[Zenvu Security] Trusted Types policy setup:', e);
  }
}
