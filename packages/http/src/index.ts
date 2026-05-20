/**
 * @zenvu/http - Fetch API Client
 */
export async function useFetch<T>(url: string, options?: RequestInit): Promise<T> {
  const res = await fetch(url, options);
  if (!res.ok) throw new Error('HTTP Error');
  return res.json();
}
