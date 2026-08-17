/* tslint:disable */
/* eslint-disable */

export class I18nEngine {
    free(): void;
    [Symbol.dispose](): void;
    addBundle(locale: string, namespace: string, json_string: string): void;
    addMedia(locale: string, namespace: string, key: string, url: string): void;
    getMedia(namespace: string, key: string): string | undefined;
    help(): string;
    constructor(default_locale: string, fallback_locale?: string | null);
    setDebugMode(enabled: boolean): void;
    setFallbackLocale(locale: string): void;
    setLocale(locale: string): void;
    t(namespace: string, key: string, args_json?: string | null): string;
    version(): string;
}

export class WasmCompiledTemplate {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    render(args_js: any): string;
}
