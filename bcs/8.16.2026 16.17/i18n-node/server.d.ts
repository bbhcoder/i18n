// i18n-node/server.d.ts

// اضافه کردن تایپ‌های تولید شده خودکار برای ولیدیتور VSCode
// @ts-ignore
import type { I18nNamespace, I18nKey } from '#build/i18n-schema';

export class I18nEngine {
  constructor(defaultLocale: string, fallbackLocale?: string);
  
  setLocale(locale: string): void;
  setFallbackLocale(locale: string): void;
  
  addBundle(locale: string, namespace: string, jsonString: string): void;
  addMedia(locale: string, namespace: string, key: string, url: string): void;
  getMedia(namespace: string, key: string): string | undefined;
  
  // متد جدید که در لایه Node.js پیاده‌سازی کردیم
  scanMediaFolderAsync(namespace: string, rootPath: string, baseUrl: string): Promise<void>;
  
  // اتصال متد t به سیستم تایپ داینامیک
  t<Ns extends I18nNamespace>(namespace: Ns, key: I18nKey<Ns>, argsJson?: string): string;
  
  version(): string;
  
  // متدهای مربوط به دیباگ و سیستم Help
  setDebugMode(enabled: boolean): void;
  diagnostics(): string;

  help(): string;
}

export class JsCompiledTemplate {
  constructor();
  render(args?: Record<string, string>): string;
}