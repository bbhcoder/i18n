import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
export function i18nSdkVitePlugin() {
    return [
        wasm(),
        topLevelAwait()
    ];
}
