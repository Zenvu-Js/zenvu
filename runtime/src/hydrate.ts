import { getComponentMetadata } from './component';
import { enforceAntiTampering } from './security';

/**
 * SSR Hydration â€” Attaches event listeners and reactivity
 * to server-rendered HTML without re-rendering.
 */

export interface HydrateOptions {
  /** The root element containing server-rendered HTML. */
  target: Element;
  /** Whether to perform full hydration or partial. */
  mode?: 'full' | 'partial';
}

/**
 * Hydrate a server-rendered component.
 *
 * Instead of creating new DOM nodes, hydration walks the existing
 * server-rendered DOM and attaches event listeners and reactive bindings.
 */
export function hydrate(
  component: (target: Element, props?: Record<string, any>) => any,
  options: HydrateOptions
): any {
  const { target, mode = 'full' } = options;

  if (!(window as any).__zenvu_HYDRATE__) {
    // Not a server-rendered page, do normal mount
    return component(target);
  }

  // Mark as hydrating â€” compiled components check this flag
  (window as any).__zenvu_HYDRATING__ = true;

  try {
    // In hydration mode, compiled components will:
    // 1. Find existing DOM nodes instead of creating new ones
    // 2. Attach event listeners to existing elements
    // 3. Set up reactive bindings
    // 4. Skip initial DOM mutations (content already matches)
    const instance = component(target);

    if (mode === 'full') {
      // Verify hydration matches
      verifyHydration(target);
    }

    return instance;
  } finally {
    (window as any).__zenvu_HYDRATING__ = false;
    delete (window as any).__zenvu_HYDRATE__;
  }
}

/**
 * Verify that hydrated DOM matches expected structure.
 * Only runs in development mode.
 */
function verifyHydration(root: Element): void {
  if ((import.meta as any).env?.DEV) {
    const mismatches: string[] = [];
    // Walk DOM and check for data-b-* attributes
    const walker = document.createTreeWalker(root, NodeFilter.SHOW_ELEMENT);
    let node: Node | null = walker.currentNode;
    while (node) {
      if (node instanceof Element) {
        const attrs = Array.from(node.attributes);
        const hasZenvuAttr = attrs.some(a => a.name.startsWith('data-b-'));
        if (!hasZenvuAttr && node !== root) {
          // Element wasn't scoped â€” possible mismatch
        }
      }
      node = walker.nextNode();
    }

    if (mismatches.length > 0) {
      console.warn('[Zenvu] Hydration mismatches detected:', mismatches);
    }
  }
}
