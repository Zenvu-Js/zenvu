import type { SpringConfig } from './transition';

/**
 * 🌀 Zenvu Responsive Animation & Touch Gesture Engine
 * Handles 60fps animations and native swipe/pan gestures.
 */

export class MotionEngine {
    static initGestures(el: HTMLElement, onSwipe: (dir: 'left'|'right'|'up'|'down') => void) {
        let touchStartX = 0;
        let touchStartY = 0;

        el.addEventListener('touchstart', (e) => {
            touchStartX = e.changedTouches[0].screenX;
            touchStartY = e.changedTouches[0].screenY;
        }, { passive: true });

        el.addEventListener('touchend', (e) => {
            const touchEndX = e.changedTouches[0].screenX;
            const touchEndY = e.changedTouches[0].screenY;
            this.handleSwipe(touchStartX, touchStartY, touchEndX, touchEndY, onSwipe);
        }, { passive: true });
    }

    private static handleSwipe(startX: number, startY: number, endX: number, endY: number, cb: Function) {
        const xDiff = endX - startX;
        const yDiff = endY - startY;

        if (Math.abs(xDiff) > Math.abs(yDiff)) {
            if (Math.abs(xDiff) > 50) cb(xDiff > 0 ? 'right' : 'left');
        } else {
            if (Math.abs(yDiff) > 50) cb(yDiff > 0 ? 'down' : 'up');
        }
    }

    static animateResponsive(el: HTMLElement, keyframes: Keyframe[], options: KeyframeAnimationOptions) {
        // Battery-Saving Engine: Drops animation frame rate or skips it entirely if battery is low
        if (document.body.classList.contains('zenvu-battery-saver')) {
            console.warn('ðŸ”‹ Animation skipped to save battery life.');
            // Instantly jump to final state
            if (keyframes.length > 0) {
                Object.assign(el.style, keyframes[keyframes.length - 1]);
            }
            return null;
        }

        // RAM Limiter: Use fallback transitions if device is struggling
        if (document.documentElement.getAttribute('data-zenvu-throttle')) {
            options.duration = 0;
        }

        return el.animate(keyframes, options);
    }
}

export interface AnimateOptions {
    duration?: number;
    easing?: (t: number) => number;
    onUpdate: (value: number) => void;
}

/**
 * Run a smooth, frame-rate independent custom transition.
 */
export function animate(
    from: number,
    to: number,
    options: AnimateOptions
): Promise<void> {
    const { duration = 300, easing = (t: number) => t, onUpdate } = options;
    return new Promise((resolve) => {
        const startTime = performance.now();
        function tick(now: number) {
            const elapsed = now - startTime;
            const progress = Math.min(elapsed / duration, 1);
            const eased = easing(progress);
            const value = from + (to - from) * eased;
            onUpdate(value);
            if (progress < 1) {
                requestAnimationFrame(tick);
            } else {
                onUpdate(to);
                resolve();
            }
        }
        requestAnimationFrame(tick);
    });
}

export interface SpringOptions extends SpringConfig {
    onUpdate: (value: number) => void;
}

/**
 * Run a spring-physics-based fluid animation.
 */
export function spring(
    from: number,
    to: number,
    options: SpringOptions
): Promise<void> {
    const { stiffness = 170, damping = 26, mass = 1, onUpdate } = options;
    return new Promise((resolve) => {
        let velocity = 0;
        let current = from;
        const target = to;
        function tick() {
            const springForce = stiffness * (target - current);
            const damperForce = damping * velocity;
            const acceleration = (springForce - damperForce) / mass;
            velocity += acceleration * (1 / 60);
            current += velocity * (1 / 60);
            onUpdate(current);
            if (Math.abs(velocity) < 0.01 && Math.abs(target - current) < 0.01) {
                onUpdate(target);
                resolve();
            } else {
                requestAnimationFrame(tick);
            }
        }
        requestAnimationFrame(tick);
    });
}

