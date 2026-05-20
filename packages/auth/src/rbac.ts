/**
 * Role-Based Access Control (RBAC) for Zenvu.js.
 */

export interface Permission {
  resource: string;
  actions: ('create' | 'read' | 'update' | 'delete' | '*')[];
}

export interface Role {
  name: string;
  permissions: Permission[];
  inherits?: string[]; // Inherit permissions from other roles
}

export class RBAC {
  private roles: Map<string, Role> = new Map();

  /** Define a role. */
  defineRole(role: Role): void {
    this.roles.set(role.name, role);
  }

  /** Check if a role has permission to perform an action on a resource. */
  hasPermission(roleName: string, resource: string, action: string): boolean {
    const role = this.roles.get(roleName);
    if (!role) return false;

    // Check direct permissions
    for (const perm of role.permissions) {
      if (this.matchResource(perm.resource, resource)) {
        if (perm.actions.includes('*') || perm.actions.includes(action as any)) {
          return true;
        }
      }
    }

    // Check inherited roles
    if (role.inherits) {
      for (const parentRole of role.inherits) {
        if (this.hasPermission(parentRole, resource, action)) {
          return true;
        }
      }
    }

    return false;
  }

  /** Check if any of the given roles has the required permission. */
  authorize(roles: string[], resource: string, action: string): boolean {
    return roles.some(role => this.hasPermission(role, resource, action));
  }

  /** Create middleware that enforces RBAC. */
  guard(resource: string, action: string) {
    return (ctx: { auth: { roles: string[] } }, next: () => void) => {
      if (!this.authorize(ctx.auth.roles, resource, action)) {
        throw new Error(`Access denied: requires ${action} on ${resource}`);
      }
      return next();
    };
  }

  private matchResource(pattern: string, resource: string): boolean {
    if (pattern === '*') return true;
    if (pattern === resource) return true;
    if (pattern.endsWith('*')) {
      return resource.startsWith(pattern.slice(0, -1));
    }
    return false;
  }
}

/** Pre-configured RBAC with common roles. */
export function createDefaultRBAC(): RBAC {
  const rbac = new RBAC();

  rbac.defineRole({
    name: 'admin',
    permissions: [{ resource: '*', actions: ['*'] }],
  });

  rbac.defineRole({
    name: 'editor',
    permissions: [
      { resource: 'posts', actions: ['create', 'read', 'update'] },
      { resource: 'media', actions: ['create', 'read', 'update', 'delete'] },
      { resource: 'comments', actions: ['read', 'update', 'delete'] },
    ],
  });

  rbac.defineRole({
    name: 'viewer',
    permissions: [
      { resource: 'posts', actions: ['read'] },
      { resource: 'media', actions: ['read'] },
      { resource: 'comments', actions: ['read', 'create'] },
    ],
  });

  return rbac;
}
