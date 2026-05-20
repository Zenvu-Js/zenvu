/**
 * @zenvu/plugin-markdown
 * Parses markdown files into Zenvu.js interactive components at compile-time.
 */
export function markdownPlugin() {
  return {
    name: 'zenvu-plugin-markdown',
    transform(code: string, id: string) {
      if (id.endsWith('.md')) {
        // Compile markdown to VNodes
        return `export default function() { return /* compiled markdown */ }`;
      }
    }
  };
}
