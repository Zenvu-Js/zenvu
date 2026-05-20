/**
 * @zenvu/router â€” Built-in router for Zenvu.js
 *
 * Supports SPA history-based routing, SSR-compatible,
 * navigation guards, code splitting, and nested routes.
 */

export { createRouter, useRoute, useRouter } from './router';
export { navigate, back, forward, replace } from './router';
export type { Route, RouteConfig, RouterOptions, NavigationGuard } from './router';
export { createLink } from './link';
export { beforeEach, beforeResolve, afterEach } from './guards';
