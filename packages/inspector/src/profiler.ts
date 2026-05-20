/**
 * @zenvu/inspector - Performance Profiler
 */

export function startProfiler() {
  if (typeof window === 'undefined') return;

  console.log('[Zenvu Profiler] Starting performance tracing...');

  const observer = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      if (entry.entryType === 'measure' && entry.name.startsWith('Zenvu:')) {
        console.debug(`[Zenvu Profiler] âš¡ ${entry.name}: ${entry.duration.toFixed(2)}ms`);
      }
    }
  });

  observer.observe({ entryTypes: ['measure', 'paint', 'longtask'] });
}

export function measure(name: string, fn: () => void) {
  const startMark = `Zenvu:start:${name}`;
  const endMark = `Zenvu:end:${name}`;
  performance.mark(startMark);
  
  fn();
  
  performance.mark(endMark);
  performance.measure(`Zenvu:${name}`, startMark, endMark);
}
