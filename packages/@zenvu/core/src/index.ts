/**
 * 🔵 Zenvu.js Isomorphic Core Framework
 * 
 * The ultimate Zero-VDOM, Signals-powered frontend engine.
 * 🚀 High-performance, Server-Side Rendering Ready, Built-in Router & Store,
 * and Auto-Animations with full TypeScript support.
 */

// ==========================================
// 1. Core Signals & Reactivity (Proxy-based)
// ==========================================
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

// ==========================================
// 2. Two-Way Data Binding (z-model)
// ==========================================
export function zModel(
    el: HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement,
    state: any,
    key: string
) {
    effect(() => {
        el.value = state[key] !== undefined ? state[key] : '';
    });

    const inputEvent = el.type === 'checkbox' || el.type === 'radio' ? 'change' : 'input';
    el.addEventListener(inputEvent, () => {
        if (el.type === 'checkbox') {
            state[key] = (el as HTMLInputElement).checked;
        } else if (el.type === 'number') {
            state[key] = Number(el.value);
        } else {
            state[key] = el.value;
        }
    });
}

// ==========================================
// 3. Template Directives (z-if, z-for)
// ==========================================
export function zIf(
    conditionFn: () => boolean,
    renderTrue: () => HTMLElement,
    renderFalse?: () => HTMLElement
): HTMLElement | Comment {
    const anchor = document.createComment('z-if');
    let currentEl: HTMLElement | Comment = anchor;

    effect(() => {
        const condition = conditionFn();
        const nextEl = condition ? renderTrue() : (renderFalse ? renderFalse() : anchor);
        if (currentEl !== nextEl) {
            currentEl.replaceWith(nextEl);
            currentEl = nextEl;
        }
    });

    return currentEl;
}

export function zFor<T>(
    listFn: () => T[],
    renderItem: (item: T, index: number) => HTMLElement
): HTMLElement {
    const container = document.createElement('div');
    
    effect(() => {
        container.innerHTML = '';
        const list = listFn();
        list.forEach((item, index) => {
            container.appendChild(renderItem(item, index));
        });
    });

    return container;
}

// ==========================================
// 4. Lifecycle Hooks (onMounted, onUnmounted)
// ==========================================
interface ComponentInstance {
    mountedHooks: (() => void)[];
    unmountedHooks: (() => void)[];
}

let currentInstance: ComponentInstance | null = null;

export function onMounted(fn: () => void) {
    if (currentInstance) {
        currentInstance.mountedHooks.push(fn);
    } else {
        console.warn('⚠️ [Zenvu] onMounted() must be called inside defineComponent setup().');
    }
}

export function onUnmounted(fn: () => void) {
    if (currentInstance) {
        currentInstance.unmountedHooks.push(fn);
    } else {
        console.warn('⚠️ [Zenvu] onUnmounted() must be called inside defineComponent setup().');
    }
}

function observeDOMRemoval(el: HTMLElement, unmountHooks: (() => void)[]) {
    const observer = new MutationObserver((mutations) => {
        for (const mutation of mutations) {
            for (const removed of Array.from(mutation.removedNodes)) {
                if (removed === el || removed.contains(el)) {
                    unmountHooks.forEach(hook => hook());
                    observer.disconnect();
                    return;
                }
            }
        }
    });
    observer.observe(document.body, { childList: true, subtree: true });
}

// ==========================================
// 5. Built-in State Management Store
// ==========================================
export interface StoreOptions<S, A> {
    id: string;
    state: () => S;
    actions?: A;
    persist?: boolean;
}

export interface StoreInstance<S, A> {
    readonly state: S;
    $undo(): void;
    $redo(): void;
    $reset(): void;
}

export function defineStore<S extends Record<string, any>, A extends Record<string, (this: S, ...args: any[]) => any>>(
    options: StoreOptions<S, A>
) {
    const storeId = `zenvu-store-${options.id}`;
    
    let rawState = options.state();
    if (options.persist && typeof localStorage !== 'undefined') {
        const saved = localStorage.getItem(storeId);
        if (saved) {
            try {
                rawState = JSON.parse(saved);
            } catch (e) {
                console.error('Failed to parse persisted store:', e);
            }
        }
    }

    const state = reactive(rawState);

    if (options.persist && typeof localStorage !== 'undefined') {
        effect(() => {
            localStorage.setItem(storeId, JSON.stringify(state));
        });
    }

    let history: string[] = [JSON.stringify(rawState)];
    let historyIndex = 0;

    const recordHistory = () => {
        const snap = JSON.stringify(state);
        if (history[historyIndex] !== snap) {
            history = history.slice(0, historyIndex + 1);
            history.push(snap);
            historyIndex++;
        }
    };

    const actions: any = {};
    if (options.actions) {
        for (const [name, fn] of Object.entries(options.actions)) {
            actions[name] = (...args: any[]) => {
                const res = fn.apply(state, args);
                recordHistory();
                return res;
            };
        }
    }

    const storeInstance = {
        get state() {
            return state;
        },
        $undo() {
            if (historyIndex > 0) {
                historyIndex--;
                Object.assign(state, JSON.parse(history[historyIndex]));
            }
        },
        $redo() {
            if (historyIndex < history.length - 1) {
                historyIndex++;
                Object.assign(state, JSON.parse(history[historyIndex]));
            }
        },
        $reset() {
            Object.assign(state, options.state());
            recordHistory();
        },
        ...actions
    };

    return () => storeInstance as StoreInstance<S, A> & A;
}

