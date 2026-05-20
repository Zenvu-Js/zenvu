/**
 * Navigation guards — run before/after route changes.
 */
import type { Route, NavigationGuard } from './router';

const beforeEachGuards: NavigationGuard[] = [];
const beforeResolveGuards: NavigationGuard[] = [];
const afterEachHooks: ((to: Route, from: Route) => void)[] = [];

export function beforeEach(guard: NavigationGuard): () => void {
  beforeEachGuards.push(guard);
  return () => {
    const i = beforeEachGuards.indexOf(guard);
    if (i > -1) beforeEachGuards.splice(i, 1);
  };
}

export function beforeResolve(guard: NavigationGuard): () => void {
  beforeResolveGuards.push(guard);
  return () => {
    const i = beforeResolveGuards.indexOf(guard);
    if (i > -1) beforeResolveGuards.splice(i, 1);
  };
}

export function afterEach(hook: (to: Route, from: Route) => void): () => void {
  afterEachHooks.push(hook);
  return () => {
    const i = afterEachHooks.indexOf(hook);
    if (i > -1) afterEachHooks.splice(i, 1);
  };
}

/** Run all beforeEach guards. Returns false if navigation should be cancelled. */
export async function runBeforeEachGuards(to: Route, from: Route): Promise<boolean> {
  for (const guard of beforeEachGuards) {
    const result = await guard(to, from);
    if (result === false) return false;
    if (typeof result === 'string') return false; // redirect handled externally
  }
  return true;
}
