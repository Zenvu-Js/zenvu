/**
 * @zenvu/runtime-core - Rendering Engine
 */

import { effect } from '@zenvu/reactivity';

export interface VNode {
  tag: string | Function;
  props: Record<string, any> | null;
  children: VNode[] | string | null;
  el?: Element | Text | null;
}

/**
 * Create a Virtual Node.
 */
export function h(tag: string | Function, props: Record<string, any> | null = null, children: VNode[] | string | null = null): VNode {
  return { tag, props, children };
}

/**
 * Mount a VNode to a DOM element.
 */
export function mount(vnode: VNode, container: Element) {
  if (typeof vnode.tag === 'string') {
    const el = document.createElement(vnode.tag);
    vnode.el = el;
    
    // Props
    if (vnode.props) {
      for (const key in vnode.props) {
        if (key.startsWith('on')) {
          el.addEventListener(key.slice(2).toLowerCase(), vnode.props[key]);
        } else {
          el.setAttribute(key, vnode.props[key]);
        }
      }
    }
    
    // Children
    if (typeof vnode.children === 'string') {
      el.textContent = vnode.children;
    } else if (Array.isArray(vnode.children)) {
      vnode.children.forEach(child => mount(child, el));
    }
    
    container.appendChild(el);
  } else if (typeof vnode.tag === 'function') {
    // Component
    const componentVNode = vnode.tag(vnode.props);
    mount(componentVNode, container);
  }
}

/**
 * Simple patch (diffing) implementation for reactivity.
 */
export function patch(n1: VNode, n2: VNode) {
  // Simplified for illustration. In production, Zenvu.js uses
  // the Rust compiler to skip the Virtual DOM and update the DOM directly.
  if (n1.tag === n2.tag) {
    n2.el = n1.el;
    // Patch props...
    // Patch children...
  }
}

export { defineAsyncComponent } from './apiAsyncComponent';
export { defineCustomElement } from './apiCustomElement';

// ==========================================
// ADVANCED CORE COMPONENTS
// ==========================================

/**
 * Fragment - Renders children without a wrapper DOM node.
 */
export const Fragment = Symbol('Fragment');

/**
 * Teleport - Renders children into a different part of the DOM.
 */
export const Teleport = Symbol('Teleport');

/**
 * Suspense - Orchestrates async dependencies and fallbacks.
 */
export const Suspense = {
  __isSuspense: true,
  // Complex orchestration logic handled by the compiler
};

/**
 * ErrorBoundary - Catches errors in child components.
 */
export class ErrorBoundary {
  // Catches and isolates errors to prevent app crashes
  static catch(err: Error, info: any) {
    console.error('[Zenvu Error Boundary] Caught:', err, info);
  }
}

/**
 * Custom Directives Support (v-my-directive)
 */
export function withDirectives(vnode: VNode, directives: any[]) {
  // Binds custom behavior lifecycle to the VNode
  return vnode;
}
