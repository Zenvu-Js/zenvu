/**
 * Lifecycle hooks for Zenvu.js components.
 *
 * Hooks are registered during component setup and called
 * at the appropriate phase of the component lifecycle.
 */

import { getCurrentComponent, type ComponentInstance } from './component';

type HookCallback = () => void | (() => void);

const hooks = new WeakMap<ComponentInstance, Map<string, HookCallback[]>>();

function registerHook(name: string, callback: HookCallback): void {
  const component = getCurrentComponent();
  if (!component) {
    console.warn(`[Zenvu] ${name}() called outside of component setup`);
    return;
  }

  if (!hooks.has(component)) {
    hooks.set(component, new Map());
  }
  const componentHooks = hooks.get(component)!;
  if (!componentHooks.has(name)) {
    componentHooks.set(name, []);
  }
  componentHooks.get(name)!.push(callback);
}

/** Called after the component is mounted to the DOM. */
export function onMount(callback: HookCallback): void {
  registerHook('onMount', callback);
}

/** Called after any reactive state update. */
export function onUpdate(callback: HookCallback): void {
  registerHook('onUpdate', callback);
}

/** Called before the component is destroyed. */
export function onDestroy(callback: HookCallback): void {
  registerHook('onDestroy', callback);
}

/** Called when an error occurs during rendering or updates. */
export function onError(callback: (error: Error) => void): void {
  registerHook('onError', callback as HookCallback);
}

/** Run all registered hooks of a given type for a component. */
export function runLifecycleHook(name: string, component: ComponentInstance): void {
  const componentHooks = hooks.get(component);
  if (!componentHooks) return;

  const callbacks = componentHooks.get(name);
  if (!callbacks) return;

  for (const cb of callbacks) {
    try {
      cb();
    } catch (err) {
      console.error(`[Zenvu] Error in ${name}:`, err);
      // Try to run onError hooks
      if (name !== 'onError') {
        const errorHooks = componentHooks.get('onError');
        if (errorHooks) {
          for (const eh of errorHooks) {
            (eh as (error: Error) => void)(err as Error);
          }
        }
      }
    }
  }
}
