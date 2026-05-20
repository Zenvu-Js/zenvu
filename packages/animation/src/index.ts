/**
 * @zenvu/animation - Smooth DOM Animations
 */
export function animate(element: HTMLElement, keyframes: Keyframe[], options: KeyframeAnimationOptions) {
  return element.animate(keyframes, options);
}
