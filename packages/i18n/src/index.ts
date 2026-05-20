/**
 * @zenvu/i18n - Multi Language Support
 */
export function createI18n(messages: Record<string, any>, defaultLocale = 'en') {
  let locale = defaultLocale;
  return {
    t: (key: string) => messages[locale][key] || key,
    setLocale: (l: string) => { locale = l; }
  };
}
