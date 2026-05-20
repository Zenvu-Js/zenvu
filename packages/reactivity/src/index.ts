//! Advanced Reactivity Engine (Fine-Grained Signals)
//!
//! Powers the Zero-VDOM engine. Automatically tracks dependencies via Proxy
//! and triggers surgical DOM updates (Effects) without any diffing.
//! Features automatic dependency cleanup, memoized computed properties,
//! and queued batched updates.

type Subscriber = () => void;

interface SignalNode<T = any> {
    value: T;
    subscribers: Set<EffectNode>;
}

interface EffectNode {
    fn: Subscriber;
    dependencies: Set<SignalNode>;
    run(): void;
}

let activeEffectNode: EffectNode | null = null;

// Batched updates queue
const effectQueue = new Set<EffectNode>();
let isBatching = false;
let isQueuePending = false;

function flushQueue() {
    const list = Array.from(effectQueue);
    effectQueue.clear();
    isQueuePending = false;
    list.forEach(node => {
        try {
            node.run();
        } catch (e) {
            console.error('🔴 [Zenvu Reactivity] Effect execution failed:', e);
        }
    });
}

/**
 * Batches multiple state updates together, running all dependent effects
 * exactly once at the end of the batch.
 */
export function batch(fn: () => void) {
    const prevBatching = isBatching;
    isBatching = true;
    try {
        fn();
    } finally {
        isBatching = prevBatching;
        if (!isBatching && effectQueue.size > 0 && !isQueuePending) {
            isQueuePending = true;
            queueMicrotask(flushQueue);
        }
    }
}

/**
 * Creates a raw Signal representing a single piece of fine-grained reactive state.
 * Returns a getter and setter tuple [get, set].
 */
export function createSignal<T>(initialValue: T): [() => T, (val: T | ((prev: T) => T)) => void] {
    const node: SignalNode<T> = {
        value: initialValue,
        subscribers: new Set()
    };

    const get = () => {
        if (activeEffectNode) {
            node.subscribers.add(activeEffectNode);
            activeEffectNode.dependencies.add(node);
        }
        return node.value;
    };

    const set = (newValue: T | ((prev: T) => T)) => {
        const nextValue = typeof newValue === 'function' 
            ? (newValue as Function)(node.value) 
            : newValue;
            
        if (nextValue !== node.value) {
            node.value = nextValue;
            
            node.subscribers.forEach(sub => {
                effectQueue.add(sub);
            });

            if (!isBatching && effectQueue.size > 0 && !isQueuePending) {
                isQueuePending = true;
                queueMicrotask(flushQueue);
            }
        }
    };

    return [get, set];
}

/**
 * Registers a side effect that automatically tracks accessed signals
 * and runs whenever any of those signals change.
 */
export function effect(fn: Subscriber) {
    const node: EffectNode = {
        fn,
        dependencies: new Set(),
        run() {
            // Dynamic dependency tracking: Clean up old signal links
            this.dependencies.forEach(sig => sig.subscribers.delete(this));
            this.dependencies.clear();

            const prevEffect = activeEffectNode;
            activeEffectNode = this;
            try {
                this.fn();
            } finally {
                activeEffectNode = prevEffect;
            }
        }
    };
    node.run();
}

/**
 * Computed/Derived state. Evaluated lazily, automatically memoized,
 * and only recalculates when its dependencies change.
 */
export function computed<T>(getter: () => T): { readonly value: T } {
    const [getSignal, setSignal] = createSignal<T>(undefined as any);

    effect(() => {
        const newValue = getter();
        setSignal(newValue);
    });

    return {
        get value() {
            return getSignal();
        }
    };
}

// Global WeakMap for proxy reactive state tracking
const targetMap = new WeakMap<object, Map<string, { get: () => any, set: (v: any) => void }>>();

/**
 * Creates a reactive Proxy object for intuitive, property-based state management.
 * Transparently wraps each property in a fine-grained Signal.
 */
export function reactive<T extends object>(target: T): T {
    if (typeof target !== 'object' || target === null) return target;

    let signalMap = targetMap.get(target);
    if (!signalMap) {
        signalMap = new Map();
        targetMap.set(target, signalMap);
    }

    const getSignalForProp = (key: string, initialValue: any) => {
        let sig = signalMap!.get(key);
        if (!sig) {
            const [get, set] = createSignal(initialValue);
            sig = { get, set };
            signalMap!.set(key, sig);
        }
        return sig;
    };

    return new Proxy(target, {
        get(obj, key: string) {
            const val = Reflect.get(obj, key);
            if (typeof val === 'function') {
                return val.bind(obj);
            }
            if (typeof val === 'object' && val !== null) {
                return reactive(val); // Nested proxy wrapping
            }
            return getSignalForProp(key, val).get();
        },
        set(obj, key: string, value: any) {
            const result = Reflect.set(obj, key, value);
            getSignalForProp(key, value).set(value);
            return result;
        }
    });
}

/**
 * Creates a ref object wrapping a value, with a reactive .value property.
 */
export function ref<T>(initialValue: T): { value: T } {
    const [get, set] = createSignal(initialValue);
    return {
        get value() {
            return get();
        },
        set value(newVal) {
            set(newVal);
        }
    };
}
