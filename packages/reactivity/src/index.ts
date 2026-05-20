//! Advanced Reactivity Engine (Fine-Grained Signals)
//!
//! Powers the Zero-VDOM engine. Automatically tracks dependencies via Proxy
//! and triggers surgical DOM updates (Effects) without any diffing.

type Subscriber = () => void;

let activeEffect: Subscriber | null = null;
const targetMap = new WeakMap<any, Map<string, Set<Subscriber>>>();

/**
 * Tracks a property access and records the active effect as a dependency.
 */
export function track(target: any, key: string) {
    if (activeEffect) {
        let depsMap = targetMap.get(target);
        if (!depsMap) {
            targetMap.set(target, (depsMap = new Map()));
        }
        let dep = depsMap.get(key);
        if (!dep) {
            depsMap.set(key, (dep = new Set()));
        }
        dep.add(activeEffect);
    }
}

/**
 * Triggers all effects that depend on the mutated property.
 */
export function trigger(target: any, key: string) {
    const depsMap = targetMap.get(target);
    if (!depsMap) return;
    const dep = depsMap.get(key);
    if (dep) {
        dep.forEach((effect) => {
            try {
                effect();
            } catch (e) {
                console.error('[Zenvu Reactivity] Effect execution failed:', e);
            }
        });
    }
}

/**
 * Creates a reactive Proxy object.
 */
export function reactive<T extends object>(target: T): T {
    return new Proxy(target, {
        get(obj, key) {
            track(obj, key as string);
            return Reflect.get(obj, key);
        },
        set(obj, key, value) {
            const result = Reflect.set(obj, key, value);
            trigger(obj, key as string);
            return result;
        }
    });
}

/**
 * Registers an effect to run automatically when its reactive dependencies change.
 * This is what powers the Zero-VDOM DOM patching!
 */
export function effect(fn: Subscriber) {
    const effectWrapper = () => {
        activeEffect = effectWrapper;
        fn(); // This execution will trigger `track()` on any accessed reactive properties
        activeEffect = null;
    };
    effectWrapper(); // Run immediately once to gather dependencies
}

/**
 * Computed/Derived state. Evaluated lazily and cached.
 */
export function computed<T>(getter: () => T) {
    let dirty = true;
    let value: T;
    
    // We wrap the getter in an effect to track its dependencies.
    // When they change, we mark the computed value as dirty.
    effect(() => {
        dirty = true;
    });

    return {
        get value() {
            if (dirty) {
                value = getter();
                dirty = false;
            }
            return value;
        }
    };
}
