/**
 * ZenvuComponent â€” Minimal component base for compiled output.
 */

import { ZenvuScheduler } from './scheduler';
import { runLifecycleHook } from './lifecycle';

export type LifecycleHook = 'onMount' | 'onUpdate' | 'onDestroy' | 'onError';

export interface ComponentOptions {
  props?: Record<string, any>;
  slots?: Record<string, () => Node[]>;
}

export interface ComponentInstance {
  /** Mount the component into a target element. */
  $mount(target: Element): void;
  /** Destroy the component and clean up. */
  $destroy(): void;
  /** Update a prop or reactive value. */
  $set(key: string, value: any): void;
  /** Get current component state (for devtools). */
  $inspect(): Record<string, any>;
}

/** Active component stack for lifecycle hook context. */
const componentStack: ComponentInstance[] = [];

/** Get the currently initializing component. */
export function getCurrentComponent(): ComponentInstance | undefined {
  return componentStack[componentStack.length - 1];
}

/**
 * Create a new component instance.
 * This is the factory used by compiled output.
 */
export function createComponent(
  setupFn: (target: Element, props: Record<string, any>) => ComponentInstance,
  target: Element,
  options: ComponentOptions = {}
): ComponentInstance {
  const props = options.props || {};

  // Push onto stack for lifecycle hook registration
  const instance: ComponentInstance = {
    $mount: () => {},
    $destroy: () => {},
    $set: () => {},
    $inspect: () => ({}),
  };

  componentStack.push(instance);

  try {
    const result = setupFn(target, props);
    // Merge the compiled component result
    Object.assign(instance, result);
  } finally {
    componentStack.pop();
  }

  // Run onMount hooks
  runLifecycleHook('onMount', instance);

  return instance;
}

/**
 * Mount a root component into the DOM.
 */
export function mountComponent(
  component: (target: Element, props?: Record<string, any>) => ComponentInstance,
  selector: string | Element,
  props: Record<string, any> = {}
): ComponentInstance {
  const target = typeof selector === 'string'
    ? document.querySelector(selector)
    : selector;

  if (!target) {
    throw new Error(`[Zenvu] Mount target not found: ${selector}`);
  }

  return createComponent(
    (el, p) => component(el, p),
    target,
    { props }
  );
}

/**
 * Zenvu component base class (alternative OOP style).
 */
export abstract class ZenvuComponent {
  protected scheduler = new ZenvuScheduler();
  protected el: Element | null = null;

  abstract render(target: Element): void;

  $mount(target: Element): void {
    this.el = target;
    this.render(target);
  }

  $destroy(): void {
    if (this.el) {
      this.el.innerHTML = '';
      this.el = null;
    }
    this.scheduler.clear();
  }

  $set(_key: string, _value: any): void {
    // Override in subclass
  }

  $inspect(): Record<string, any> {
    return {};
  }

  /** Queue a reactive update. */
  protected invalidate(updateFn: () => void): void {
    this.scheduler.enqueue(updateFn);
  }
}
