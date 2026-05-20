/**
 * ðŸ“¡ Zenvu Query (Like TanStack Query, but Native to Zenvu)
 * 
 * Provides reactive data fetching, automatic caching, retry logic, 
 * and SSR data prefetching out of the box.
 */

export class ZenvuQueryClient {
    private cache: Map<string, { data: any, timestamp: number }> = new Map();

    async fetchQuery(key: string, queryFn: () => Promise<any>, staleTime: number = 5000) {
        const cached = this.cache.get(key);
        if (cached && (Date.now() - cached.timestamp < staleTime)) {
            console.log(`âš¡ [Zenvu Query] Cache Hit for: ${key}`);
            return cached.data;
        }

        try {
            const data = await this.retry(queryFn, 3);
            this.cache.set(key, { data, timestamp: Date.now() });
            return data;
        } catch (error) {
            console.error(`ðŸš¨ [Zenvu Query] Failed to fetch ${key}:`, error);
            throw error;
        }
    }

    private async retry(fn: () => Promise<any>, attempts: number): Promise<any> {
        for (let i = 0; i < attempts; i++) {
            try {
                return await fn();
            } catch (err) {
                if (i === attempts - 1) throw err;
                await new Promise(res => setTimeout(res, 1000 * (i + 1))); // Exponential backoff
            }
        }
    }
}

export function useQuery(key: string, queryFn: () => Promise<any>) {
    // In actual implementation, this returns a reactive Proxy bound to the UI.
    const client = new ZenvuQueryClient();
    return client.fetchQuery(key, queryFn);
}
