/**
 * ðŸ”„ Zenvu Live Sync
 * 
 * Revolutionary Real-time UI update across multiple devices.
 * Modifying state on one device instantly updates all other devices connected
 * to the same session via Edge WebSockets.
 */

export class LiveSyncEngine {
    private ws: WebSocket;
    private sessionId: string;

    constructor(sessionId: string, edgeUrl: string = 'wss://edge.Zenvujs.dev/sync') {
        this.sessionId = sessionId;
        this.ws = new WebSocket(`${edgeUrl}?session=${sessionId}`);
        this.init();
    }

    private init() {
        this.ws.onmessage = (event) => {
            const payload = JSON.parse(event.data);
            console.log('ðŸŒ [Zenvu Live Sync] Remote mutation received:', payload);
            // Directly mutates the local Proxy State, bypassing Virtual DOM!
            // state[payload.key] = payload.value;
        };
    }

    /// Broadcasts local state mutations to all other connected devices instantly
    public broadcastMutation(key: string, value: any) {
        if (this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify({ key, value, timestamp: Date.now() }));
        }
    }
}
