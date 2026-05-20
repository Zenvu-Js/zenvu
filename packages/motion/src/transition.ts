/**
 * Built-in transition presets.
 */

export interface TransitionConfig {
  duration?: number;
  delay?: number;
  easing?: string;
  css?: (t: number) => string;
}

export interface SpringConfig {
  stiffness?: number;
  damping?: number;
  mass?: number;
}

type TransitionFn = (node: Element, config?: TransitionConfig) => {
  enter: () => Promise<void>;
  leave: () => Promise<void>;
};

/** Fade in/out transition. */
export function fade(config: TransitionConfig = {}): TransitionFn {
  const { duration = 300, delay = 0, easing = 'ease' } = config;
  return (node: Element) => ({
    async enter() {
      const el = node as HTMLElement;
      el.style.transition = `opacity ${duration}ms ${easing} ${delay}ms`;
      el.style.opacity = '0';
      await frame();
      el.style.opacity = '1';
      await wait(duration + delay);
    },
    async leave() {
      const el = node as HTMLElement;
      el.style.transition = `opacity ${duration}ms ${easing} ${delay}ms`;
      el.style.opacity = '0';
      await wait(duration + delay);
    },
  });
}

/** Slide in/out transition. */
export function slide(config: TransitionConfig & { y?: number; x?: number } = {}): TransitionFn {
  const { duration = 300, delay = 0, easing = 'ease', y = -20, x = 0 } = config;
  return (node: Element) => ({
    async enter() {
      const el = node as HTMLElement;
      el.style.transition = `transform ${duration}ms ${easing} ${delay}ms, opacity ${duration}ms ${easing} ${delay}ms`;
      el.style.transform = `translate(${x}px, ${y}px)`;
      el.style.opacity = '0';
      await frame();
      el.style.transform = 'translate(0, 0)';
      el.style.opacity = '1';
      await wait(duration + delay);
    },
    async leave() {
      const el = node as HTMLElement;
      el.style.transition = `transform ${duration}ms ${easing} ${delay}ms, opacity ${duration}ms ${easing} ${delay}ms`;
      el.style.transform = `translate(${x}px, ${y}px)`;
      el.style.opacity = '0';
      await wait(duration + delay);
    },
  });
}

/** Scale in/out transition. */
export function scale(config: TransitionConfig & { start?: number } = {}): TransitionFn {
  const { duration = 300, delay = 0, easing = 'ease', start = 0.8 } = config;
  return (node: Element) => ({
    async enter() {
      const el = node as HTMLElement;
      el.style.transition = `transform ${duration}ms ${easing} ${delay}ms, opacity ${duration}ms ${easing} ${delay}ms`;
      el.style.transform = `scale(${start})`;
      el.style.opacity = '0';
      await frame();
      el.style.transform = 'scale(1)';
      el.style.opacity = '1';
      await wait(duration + delay);
    },
    async leave() {
      const el = node as HTMLElement;
      el.style.transition = `transform ${duration}ms ${easing} ${delay}ms, opacity ${duration}ms ${easing} ${delay}ms`;
      el.style.transform = `scale(${start})`;
      el.style.opacity = '0';
      await wait(duration + delay);
    },
  });
}

/** Fly transition — combines translate and opacity. */
export function fly(config: TransitionConfig & { x?: number; y?: number } = {}): TransitionFn {
  return slide({ ...config, y: config.y ?? -50, x: config.x ?? 0 });
}

/** Draw transition for SVG paths. */
export function draw(config: TransitionConfig = {}): TransitionFn {
  const { duration = 800, delay = 0, easing = 'ease' } = config;
  return (node: Element) => ({
    async enter() {
      if (node instanceof SVGPathElement) {
        const length = node.getTotalLength();
        node.style.strokeDasharray = `${length}`;
        node.style.strokeDashoffset = `${length}`;
        node.style.transition = `stroke-dashoffset ${duration}ms ${easing} ${delay}ms`;
        await frame();
        node.style.strokeDashoffset = '0';
        await wait(duration + delay);
      }
    },
    async leave() {
      if (node instanceof SVGPathElement) {
        const length = node.getTotalLength();
        node.style.transition = `stroke-dashoffset ${duration}ms ${easing} ${delay}ms`;
        node.style.strokeDashoffset = `${length}`;
        await wait(duration + delay);
      }
    },
  });
}

function frame(): Promise<void> {
  return new Promise(r => requestAnimationFrame(() => requestAnimationFrame(() => r())));
}

function wait(ms: number): Promise<void> {
  return new Promise(r => setTimeout(r, ms));
}
