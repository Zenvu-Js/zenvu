/**
 * ðŸ”µ Zenvu.js Runtime
 *
 * Minimal browser runtime (< 2KB gzipped).
 * Provides scheduling, event delegation, lifecycle management,
 * and SSR hydration support.
 */

export { ZenvuScheduler } from './scheduler';
export { ZenvuEvents } from './events';
export { ZenvuComponent, createComponent, mountComponent } from './component';
export { onMount, onUpdate, onDestroy, onError } from './lifecycle';
export { hydrate } from './hydrate';

// Re-export types
export type { ComponentOptions, ComponentInstance, LifecycleHook } from './component';
