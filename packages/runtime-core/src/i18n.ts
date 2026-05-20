/**
 * ðŸŒ Zenvu Native i18n Engine
 * High-performance localization with proxy-based reactive translation strings.
 */

export class I18nEngine {
    private locale: string = 'en';
    private messages: Record<string, Record<string, string>> = {};

    constructor(defaultLocale: string, messages: Record<string, Record<string, string>>) {
        this.locale = defaultLocale;
        this.messages = messages;
    }

    setLocale(lang: string) {
        if (this.messages[lang]) {
            this.locale = lang;
            document.documentElement.lang = lang;
            console.log(`ðŸŒ [Zenvu i18n] Switched to ${lang}`);
        }
    }

    t(key: string, params?: Record<string, string>): string {
        let text = this.messages[this.locale]?.[key] || key;
        if (params) {
            Object.keys(params).forEach(p => {
                text = text.replace(`{${p}}`, params[p]);
            });
        }
        return text;
    }
}
