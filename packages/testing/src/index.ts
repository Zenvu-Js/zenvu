/**
 * @zenvu/testing - Unit Test Utilities
 */
export function renderComponent(component: any, props: any) {
  // Mounts component virtually and returns DOM testing helpers
  return {
    getByText: (text: string) => document.body, // Mock
    click: () => {} // Mock
  };
}
