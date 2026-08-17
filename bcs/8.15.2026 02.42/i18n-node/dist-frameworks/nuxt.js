// i18n-node/frameworks/nuxt.ts
import { defineNuxtModule, addVitePlugin } from '@nuxt/kit';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import * as fs from 'fs';
import * as path from 'path';
export default defineNuxtModule({
    meta: {
        name: 'i18n-sdk-nuxt',
        configKey: 'i18nSdk',
        compatibility: { nuxt: '>=3.0.0' }
    },
    defaults: {
        localeDir: 'i18n'
    },
    setup(options, nuxt) {
        addVitePlugin(wasm());
        addVitePlugin(topLevelAwait());
        const virtualModuleId = 'virtual:i18n-sdk-bundles';
        const resolvedVirtualModuleId = '\0' + virtualModuleId;
        addVitePlugin({
            name: 'vite-plugin-i18n-sdk-auto-loader',
            resolveId(id) {
                if (id === virtualModuleId)
                    return resolvedVirtualModuleId;
            },
            load(id) {
                if (id === resolvedVirtualModuleId) {
                    const i18nDir = path.resolve(nuxt.options.rootDir, options.localeDir);
                    const bundles = {};
                    if (fs.existsSync(i18nDir)) {
                        // 💥 اضافه کردن (f: string) برای رفع خطای تایپ‌اسکریپت
                        const locales = fs.readdirSync(i18nDir).filter((f) => fs.statSync(path.join(i18nDir, f)).isDirectory());
                        for (const locale of locales) {
                            bundles[locale] = {};
                            const localePath = path.join(i18nDir, locale);
                            // 💥 اضافه کردن (f: string)
                            const files = fs.readdirSync(localePath).filter((f) => f.endsWith('.json'));
                            for (const file of files) {
                                const namespace = file.replace('.json', '');
                                const content = fs.readFileSync(path.join(localePath, file), 'utf-8');
                                try {
                                    bundles[locale][namespace] = JSON.parse(content);
                                } catch (e) {
                                    console.error(`\n❌ Error parsing JSON in file: ${localePath}/${file}\n`);
                                    throw e;
                                }
                            }
                        }
                    }
                    return `export default ${JSON.stringify(bundles)};`;
                }
            }
        });
        nuxt.options.vite.optimizeDeps = nuxt.options.vite.optimizeDeps || {};
        nuxt.options.vite.optimizeDeps.exclude = nuxt.options.vite.optimizeDeps.exclude || [];
        nuxt.options.vite.optimizeDeps.exclude.push('i18n-sdk');
        nuxt.options.vite.ssr = nuxt.options.vite.ssr || {};
        nuxt.options.vite.ssr.external = nuxt.options.vite.ssr.external || [];
        nuxt.options.vite.ssr.external.push('i18n-sdk');
    }
});
