/**
 * â³ Zenvu Suspense & Lazy Mount Engine
 * 
 * Natively supports async component resolution and fallback boundaries.
 */

export class SuspenseBoundary {
    private fallback: HTMLElement;
    private target: HTMLElement;
    private resolved: boolean = false;

    constructor(fallback: HTMLElement, target: HTMLElement) {
        this.fallback = fallback;
        this.target = target;
    }

    /**
     * Executes the async component promise. Shows fallback until resolved.
     */
    async mountLazy(asyncComponentFn: () => Promise<HTMLElement>) {
        // 1. Show Fallback (Skeleton/Spinner)
        this.target.appendChild(this.fallback);

        try {
            // 2. Await the lazy chunk import
            const resolvedComponent = await asyncComponentFn();
            
            // 3. Reconcile & Replace Fallback
            this.target.innerHTML = '';
            this.target.appendChild(resolvedComponent);
            this.resolved = true;
            console.log('âœ¨ [Zenvu Suspense] Async component resolved and mounted.');
        } catch (error) {
            console.error('ðŸš¨ [Zenvu Suspense] Failed to load lazy component:', error);
            this.target.innerHTML = '<div class="zenvu-error">Failed to load component.</div>';
        }
    }
}

/**
 * Global Lazy Mount System API
 */
export function lazy(importFn: () => Promise<any>): () => Promise<HTMLElement> {
    return async () => {
        const module = await importFn();
        // Assuming the module exports a mountable factory as default
        return module.default();
    };
}
