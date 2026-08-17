// frameworks/nuxt.ts
import { defineNuxtModule, addVitePlugin } from '@nuxt/kit';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
export default defineNuxtModule({
    meta: {
        name: 'i18n-sdk-nuxt',
        configKey: 'i18nSdk',
        compatibility: {
            nuxt: '>=3.0.0'
        }
    },
    setup(_options, nuxt) {
        addVitePlugin(wasm());
        addVitePlugin(topLevelAwait());
        // برای کلاینت: جلوگیری از باگ‌های بهینه‌سازی Vite
        nuxt.options.vite.optimizeDeps = nuxt.options.vite.optimizeDeps || {};
        nuxt.options.vite.optimizeDeps.exclude = nuxt.options.vite.optimizeDeps.exclude || [];
        nuxt.options.vite.optimizeDeps.exclude.push('i18n-sdk');
        // 💥 راه حل طلایی برای سرور: به Vite می‌گیم تو محیط SSR کدهای ما رو باندل نکن!
        nuxt.options.vite.ssr = nuxt.options.vite.ssr || {};
        nuxt.options.vite.ssr.external = nuxt.options.vite.ssr.external || [];
        nuxt.options.vite.ssr.external.push('i18n-sdk');
    }
});
