/**
 * Authentication middleware for Zenvu.js server routes.
 */

export interface AuthConfig {
  /** Routes that don't require authentication. */
  publicPaths: string[];
  /** Login redirect path. */
  loginPath: string;
  /** Session cookie name. */
  sessionCookie: string;
  /** Maximum session age in seconds. */
  maxAge: number;
}

const defaultConfig: AuthConfig = {
  publicPaths: ['/', '/login', '/register', '/api/health'],
  loginPath: '/login',
  sessionCookie: '__zenvu_session',
  maxAge: 86400,
};

export interface AuthContext {
  isAuthenticated: boolean;
  userId?: string;
  roles: string[];
  sessionId?: string;
}

type NextFn = () => Promise<void> | void;
type MiddlewareFn = (ctx: { path: string; headers: Record<string, string>; auth: AuthContext }, next: NextFn) => Promise<void> | void;

/**
 * Create authentication middleware.
 */
export function createAuthMiddleware(config: Partial<AuthConfig> = {}): MiddlewareFn {
  const cfg = { ...defaultConfig, ...config };

  return async (ctx, next) => {
    // Allow public paths
    if (cfg.publicPaths.some(p => ctx.path === p || ctx.path.startsWith(p + '/'))) {
      ctx.auth = { isAuthenticated: false, roles: [] };
      return next();
    }

    // Extract session cookie
    const cookieHeader = ctx.headers['cookie'] || '';
    const sessionMatch = cookieHeader.match(new RegExp(`${cfg.sessionCookie}=([^;]+)`));

    if (!sessionMatch) {
      ctx.auth = { isAuthenticated: false, roles: [] };
      // In API routes, return 401; in pages, redirect to login
      if (ctx.path.startsWith('/api/')) {
        throw new AuthError(401, 'Authentication required');
      }
      // Redirect would be handled by the server
      return;
    }

    const sessionId = sessionMatch[1];

    // Session validation would be done by the Rust session store
    // For now, set authenticated context
    ctx.auth = {
      isAuthenticated: true,
      sessionId,
      userId: undefined, // Resolved from session store
      roles: [],
    };

    return next();
  };
}

export class AuthError extends Error {
  constructor(public status: number, message: string) {
    super(message);
    this.name = 'AuthError';
  }
}
