/**
 * ZenvuScheduler â€” Batches DOM updates into microtasks.
 *
 * Instead of updating the DOM immediately on every state change,
 * the scheduler collects all pending updates and flushes them
 * in a single microtask, preventing layout thrashing.
 */

type UpdateFn = () => void;

export class ZenvuScheduler {
  private queue: Set<UpdateFn> = new Set();
  private pending = false;

  /**
   * Enqueue an update function to run in the next microtask.
   * Duplicate functions are automatically deduplicated.
   */
  enqueue(update: UpdateFn): void {
    this.queue.add(update);
    if (!this.pending) {
      this.pending = true;
      queueMicrotask(() => this.flush());
    }
  }

  /**
   * Flush all pending updates synchronously.
   * Called automatically via microtask, but can be called manually
   * for testing or when immediate updates are needed.
   */
  flush(): void {
    const updates = Array.from(this.queue);
    this.queue.clear();
    this.pending = false;

    for (const fn of updates) {
      try {
        fn();
      } catch (err) {
        console.error('[Zenvu] Update error:', err);
      }
    }
  }

  /** Returns the number of pending updates. */
  get size(): number {
    return this.queue.size;
  }

  /** Cancel all pending updates. */
  clear(): void {
    this.queue.clear();
    this.pending = false;
  }
}

/** Global scheduler instance shared across all components. */
export const globalScheduler = new ZenvuScheduler();
