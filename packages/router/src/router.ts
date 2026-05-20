/**
 * Core router implementation.
 */

export interface Route {
  path: string;
  name?: string;
  params: Record<string, string>;
  query: Record<string, string>;
  hash: string;
  matched: RouteConfig[];
}

export interface RouteConfig {
  path: string;
  name?: string;
  component: () => Promise<any> | any;
  children?: RouteConfig[];
  meta?: Record<string, any>;
  props?: boolean | Record<string, any>;
  redirect?: string;
  beforeEnter?: NavigationGuard;
  transition?: string; // Route transition animation
  layout?: string; // Layout routes
}

export interface RouterOptions {
  mode?: 'history' | 'hash';
  routes: RouteConfig[];
  base?: string;
  scrollBehavior?: (to: Route, from: Route) => { x: number, y: number };
}

export type NavigationGuard = (to: Route, from: Route) => boolean | string | void | Promise<boolean | string | void>;

let currentRoute: Route = {
  path: '/', name: undefined, params: {}, query: {}, hash: '', matched: [],
};
let routeConfigs: RouteConfig[] = [];
let routeListeners: ((route: Route) => void)[] = [];
let routerMode: 'history' | 'hash' = 'history';

// Advanced routing mechanisms
const routeCache: Map<string, any> = new Map();
let globalGuards: NavigationGuard[] = [];

/** Register a global middleware/guard */
export function beforeEach(guard: NavigationGuard) {
  globalGuards.push(guard);
}

/** Prefetch a route bundle before navigation */
export function prefetch(path: string) {
  const matched = matchRoutes(path, routeConfigs);
  if (matched.length > 0 && typeof matched[0].component === 'function') {
    // Execute the dynamic import to warm up the cache
    matched[0].component();
  }
}

/** Create and configure the router. */
export function createRouter(options: RouterOptions) {
  routeConfigs = options.routes;
  routerMode = options.mode || 'history';

  // Listen for browser navigation
  window.addEventListener('popstate', () => {
    const path = routerMode === 'hash'
      ? window.location.hash.slice(1) || '/'
      : window.location.pathname;
    resolveRoute(path);
  });

  // Resolve initial route
  const initialPath = routerMode === 'hash'
    ? window.location.hash.slice(1) || '/'
    : window.location.pathname;
  resolveRoute(initialPath);

  return { install: () => {}, currentRoute: () => currentRoute };
}

/** Get the current route reactively. */
export function useRoute(): Route {
  return currentRoute;
}

/** Get the router instance. */
export function useRouter() {
  return { navigate, back, forward, replace, currentRoute: () => currentRoute };
}

/** Navigate to a path. */
export function navigate(path: string, state?: any): void {
  if (routerMode === 'hash') {
    window.location.hash = path;
  } else {
    window.history.pushState(state || {}, '', path);
  }
  resolveRoute(path);
}

/** Go back in history. */
export function back(): void { window.history.back(); }

/** Go forward in history. */
export function forward(): void { window.history.forward(); }

/** Replace the current history entry. */
export function replace(path: string): void {
  if (routerMode === 'hash') {
    window.location.replace(`#${path}`);
  } else {
    window.history.replaceState({}, '', path);
  }
  resolveRoute(path);
}

function resolveRoute(path: string): void {
  const [pathname, queryString] = path.split('?');
  const [cleanPath, hash] = pathname.split('#');

  const query: Record<string, string> = {};
  if (queryString) {
    new URLSearchParams(queryString).forEach((v, k) => { query[k] = v; });
  }

  const matched = matchRoutes(cleanPath, routeConfigs);

  const targetRoute: Route = {
    path: cleanPath,
    name: matched[0]?.name,
    params: extractParams(cleanPath, matched[0]?.path || ''),
    query,
    hash: hash || '',
    matched,
  };

  // Run Global Middleware (Protected Routes)
  for (const guard of globalGuards) {
    const result = guard(targetRoute, currentRoute);
    if (typeof result === 'string') {
      navigate(result); // Redirect
      return;
    } else if (result === false) {
      return; // Cancel navigation
    }
  }

  // Run Per-Route Guard
  if (matched[0]?.beforeEnter) {
    const result = matched[0].beforeEnter(targetRoute, currentRoute);
    if (result === false) return;
  }

  currentRoute = targetRoute;

  // Check for redirects
  if (matched[0]?.redirect) {
    navigate(matched[0].redirect);
    return;
  }

  // Notify listeners (Triggers UI Update and Route Transitions)
  for (const listener of routeListeners) {
    listener(currentRoute);
  }
}

function matchRoutes(path: string, routes: RouteConfig[]): RouteConfig[] {
  for (const route of routes) {
    if (matchPath(path, route.path)) {
      return [route];
    }
    if (route.children) {
      const childMatch = matchRoutes(path, route.children);
      if (childMatch.length > 0) {
        return [route, ...childMatch];
      }
    }
  }
  return [];
}

function matchPath(path: string, pattern: string): boolean {
  if (pattern === path) return true;
  if (pattern === '*') return true;

  const patternParts = pattern.split('/').filter(Boolean);
  const pathParts = path.split('/').filter(Boolean);

  if (patternParts.length !== pathParts.length) return false;

  return patternParts.every((part, i) => part.startsWith(':') || part === pathParts[i]);
}

function extractParams(path: string, pattern: string): Record<string, string> {
  const params: Record<string, string> = {};
  const patternParts = pattern.split('/').filter(Boolean);
  const pathParts = path.split('/').filter(Boolean);

  patternParts.forEach((part, i) => {
    if (part.startsWith(':')) {
      params[part.slice(1)] = pathParts[i] || '';
    }
  });

  return params;
}
