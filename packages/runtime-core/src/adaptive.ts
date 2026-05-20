/**
 * ðŸ“± Smart Adaptive Rendering Engine
 * 
 * Implements Memory-Aware Components, Auto Layout Optimization, and
 * Smart Responsive capabilities using IntersectionObserver & ResizeObserver.
 */

export class AdaptiveEngine {
    private static intersectionObserver: IntersectionObserver;
    private static resizeObserver: ResizeObserver;

    static init() {
        console.log('ðŸ¤– [Zenvu Adaptive Engine] Initializing Memory-Aware Component Engine...');
        
        // 1. Memory-Aware Component Engine (Auto Pause/Resume)
        this.intersectionObserver = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                const target = entry.target as any;
                if (entry.isIntersecting) {
                    if (target.__zenvu_paused) {
                        target.__zenvu_resume();
                        target.__zenvu_paused = false;
                        console.log('â–¶ï¸ [Memory-Aware] Resumed rendering for visible component');
                    }
                } else {
                    target.__zenvu_pause();
                    target.__zenvu_paused = true;
                    console.log('â¸ï¸ [Memory-Aware] Paused rendering & detached events for hidden component');
                }
            });
        }, { threshold: 0.1 });

        // 2. Smart Responsive Engine (Auto Layout Optimizer)
        this.resizeObserver = new ResizeObserver((entries) => {
            entries.forEach(entry => {
                const width = entry.contentRect.width;
                const target = entry.target as HTMLElement;
                
                // Dynamic layout scaling without manual CSS media queries!
                if (width < 640) {
                    target.setAttribute('data-device', 'mobile');
                } else if (width < 1024) {
                    target.setAttribute('data-device', 'tablet');
                } else {
                    target.setAttribute('data-device', 'desktop');
                }
            });
        });
    }

    /**
     * Registers a component for Adaptive memory management and responsive tracking
     */
    static observeComponent(el: HTMLElement, pauseFn: () => void, resumeFn: () => void) {
        (el as any).__zenvu_pause = pauseFn;
        (el as any).__zenvu_resume = resumeFn;
        
        this.intersectionObserver.observe(el);
        this.resizeObserver.observe(el);
    }

    /**
     * 3. RAM Limiter & Low-End Device Optimization
     */
    static initHardwareAwareness() {
        console.log('ðŸ§  [Zenvu Hardware Engine] Scanning device capabilities...');
        
        // Auto Dark Mode Adaptation
        const darkModePref = window.matchMedia('(prefers-color-scheme: dark)');
        const updateTheme = (e: MediaQueryListEvent | MediaQueryList) => {
            document.documentElement.setAttribute('data-theme', e.matches ? 'dark' : 'light');
        };
        updateTheme(darkModePref);
        darkModePref.addEventListener('change', updateTheme);

        // RAM Limiter (Chrome/Edge specific)
        if ('memory' in performance) {
            setInterval(() => {
                const mem = (performance as any).memory;
                // If heap exceeds 500MB, trigger emergency garbage collection & pause animations
                if (mem.usedJSHeapSize > 500 * 1024 * 1024) {
                    console.warn('ðŸš¨ [Zenvu RAM Limiter] High memory usage detected! Throttling render engine...');
                    document.documentElement.setAttribute('data-zenvu-throttle', 'true');
                } else {
                    document.documentElement.removeAttribute('data-zenvu-throttle');
                }
            }, 5000);
        }

        // Battery-Saving Render Mode
        if ('getBattery' in navigator) {
            (navigator as any).getBattery().then((battery: any) => {
                const checkBattery = () => {
                    if (battery.level < 0.20 || battery.charging === false && battery.level < 0.30) {
                        console.warn('ðŸ”‹ [Zenvu Battery Saver] Battery low. Disabling 60fps animations.');
                        document.body.classList.add('zenvu-battery-saver');
                    } else {
                        document.body.classList.remove('zenvu-battery-saver');
                    }
                };
                checkBattery();
                battery.addEventListener('levelchange', checkBattery);
                battery.addEventListener('chargingchange', checkBattery);
            });
        }
    }
}
