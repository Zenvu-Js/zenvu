/**
 * Reactive store implementation.
 */

type Subscriber = () => void;

export interface StoreDefinition<S = any> {
  state: () => S;
  actions?: Record<string, (this: StoreInstance<S>, ...args: any[]) => any>;
  getters?: Record<string, (state: S) => any>;
  persist?: boolean;    // LocalStorage persistence
  immutable?: boolean;  // Prevent external direct mutation
}

export interface StoreInstance<S = any> {
  /** Reactive state object. */
  readonly state: S;
  /** Subscribe to state changes. */
  $subscribe(callback: Subscriber): () => void;
  /** Reset state to initial values. */
  $reset(): void;
  /** Patch state partially. */
  $patch(partial: Partial<S>): void;
  /** Time-travel Debugging: Undo last state mutation */
  $undo(): void;
  /** Time-travel Debugging: Redo state mutation */
  $redo(): void;
  /** All actions bound to this store. */
  [key: string]: any;
}

const stores = new Map<string, StoreInstance>();

/** Define a new store. */
export function defineStore<S extends Record<string, any>>(
  id: string,
  definition: StoreDefinition<S>
): () => StoreInstance<S> {
  return () => {
    if (stores.has(id)) {
      return stores.get(id) as StoreInstance<S>;
    }

    const initialState = definition.state();
    let state = { ...initialState };
    
    // Auto-load persisted state
    if (definition.persist && typeof localStorage !== 'undefined') {
      const saved = localStorage.getItem(`zenvu-store-${id}`);
      if (saved) state = JSON.parse(saved);
    }

    const subscribers: Set<Subscriber> = new Set();
    
    // Time-travel history
    let history: string[] = [JSON.stringify(state)];
    let historyIndex = 0;

    const notify = () => {
      // Time-travel snapshotting
      const snap = JSON.stringify(state);
      if (history[historyIndex] !== snap) {
        history = history.slice(0, historyIndex + 1);
        history.push(snap);
        historyIndex++;
      }
      
      // Auto-save persistence
      if (definition.persist && typeof localStorage !== 'undefined') {
        localStorage.setItem(`zenvu-store-${id}`, snap);
      }

      for (const sub of subscribers) {
        try { sub(); } catch (e) { console.error('[Zenvu Store] Subscriber error:', e); }
      }
    };

    // Create reactive proxy
    const reactiveState = new Proxy(state, {
      set(target, prop, value) {
        if (definition.immutable) {
          console.warn(`[Zenvu Store ðŸš¨] Strict Mode: Direct state mutation blocked on '${id}.${String(prop)}'. Use actions/$patch instead.`);
          return false;
        }
        (target as any)[prop as string] = value;
        notify();
        return true;
      },
    });

    const instance: StoreInstance<S> = {
      get state() { return reactiveState as S; },
      $subscribe(callback: Subscriber) {
        subscribers.add(callback);
        return () => subscribers.delete(callback);
      },
      $reset() {
        Object.assign(state, definition.state());
        notify();
      },
      $patch(partial: Partial<S>) {
        Object.assign(state, partial);
        notify();
      },
      $undo() {
        if (historyIndex > 0) {
          historyIndex--;
          Object.assign(state, JSON.parse(history[historyIndex]));
          // Notify without triggering a new snapshot
          for (const sub of subscribers) sub();
        }
      },
      $redo() {
        if (historyIndex < history.length - 1) {
          historyIndex++;
          Object.assign(state, JSON.parse(history[historyIndex]));
          for (const sub of subscribers) sub();
        }
      },
    };

    // Bind actions
    if (definition.actions) {
      for (const [name, action] of Object.entries(definition.actions)) {
        (instance as any)[name] = (...args: any[]) => action.apply(instance, args);
      }
    }

    // Bind getters
    if (definition.getters) {
      for (const [name, getter] of Object.entries(definition.getters)) {
        Object.defineProperty(instance, name, {
          get: () => getter(reactiveState as S),
          enumerable: true,
        });
      }
    }

    stores.set(id, instance);
    return instance;
  };
}

/** Get or create a store instance by ID. */
export function useStore<S = any>(id: string): StoreInstance<S> {
  const store = stores.get(id);
  if (!store) throw new Error(`[Zenvu Store] Store '${id}' not defined. Call defineStore() first.`);
  return store as StoreInstance<S>;
}
