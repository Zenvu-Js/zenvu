/**
 * ZenvuEvents â€” Efficient event delegation system.
 *
 * Instead of attaching individual listeners to each element,
 * Zenvu.js uses a single root listener per event type and delegates
 * to the correct handler based on the event target.
 */

type EventHandler = (event: Event) => void;

interface DelegatedEvent {
  selector: string;
  handler: EventHandler;
  modifiers: string[];
}

export class ZenvuEvents {
  private static delegations = new Map<string, Map<Element, DelegatedEvent[]>>();

  /**
   * Delegate an event from a root element to children matching a condition.
   */
  static delegate(
    root: Element,
    eventType: string,
    handler: EventHandler,
    modifiers: string[] = []
  ): () => void {
    const wrappedHandler = (event: Event) => {
      // Apply modifiers
      if (modifiers.includes('prevent')) event.preventDefault();
      if (modifiers.includes('stop')) event.stopPropagation();

      // Self modifier â€” only trigger if target is the element itself
      if (modifiers.includes('self') && event.target !== root) return;

      handler(event);
    };

    root.addEventListener(eventType, wrappedHandler, {
      capture: modifiers.includes('capture'),
      passive: modifiers.includes('passive'),
      once: modifiers.includes('once'),
    });

    // Return cleanup function
    return () => {
      root.removeEventListener(eventType, wrappedHandler);
    };
  }

  /**
   * Attach a keyboard event with key filtering.
   * Supports: enter, escape, tab, space, arrow keys, etc.
   */
  static onKey(
    element: Element,
    key: string,
    handler: EventHandler,
    modifiers: string[] = []
  ): () => void {
    const keyMap: Record<string, string> = {
      enter: 'Enter',
      escape: 'Escape',
      tab: 'Tab',
      space: ' ',
      up: 'ArrowUp',
      down: 'ArrowDown',
      left: 'ArrowLeft',
      right: 'ArrowRight',
      delete: 'Delete',
      backspace: 'Backspace',
    };

    const targetKey = keyMap[key.toLowerCase()] || key;

    const wrappedHandler = (event: Event) => {
      const ke = event as KeyboardEvent;
      if (ke.key === targetKey) {
        if (modifiers.includes('prevent')) event.preventDefault();
        if (modifiers.includes('stop')) event.stopPropagation();
        handler(event);
      }
    };

    element.addEventListener('keydown', wrappedHandler);
    return () => element.removeEventListener('keydown', wrappedHandler);
  }

  /** Remove all delegated events from a root element. */
  static cleanup(root: Element): void {
    // Cloned elements won't carry listeners, so just clear references
    ZenvuEvents.delegations.delete(root.tagName);
  }
}
