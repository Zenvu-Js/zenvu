/**
 * Client-side session management.
 */

export class SessionClient {
  private sessionId: string | null = null;

  /** Check if there's an active session. */
  isAuthenticated(): boolean {
    return this.getSessionId() !== null;
  }

  /** Get the current session ID from cookies. */
  getSessionId(): string | null {
    if (this.sessionId) return this.sessionId;
    const match = document.cookie.match(/__zenvu_session=([^;]+)/);
    this.sessionId = match ? match[1] : null;
    return this.sessionId;
  }

  /** Logout â€” destroy the session. */
  async logout(logoutUrl = '/api/auth/logout'): Promise<void> {
    await fetch(logoutUrl, {
      method: 'POST',
      credentials: 'same-origin',
      headers: { 'X-CSRF-Token': this.getCsrfToken() },
    });
    this.sessionId = null;
    // Clear cookie client-side as well
    document.cookie = '__zenvu_session=; Max-Age=0; Path=/; SameSite=Strict';
  }

  private getCsrfToken(): string {
    const meta = document.querySelector('meta[name="csrf-token"]');
    return meta?.getAttribute('content') || '';
  }
}
