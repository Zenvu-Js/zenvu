/**
 * @zenvu/runtime-core - Web Component Support
 */

import { mount, h } from './index';

/**
 * Wraps a Zenvu component into a native Web Component (Custom Element).
 * Allows exporting Zenvu components to be used in React, Vue, or Vanilla JS.
 */
export function defineCustomElement(ZenvuComponent: Function, tagName?: string): CustomElementConstructor {
  const CustomElement = class extends HTMLElement {
    private isMounted = false;

    constructor() {
      super();
      this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
      if (!this.isMounted && this.shadowRoot) {
        // Collect attributes as props
        const props: Record<string, string> = {};
        for (const attr of this.attributes) {
          props[attr.name] = attr.value;
        }

        // Mount the Zenvu component inside the Shadow DOM
        const vnode = h(ZenvuComponent, props);
        mount(vnode, this.shadowRoot as any);
        this.isMounted = true;
      }
    }

    disconnectedCallback() {
      // Cleanup hooks would go here
    }
  };

  // Register automatically if a tag name is provided
  if (tagName && typeof customElements !== 'undefined') {
    customElements.define(tagName, CustomElement);
  }

  return CustomElement;
}
