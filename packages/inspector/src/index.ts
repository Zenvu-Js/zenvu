/**
 * @zenvu/inspector - Browser Component Debugger Overlay
 */

interface InspectorState {
  enabled: boolean;
  activeElement: HTMLElement | null;
  overlay: HTMLElement | null;
  highlightBox: HTMLElement | null;
  tooltip: HTMLElement | null;
}

const state: InspectorState = {
  enabled: false,
  activeElement: null,
  overlay: null,
  highlightBox: null,
  tooltip: null,
};

/**
 * Injects the Inspector UI into the DOM.
 */
export function mountInspector() {
  if (typeof window === 'undefined') return;
  if (document.getElementById('zenvu-inspector-root')) return;

  // Global Toggle Button
  const root = document.createElement('div');
  root.id = 'zenvu-inspector-root';
  root.style.cssText = `
    position: fixed;
    bottom: 24px;
    right: 24px;
    z-index: 2147483647; /* Max z-index */
    font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  `;

  const btn = document.createElement('button');
  btn.innerHTML = `
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="10"></circle>
      <line x1="12" y1="16" x2="12" y2="12"></line>
      <line x1="12" y1="8" x2="12.01" y2="8"></line>
    </svg>
    <span style="margin-left: 8px; font-weight: 600;">Inspect</span>
  `;
  btn.style.cssText = `
    display: flex;
    align-items: center;
    background: #1e293b;
    color: #38bdf8;
    border: 1px solid #334155;
    padding: 10px 16px;
    border-radius: 9999px;
    cursor: pointer;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    transition: all 0.2s ease;
  `;

  btn.onmouseover = () => btn.style.background = '#334155';
  btn.onmouseout = () => btn.style.background = '#1e293b';
  btn.onclick = toggleInspector;

  root.appendChild(btn);
  document.body.appendChild(root);

  // Highlight Box (Shows element boundaries)
  state.highlightBox = document.createElement('div');
  state.highlightBox.style.cssText = `
    position: fixed;
    pointer-events: none;
    z-index: 2147483646;
    background: rgba(56, 189, 248, 0.2);
    border: 1px dashed #38bdf8;
    display: none;
    transition: all 0.1s ease-out;
  `;
  document.body.appendChild(state.highlightBox);

  // Tooltip (Shows component info)
  state.tooltip = document.createElement('div');
  state.tooltip.style.cssText = `
    position: fixed;
    pointer-events: none;
    z-index: 2147483647;
    background: #0f172a;
    color: #f8fafc;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 12px;
    box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
    border: 1px solid #334155;
    display: none;
    white-space: pre;
  `;
  document.body.appendChild(state.tooltip);
}

function toggleInspector() {
  state.enabled = !state.enabled;
  const btn = document.querySelector('#zenvu-inspector-root button') as HTMLElement;
  
  if (state.enabled) {
    btn.style.color = '#10b981'; // Green
    btn.style.borderColor = '#10b981';
    document.addEventListener('mousemove', onMouseMove, true);
    document.addEventListener('click', onMouseClick, true);
  } else {
    btn.style.color = '#38bdf8'; // Zenvue
    btn.style.borderColor = '#334155';
    document.removeEventListener('mousemove', onMouseMove, true);
    document.removeEventListener('click', onMouseClick, true);
    if (state.highlightBox) state.highlightBox.style.display = 'none';
    if (state.tooltip) state.tooltip.style.display = 'none';
  }
}

function onMouseMove(e: MouseEvent) {
  if (!state.enabled) return;

  const target = document.elementFromPoint(e.clientX, e.clientY) as HTMLElement;
  if (!target || target.closest('#zenvu-inspector-root')) return;

  if (state.activeElement !== target) {
    state.activeElement = target;
    const rect = target.getBoundingClientRect();

    // Update Highlight Box
    if (state.highlightBox) {
      state.highlightBox.style.display = 'block';
      state.highlightBox.style.top = `${rect.top}px`;
      state.highlightBox.style.left = `${rect.left}px`;
      state.highlightBox.style.width = `${rect.width}px`;
      state.highlightBox.style.height = `${rect.height}px`;
    }

    // Determine Component Name (In Zenvu, we compile data-b-cid attributes)
    const componentId = target.getAttribute('data-b-cid') || target.tagName.toLowerCase();
    
    // Update Tooltip
    if (state.tooltip) {
      state.tooltip.style.display = 'block';
      state.tooltip.style.top = `${rect.bottom + 8}px`;
      state.tooltip.style.left = `${rect.left}px`;
      state.tooltip.innerHTML = `
<span style="color: #c678dd;">&lt;${componentId}&gt;</span>
<span style="color: #61afef;">width:</span> ${Math.round(rect.width)}px
<span style="color: #61afef;">height:</span> ${Math.round(rect.height)}px
      `.trim();
    }
  }
}

function onMouseClick(e: MouseEvent) {
  if (!state.enabled) return;
  e.preventDefault();
  e.stopPropagation();

  const target = state.activeElement;
  if (target) {
    console.log('%c[Zenvu Inspector] Component Selected:', 'color: #38bdf8; font-weight: bold;', target);
    // In a real devtools, this would dispatch a CustomEvent to the Chrome Extension
    // window.dispatchEvent(new CustomEvent('zenvu-devtools-select', { detail: { id: componentId } }));
  }
}