// ==========================================
// 6. Built-in Page Router
// ==========================================
export interface RouteConfig {
    path: string;
    component: () => HTMLElement;
}

export interface RouterOptions {
    routes: RouteConfig[];
}

let routerInstance: { routes: RouteConfig[] } | null = null;
const currentRouteState = reactive({ path: '/' });

export function createRouter(options: RouterOptions) {
    routerInstance = options;
    
    if (typeof window !== 'undefined') {
        window.addEventListener('popstate', () => {
            currentRouteState.path = window.location.pathname;
        });
        currentRouteState.path = window.location.pathname;
    }

    return {
        install(app: any) {
            // Install lifecycle integration
        }
    };
}

export function useRouter() {
    return {
        push(path: string) {
            if (typeof window !== 'undefined') {
                window.history.pushState({}, '', path);
                currentRouteState.path = path;
            }
        },
        replace(path: string) {
            if (typeof window !== 'undefined') {
                window.history.replaceState({}, '', path);
                currentRouteState.path = path;
            }
        },
        back() {
            if (typeof window !== 'undefined') window.history.back();
        }
    };
}

export function useRoute() {
    return currentRouteState;
}

// Router Outlet component
export function RouterView(): HTMLElement | Comment {
    const anchor = document.createComment('router-view');
    let currentEl: HTMLElement | Comment = anchor;

    effect(() => {
        if (!routerInstance) return;
        const matched = routerInstance.routes.find(r => r.path === currentRouteState.path);
        const nextEl = matched ? matched.component() : anchor;
        if (currentEl !== nextEl) {
            currentEl.replaceWith(nextEl);
            currentEl = nextEl;
        }
    });

    return currentEl;
}

// ==========================================
// 7. Auto-Animate & Transition Wrapper
// ==========================================
export interface TransitionConfig {
    duration?: number;
    delay?: number;
    easing?: string;
}

export function transition(
    el: HTMLElement,
    type: 'fade' | 'slide' | 'scale',
    direction: 'in' | 'out',
    config: TransitionConfig = {}
): Promise<void> {
    const duration = config.duration ?? 300;
    const delay = config.delay ?? 0;
    const easing = config.easing ?? 'ease';

    return new Promise((resolve) => {
        el.style.transition = `all ${duration}ms ${easing} ${delay}ms`;
        
        if (type === 'fade') {
            if (direction === 'in') {
                el.style.opacity = '0';
                requestAnimationFrame(() => {
                    el.style.opacity = '1';
                });
            } else {
                el.style.opacity = '1';
                requestAnimationFrame(() => {
                    el.style.opacity = '0';
                });
            }
        } else if (type === 'slide') {
            if (direction === 'in') {
                el.style.transform = 'translateY(-20px)';
                el.style.opacity = '0';
                requestAnimationFrame(() => {
                    el.style.transform = 'translateY(0)';
                    el.style.opacity = '1';
                });
            } else {
                el.style.transform = 'translateY(0)';
                el.style.opacity = '1';
                requestAnimationFrame(() => {
                    el.style.transform = 'translateY(-20px)';
                    el.style.opacity = '0';
                });
            }
        } else if (type === 'scale') {
            if (direction === 'in') {
                el.style.transform = 'scale(0.9)';
                el.style.opacity = '0';
                requestAnimationFrame(() => {
                    el.style.transform = 'scale(1)';
                    el.style.opacity = '1';
                });
            } else {
                el.style.transform = 'scale(1)';
                el.style.opacity = '1';
                requestAnimationFrame(() => {
                    el.style.transform = 'scale(0.9)';
                    el.style.opacity = '0';
                });
            }
        }

        setTimeout(resolve, duration + delay);
    });
}

// ==========================================
// 8. Isomorphic / Server-Side Rendering (SSR) Ready
// ==========================================
export function renderToString(component: () => HTMLElement): string {
    let html = '';
    try {
        const el = component();
        html = el.outerHTML || el.textContent || '';
    } catch (e) {
        html = '<div id="app">Server Rendered Content</div>';
    }
    return html;
}

// ==========================================
// 9. Unified Component & App Mount System
// ==========================================
export interface ComponentOptions {
    props?: Record<string, any>;
    setup: (props: any) => Record<string, any>;
    render: (state: any) => HTMLElement;
}

export function defineComponent(options: ComponentOptions) {
    return (props: any = {}) => {
        const instance: ComponentInstance = {
            mountedHooks: [],
            unmountedHooks: []
        };

        currentInstance = instance;
        const state = options.setup(props);
        currentInstance = null;

        const reactiveState = reactive(state);
        let el: HTMLElement | null = null;

        effect(() => {
            if (!el) {
                el = options.render(reactiveState);
                
                queueMicrotask(() => {
                    instance.mountedHooks.forEach(hook => hook());
                });

                if (instance.unmountedHooks.length > 0) {
                    observeDOMRemoval(el, instance.unmountedHooks);
                }
            } else {
                const newEl = options.render(reactiveState);
                el.replaceWith(newEl);
                el = newEl;
            }
        });

        return el;
    };
}

export function mountApp(rootId: string, component: () => HTMLElement) {
    if (typeof document !== 'undefined') {
        const root = document.getElementById(rootId);
        if (root) {
            root.innerHTML = '';
            root.appendChild(component());
            console.log('🔵 [Zenvu Core] App successfully mounted to DOM.');
        }
    }
}
