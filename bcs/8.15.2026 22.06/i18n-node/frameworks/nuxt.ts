// i18n-node/frameworks/nuxt.ts
import { defineNuxtModule, addVitePlugin } from '@nuxt/kit';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import * as fs from 'fs';
import * as path from 'path';

export interface ModuleOptions {
  localeDir?: string;
  // اضافه کردن امکان دریافت زبان‌ها از API یا به صورت هاردکد
  fetchLocalesApi?: string; 
  hardcodedLocales?: string[];
}

export default defineNuxtModule<ModuleOptions>({
  meta: {
    name: 'i18n-sdk-nuxt',
    configKey: 'i18nSdk',
    compatibility: { nuxt: '>=3.0.0' }
  },
  defaults: {
    localeDir: 'langs' // تغییر به پوشه مرکزی ریشه که اشاره کردید
  },
  async setup(options: any, nuxt: any) {
    addVitePlugin(wasm());
    addVitePlugin(topLevelAwait());

    const virtualModuleId = 'virtual:i18n-sdk-bundles';
    const resolvedVirtualModuleId = '\0' + virtualModuleId;

    // منطق تولید لیست زبان‌ها حین بیلد
    let availableLocales: string[] = [];
    if (options.hardcodedLocales?.length) {
      availableLocales = options.hardcodedLocales;
    } else if (options.fetchLocalesApi) {
      // در صورت وجود API، حین بیلد اطلاعات را فچ می‌کنیم
      const res = await fetch(options.fetchLocalesApi);
      const data = await res.json();
      availableLocales = data.locales; // بر اساس ساختار خروجی API شما
    }

    addVitePlugin({
      name: 'vite-plugin-i18n-sdk-auto-loader',
      resolveId(id: string) {
        if (id === virtualModuleId) return resolvedVirtualModuleId;
      },
      load(id: string) {
        if (id === resolvedVirtualModuleId) {
          const i18nDir = path.resolve(nuxt.options.rootDir, options.localeDir);
          const bundles: Record<string, Record<string, any>> = {};
          
          if (fs.existsSync(i18nDir)) {
            // خواندن خودکار زبان‌ها از نام پوشه‌ها به عنوان Fallback
            const detectedLocales = fs.readdirSync(i18nDir).filter((f: string) => fs.statSync(path.join(i18nDir, f)).isDirectory());
            if (!availableLocales.length) availableLocales = detectedLocales;

            for (const locale of availableLocales) {
              bundles[locale] = {};
              const localePath = path.join(i18nDir, locale);
              if (!fs.existsSync(localePath)) continue;

              const files = fs.readdirSync(localePath).filter((f: string) => f.endsWith('.json'));
              for (const file of files) {
                const namespace = file.replace('.json', '');
                const content = fs.readFileSync(path.join(localePath, file), 'utf-8');
                bundles[locale][namespace] = JSON.parse(content);
              }
            }
          }
          // بازگرداندن باندل‌ها و لیست زبان‌ها به فرانت
          return `
            export const bundles = ${JSON.stringify(bundles)};
            export const locales = ${JSON.stringify(availableLocales)};
          `;
        }
      }
    });

    // تنظیمات OptimizeDeps (مشابه قبل)
    nuxt.options.vite.optimizeDeps = nuxt.options.vite.optimizeDeps || {};
    nuxt.options.vite.optimizeDeps.exclude = nuxt.options.vite.optimizeDeps.exclude || [];
    nuxt.options.vite.optimizeDeps.exclude.push('i18n-sdk');
    nuxt.options.vite.ssr = nuxt.options.vite.ssr || {};
    nuxt.options.vite.ssr.external = nuxt.options.vite.ssr.external || [];
    nuxt.options.vite.ssr.external.push('i18n-sdk');
  }
});