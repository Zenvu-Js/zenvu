/**
 * @zenvu/runtime-core - Async Component / Lazy Loading
 */

import { VNode, h, mount } from './index';

export type AsyncComponentLoader = () => Promise<{ default: Function } | Function>;

interface AsyncComponentOptions {
  loader: AsyncComponentLoader;
  loadingComponent?: Function;
  errorComponent?: Function;
  delay?: number;
  timeout?: number;
}

/**
 * Define an asynchronous component that lazy-loads its implementation.
 * Enables automatic code-splitting at the bundler level via `import()`.
 */
export function defineAsyncComponent(source: AsyncComponentLoader | AsyncComponentOptions): Function {
  const options = typeof source === 'function' ? { loader: source } : source;
  
  let resolvedComp: Function | null = null;
  let error: Error | null = null;
  
  return (props: any): VNode => {
    // If already resolved, return it directly
    if (resolvedComp) return h(resolvedComp, props);
    
    // If error, show error component
    if (error && options.errorComponent) return h(options.errorComponent, { error });

    // Start loading
    options.loader()
      .then(m => {
        // Handle ES modules default export
        resolvedComp = typeof m === 'object' && m.default ? m.default : m as Function;
        // In a real reactive framework, we would trigger a re-render here
        // triggerReRender(currentInstance);
      })
      .catch(err => {
        error = err;
        console.error('[Zenvu] Async component failed to load', err);
      });
    
    // Return loading placeholder
    if (options.loadingComponent) {
      return h(options.loadingComponent, props);
    }
    
    return h('div', { 'data-zenvu-async-loading': true });
  };
}
