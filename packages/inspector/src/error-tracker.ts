/**
 * @zenvu/inspector - Error Tracking
 */

export function setupErrorTracking() {
  if (typeof window === 'undefined') return;

  window.addEventListener('error', (event) => {
    console.error('[Zenvu Error Tracker] Uncaught Exception:', {
      message: event.message,
      filename: event.filename,
      lineno: event.lineno,
      error: event.error
    });
    
    // In production, this would beacon to a service like Sentry or LogRocket
    // navigator.sendBeacon('/api/__zenvu_track', JSON.stringify(...));
  });

  window.addEventListener('unhandledrejection', (event) => {
    console.error('[Zenvu Error Tracker] Unhandled Promise Rejection:', event.reason);
  });
}
