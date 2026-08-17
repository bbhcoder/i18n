// i18n-node/server.d.ts

export class I18nEngine {
  constructor(defaultLocale: string, fallbackLocale?: string);
  setLocale(locale: string): void;
  setFallbackLocale(locale: string): void;
  addBundle(locale: string, namespace: string, jsonString: string): void;
  addMedia(locale: string, namespace: string, key: string, url: string): void;
  getMedia(namespace: string, key: string): string | undefined;
  t(namespace: string, key: string, argsJson?: string): string;
  version(): string;
}