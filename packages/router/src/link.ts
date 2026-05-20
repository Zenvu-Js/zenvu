/**
 * Router link component — creates accessible navigation links.
 */
import { navigate } from './router';

export function createLink(target: Element, props: { to: string; class?: string }): any {
  const a = document.createElement('a');
  a.href = props.to;
  a.className = props.class || '';
  a.addEventListener('click', (e) => {
    e.preventDefault();
    navigate(props.to);
  });
  target.appendChild(a);
  return { $destroy: () => a.remove(), el: a };
}
