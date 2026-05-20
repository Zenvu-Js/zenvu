/**
 * @zenvu/hooks - Lifecycle & Custom Hooks
 */

type LifecycleHook = () => void | (() => void);

let currentInstance: any = null;

export function setCurrentInstance(instance: any) {
  currentInstance = instance;
}

function injectHook(type: string, hook: LifecycleHook) {
  if (currentInstance) {
    if (!currentInstance.hooks) currentInstance.hooks = {};
    if (!currentInstance.hooks[type]) currentInstance.hooks[type] = [];
    currentInstance.hooks[type].push(hook);
  } else {
    console.warn(`[Zenvu] Lifecycle hook ${type} was called outside of a component setup.`);
  }
}

/** Called before the component is mounted to the DOM. */
export const onBeforeMount = (hook: LifecycleHook) => injectHook('beforeMount', hook);

/** Called after the component has been mounted to the DOM. */
export const onMounted = (hook: LifecycleHook) => injectHook('mounted', hook);

/** Called before the component updates due to reactive changes. */
export const onBeforeUpdate = (hook: LifecycleHook) => injectHook('beforeUpdate', hook);

/** Called after the component has updated. */
export const onUpdated = (hook: LifecycleHook) => injectHook('updated', hook);

/** Called before the component is unmounted and destroyed. */
export const onBeforeUnmount = (hook: LifecycleHook) => injectHook('beforeUnmount', hook);

/** Called after the component has been unmounted. */
export const onUnmounted = (hook: LifecycleHook) => injectHook('unmounted', hook);

// ==========================================
// REACT-COMPATIBLE HOOKS API
// ==========================================

import { ref, effect } from '@zenvu/reactivity';

/**
 * React-compatible `useState` hook powered by Zenvu's native reactivity.
 */
export function useState<T>(initialValue: T): [() => T, (val: T) => void] {
  const state = ref(initialValue);
  
  const getter = () => state.value;
  const setter = (newValue: T) => {
    state.value = newValue;
  };
  
  return [getter, setter];
}

/**
 * React-compatible `useEffect` hook.
 */
export function useEffect(callback: () => void | (() => void), dependencies?: any[]) {
  // In Zenvu.js, effects are automatically tracked. Dependencies array is optional.
  effect(() => {
    const cleanup = callback();
    // Register cleanup to unmount lifecycle if it's a function
    if (typeof cleanup === 'function') {
      onBeforeUnmount(cleanup);
    }
  });
}
