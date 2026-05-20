/**
 * ðŸ‘ï¸ Zenvu Sentinel (Monitoring Security)
 * Real-time DOM Mutation scanning to block malicious injected scripts.
 */

export function initZenvuSentinel() {
    console.log('ðŸ‘ï¸ [Zenvu Sentinel] Starting real-time DOM monitoring...');

    const observer = new MutationObserver((mutations) => {
        for (const mutation of mutations) {
            for (const node of Array.from(mutation.addedNodes)) {
                if (node.nodeName.toLowerCase() === 'script') {
                    const scriptEl = node as HTMLScriptElement;
                    
                    // Verify if script has a valid cryptographic nonce (CSP)
                    if (!scriptEl.nonce || scriptEl.nonce !== window.__zenvu_CSP_NONCE__) {
                        console.error('ðŸ‘ï¸ [Zenvu Sentinel ðŸš¨] UNAUTHORIZED SCRIPT INJECTION DETECTED! Blocking execution.');
                        scriptEl.remove(); // Kill the script before it executes
                    }
                }
            }
        }
    });

    observer.observe(document.documentElement, {
        childList: true,
        subtree: true
    });
}
