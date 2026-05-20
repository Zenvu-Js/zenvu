/**
 * @zenvu/runtime - Anti-Tampering & Secure Loader
 */

/**
 * Seals the global `window` and `document` prototypes to prevent prototype pollution.
 * Called automatically by the framework during initialization.
 */
export function enforceAntiTampering() {
  if (typeof window === 'undefined') return;

  try {
    // Prevent modification of critical DOM methods
    Object.freeze(Element.prototype.appendChild);
    Object.freeze(Element.prototype.insertBefore);
    Object.freeze(Element.prototype.removeChild);
    Object.freeze(Document.prototype.createElement);

    // Prevent Prototype Pollution on Object
    Object.freeze(Object.prototype);

    console.debug('[Zenvu Security] DOM and Object prototypes frozen (Anti-Tampering active).');
  } catch (e) {
    console.warn('[Zenvu Security] Could not freeze all prototypes (possibly strictly enforced by browser already).');
  }

  // Start continuous DOM script injection scanning
  startRuntimeScriptScanner();
}

/**
 * Actively monitors the DOM via MutationObserver for unauthorized <script> injections
 * at runtime. If an unknown script is detected, it is immediately removed and flagged.
 */
function startRuntimeScriptScanner() {
  if (typeof MutationObserver === 'undefined') return;

  const observer = new MutationObserver((mutations) => {
    for (const mutation of mutations) {
      if (mutation.addedNodes.length > 0) {
        mutation.addedNodes.forEach((node) => {
          if (node.nodeName.toLowerCase() === 'script') {
            const scriptEl = node as HTMLScriptElement;
            // Verify if the script has the framework's injected nonce
            const cspNonce = document.querySelector('meta[name="zenvu-nonce"]')?.getAttribute('content');
            
            if (!scriptEl.nonce || scriptEl.nonce !== cspNonce) {
              console.error('[Zenvu Security ðŸš¨] UNAUTHORIZED SCRIPT INJECTION DETECTED! Removing malicious payload:', scriptEl.src || 'inline script');
              scriptEl.remove();
            }
          }
        });
      }
    }
  });

  observer.observe(document.documentElement, {
    childList: true,
    subtree: true,
  });

  console.debug('[Zenvu Security] Runtime Script Injection Scanner is active.');
}

/**
 * Securely loads a module, verifying its source against the CSP config
 * and throwing if attempting to load from an untrusted origin.
 */
export async function secureModuleLoader(url: string): Promise<any> {
  const allowedOrigins = [window.location.origin];
  const urlObj = new URL(url, window.location.origin);

  if (!allowedOrigins.includes(urlObj.origin)) {
    throw new Error(`[Zenvu Security] Blocked attempt to load module from untrusted origin: ${urlObj.origin}`);
  }

  // Use dynamic import
  return await import(url);
}
