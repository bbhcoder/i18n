import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import type { PluginOption } from 'vite';

export function i18nSdkVitePlugin(): PluginOption[] {
  return [
    wasm(),
    topLevelAwait()
  ];
}