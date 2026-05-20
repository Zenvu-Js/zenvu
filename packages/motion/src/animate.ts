/**
 * ðŸŒ€ Zenvu Responsive Animation & Touch Gesture Engine
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
