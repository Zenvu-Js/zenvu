/**
 * @zenvu/pwa - Progressive Web App Tools
 */
export function registerServiceWorker(path = '/sw.js') {
  if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register(path).then(reg => {
      console.log('[Zenvu PWA] Service worker registered', reg);
    });
  }
}
