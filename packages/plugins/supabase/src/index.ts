/**
 * @zenvu/plugin-supabase
 * Seamless Supabase integration with Zenvu.js Reactivity.
 */
import { createClient } from '@supabase/supabase-js';

export function createSupabasePlugin(url: string, key: string) {
  const supabase = createClient(url, key);
  return {
    provide: { supabase }
  };
}
