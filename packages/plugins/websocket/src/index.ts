/**
 * @zenvu/plugin-websocket
 * Real-time event streams mapped to Zenvu.js reactive state.
 */
export function useWebSocket(url: string) {
  return {
    connect: () => new WebSocket(url),
    onMessage: (cb: Function) => {}
  };
}
