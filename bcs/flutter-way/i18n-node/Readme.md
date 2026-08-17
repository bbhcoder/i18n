# @encheco/i18n🚀

[![npm version](https://img.shields.io/npm/v/@encheco/i18n.svg)](https://www.npmjs.com/package/@encheco/i18n)
[![npm downloads](https://img.shields.io/npm/dm/@encheco/i18n.svg)](https://www.npmjs.com/package/@encheco/i18n)
[![license](https://img.shields.io/npm/l/@encheco/i18n.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/core-Rust-orange.svg)](https://www.rust-lang.org/)

A blazing fast, zero-lookup Internationalization (i18n) engine powered by Rust.
Built with **N-API** for Node.js (SSR) and **WebAssembly** for the browser, bringing systems-engineering performance to your JavaScript frontend.

At the heart of the library sits the **`I18NEngine`** — a single, unified Rust core that compiles to a native Node addon *and* to WASM, exposing the exact same API on both sides.

---

## Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Supported Platforms](#-supported-platforms)
- [Quick Start (Node.js)](#-quick-start-nodejs)
- [ESM Usage](#-esm-usage)
- [Browser Usage (WebAssembly)](#-browser-usage-webassembly)
- [Usage with Nuxt 3](#-usage-with-nuxt-3)
- [Usage with Vite](#-usage-with-vite)
- [Async Media Management](#️-async-media-management-nodejs)
- [Diagnostics, Debugging & Help](#-diagnostics-debugging--help)
- [Pluralization](#-pluralization)
- [API Reference](#-api-reference)
- [How It Works (The Rust Core)](#️-how-it-works-the-rust-core)
- [Comparison with Other i18n Libraries](#-comparison-with-other-i18n-libraries)
- [License](#-license)

---

## ✨ Features

* **Blazing Fast (Zero-Lookup):** Compiles translation strings into memory tokens via a Rust AST. No heavy Regex or string manipulation at runtime.
* **Universal Architecture:** The `I18NEngine` runs at native speed on Node.js (via N-API prebuilt binaries) and in the browser (via WebAssembly) — same core, same API, everywhere.
* **Dual Module Support:** Ships with both CommonJS (`server.js`) and native ESM (`server.mjs`) entry points, wired up correctly through `package.json#exports`.
* **Auto-generated Type Safety:** The Nuxt module scans your locale JSON files and writes a fully-typed `i18n-schema.d.ts` into `.nuxt/`, giving you autocomplete on both namespaces and keys via `I18NEngine.t<Ns>()`.
* **Native Plurals:** Full CLDR-compliant pluralization engine written directly in the Rust core, covering all six plural categories (`zero`, `one`, `two`, `few`, `many`, `other`) across 60+ locales — including fully-specced rules for Arabic, the East Slavic and West Slavic families, Hebrew, Baltic, Celtic, and more (see [Pluralization](#-pluralization) below).
* **Smart Media Management:** Scan a directory of localized media assets and resolve the correct file per locale — synchronously or asynchronously.
* **Built-in Diagnostics:** Toggle debug mode to catch missing keys, and print a memory report of every cached bundle straight from the Rust backend.
* **Self-documenting:** Call `engine.help()` at any time for a quick in-code API tour.
* **Framework Agnostic:** The core is standalone, with official, pre-wired integrations for **Nuxt 3** and **Vite**.

---

## 📦 Installation

```bash
npm install @encheco/i18n
```

> On install, @encheco/i18n prints a short tip reminding you that `engine.help()` is available for a quick API tour.

---

## 🖥️ Supported Platforms

The package ships prebuilt native binaries — no local Rust toolchain needed:

| OS | Architecture | libc | Binary |
|---|---|---|---|
| Linux | x64 | glibc | `index.linux-x64-gnu.node` |
| Linux | x64 | musl (Alpine) | `index.linux-x64-musl.node` |
| Windows | x64 | MSVC | `index.win32-x64-msvc.node` |

The loader auto-detects musl vs. glibc at runtime by inspecting `/etc/os-release` and `ldd`, and throws a clear `Unsupported OS/Arch` error on any other platform. For the browser, use the WebAssembly build instead (see below) — it's platform-independent.

> **Note:** there is currently no native macOS (`darwin`) binary in this build. macOS users should run through WASM or build from source.

---

## 🚀 Quick Start (Node.js)

```javascript
const { I18NEngine } = require('@encheco/i18n');

const engine = new I18NEngine('en', 'en'); // (defaultLocale, fallbackLocale)

engine.addBundle('en', 'home', JSON.stringify({ welcome: 'Hello {name}' }));

const greeting = engine.t('home', 'welcome', JSON.stringify({ name: 'Arsalan' }));
console.log(greeting); // "Hello Arsalan"
```

`t()` is fully typed once you're using the Nuxt integration (or hand-roll your own `I18nNamespace` / `I18nKey` types) — the namespace argument narrows which keys are valid for `key`.

---

## 📦 ESM Usage

`@encheco/i18n` also ships a native ESM build (`server.mjs`), resolved automatically via `import`:

```javascript
import { I18NEngine, JsCompiledTemplate } from '@encheco/i18n';

const engine = new I18NEngine('en');
engine.addBundle('en', 'home', JSON.stringify({ welcome: 'Hello {name}' }));

console.log(engine.t('home', 'welcome', JSON.stringify({ name: 'Arsalan' })));
```

---

## 🌐 Browser Usage (WebAssembly)

For client-side bundles, resolve to the WASM build via the `browser` export condition — bundlers like Vite pick this up automatically:

```javascript
import { I18NEngine } from '@encheco/i18n';

const engine = new I18NEngine('en', 'en');
engine.addBundle('en', 'home', JSON.stringify({ welcome: 'Hello {name}' }));

console.log(engine.t('home', 'welcome', JSON.stringify({ name: 'Arsalan' })));
```

The WASM build additionally exposes `[Symbol.dispose]()` / `free()` on `I18NEngine` and `WasmCompiledTemplate`, so you can release Rust-side memory explicitly when an engine instance is no longer needed.

---

## 🚀 Usage with Nuxt 3

Add the module to your `nuxt.config.ts`. It automatically wires up the WASM Vite plugins, loads your locale JSON bundles, and generates TypeScript schemas.

```typescript
export default defineNuxtConfig({
  modules: ['@encheco/i18n/nuxt'],
  i18nSdk: {
    localeDir: 'langs' // default: 'langs'
  }
})
```

### Module Options

| Option | Type | Default | Description |
|---|---|---|---|
| `localeDir` | `string` | `'langs'` | Folder (relative to `rootDir`) containing one subfolder per locale, each with `*.json` namespace files. |
| `fetchLocalesApi` | `string` | — | Optional URL to fetch the list of available locales from at build time (expects `{ locales: string[] }`). |
| `hardcodedLocales` | `string[]` | — | Skip auto-detection entirely and use this fixed list of locales. |

Under the hood, the module:

1. Injects `vite-plugin-wasm` and `vite-plugin-top-level-await` so the WASM core loads correctly.
2. Registers a virtual module, `virtual:@encheco/i18n-bundles`, exporting `bundles` and `locales` for consumption in your app.
3. Walks `localeDir`, and — using your first detected/configured locale as the source of truth — writes an auto-generated `i18n-schema.d.ts` into `.nuxt/`, giving `engine.t()` full autocomplete for namespaces and keys.
4. Marks `@encheco/i18n` as external for the Vite SSR bundle and excludes it from dep optimization, since it contains native/WASM binaries.

```javascript
// anywhere in your app
import { bundles, locales } from 'virtual:@encheco/i18n-bundles';
```

---

## ⚡ Usage with Vite

If you're not using Nuxt, the standalone Vite plugin sets up the same WASM support:

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import { i18nSdkVitePlugin } from '@encheco/i18n/vite';

export default defineConfig({
  plugins: [
    ...i18nSdkVitePlugin()
  ]
});
```

---

## 🖼️ Async Media Management (Node.js)

The `I18NEngine` can scan a directory of localized media and resolve the correct path based on the user's locale. `scanMediaFolderAsync` is a Node-only convenience method layered on top of the native `addMedia`, expecting a folder structured as `rootPath/<locale>/<file>`:

```javascript
const { I18NEngine } = require('@encheco/i18n');

const engine = new I18NEngine('en', 'en');

// rootPath/en/logo.png -> registered as media key "logo"
await engine.scanMediaFolderAsync('assets', './public/media', '/media');

const logoUrl = engine.getMedia('assets', 'logo');
console.log(logoUrl); // "/media/en/logo.png"
```

You can also register individual media entries by hand with the native, synchronous `addMedia(locale, namespace, key, url)`.

---

## 🩺 Diagnostics, Debugging & Help

Enable debug mode to catch missing translation keys, print a full memory diagnostic report of everything cached in the Rust backend, or pull up an in-code API tour at any time:

```javascript
engine.setDebugMode(true);

console.log(engine.diagnostics());

// Prints a quick tour of the available methods, straight from the engine itself
console.log(engine.help());
```

---

## 🔢 Pluralization

Plural resolution runs entirely inside the Rust core — given a locale and a number, the engine returns one of the six standard CLDR categories: `zero`, `one`, `two`, `few`, `many`, `other`. No JSON plural tables, no runtime ICU parsing — the rule for each locale is compiled straight into the binary.

Locale matching uses the language subtag only (e.g. `en-US` and `en-GB` both resolve as `en`), so regional variants are covered automatically.

### Locale coverage

| Category rule | Locales |
|---|---|
| **No plural distinction beyond one/zero** | `fa` `ja` `ko` `zh` `vi` `th` `id` `ms` `my` `km` `lo` `dz` `ig` `yo` |
| **Germanic / Romance (`one` = 1 only)** | `en` `de` `es` `it` `nl` `sv` `da` `no` `nn` `nb` `bg` `el` `fi` `hu` `et` `ca` `eo` `fo` `gl` `sw` `ur` |
| **`one` covers 0–2 (French-style)** | `fr` `pt-br` `hi` `am` `bn` `gu` `mr` `pa` `zu` |
| **Arabic — full 6-category system** | `ar` |
| **East Slavic** | `ru` `uk` `be` |
| **Polish** | `pl` |
| **West Slavic** | `cs` `sk` |
| **Romanian / Moldavian** | `ro` `mo` |
| **South Slavic** | `hr` `sr` `bs` `sh` |
| **Slovenian** | `sl` |
| **Hebrew** | `he` `iw` |
| **Lithuanian** | `lt` |
| **Latvian** | `lv` |
| **Welsh** | `cy` |
| **Irish** | `ga` |
| **Icelandic / Macedonian** | `is` `mk` |
| **Any other locale** | Falls back to a simple `one` (n = 1) / `other` split |

### Example

```javascript
engine.addBundle('ar', 'cart', JSON.stringify({
  items: '{count, plural, zero {سبد خالی است} one {یک محصول} two {دو محصول} few {# محصول} many {# محصول} other {# محصول}}'
}));

// The engine picks the right branch by resolving the CLDR category for the given count and locale
```

> The exact plural-message syntax (e.g. ICU-style `{count, plural, ...}`) is handled by the template compiler — pass whichever `count` your bundle expects, and the Rust core resolves the correct branch for the active locale.

---

## 📚 API Reference

| Method | Description |
|---|---|
| `new I18NEngine(defaultLocale, fallbackLocale?)` | Creates a new engine instance. |
| `.setLocale(locale)` | Switches the engine's active locale. |
| `.setFallbackLocale(locale)` | Sets the locale to fall back to when a key is missing. |
| `.addBundle(locale, namespace, jsonString)` | Registers a namespace's translations for a locale from a JSON string. |
| `.addMedia(locale, namespace, key, url)` | Registers a single localized media asset. |
| `.scanMediaFolderAsync(namespace, rootPath, baseUrl)` | *(Node.js only)* Recursively scans `rootPath/<locale>/*` and registers every file as media. |
| `.getMedia(namespace, key)` | Resolves a media URL for the current locale, or `undefined`. |
| `.t(namespace, key, argsJson?)` | Renders a translation, optionally interpolating a JSON-encoded args object. |
| `.setDebugMode(enabled)` | Toggles verbose logging for missing keys. |
| `.diagnostics()` | Returns a string report of cached bundles/keys in the Rust backend. |
| `.help()` | Returns a quick API tour, printed straight from the engine. |
| `.version()` | Returns the compiled engine's version string. |
| `.free()` / `[Symbol.dispose]()` | *(WASM only)* Releases the underlying Rust memory. |

`JsCompiledTemplate` (Node) / `WasmCompiledTemplate` (WASM) expose a lower-level `.render(args)` for working with a single pre-compiled template directly.

---

## ⚙️ How It Works (The Rust Core)

Unlike traditional i18n libraries that parse JSON and run Regex replacements on every render, `@encheco/i18n` uses a custom Rust parser to convert translation strings into an AST (Abstract Syntax Tree) the first time a bundle is added.

These tokens are stored in an `Arc` (Atomic Reference Counted) `HashMap` inside the `I18NEngine`. When a translation is requested via `.t()`, the engine directly renders the pre-compiled tokens — bypassing expensive string lookups and Regex entirely, whether it's running as a native N-API addon on the server or compiled to WebAssembly in the browser.

---

## 📊 Comparison with Other i18n Libraries

There are several well-established i18n libraries in the JavaScript ecosystem. Here's how **[@encheco/i18n](https://www.npmjs.com/package/@encheco/i18n)** stacks up against the most common ones:

| | **[@encheco/i18n](https://www.npmjs.com/package/@encheco/i18n)** | i18next / react-i18next | next-intl | vue-i18n |
|---|---|---|---|---|
| **Core runtime** | Rust (N-API + WASM) | Pure JavaScript | Pure JavaScript | Pure JavaScript |
| **Translation resolution** | Pre-compiled AST tokens, zero-lookup at render time | Runtime key lookup / interpolation | Runtime key lookup, ICU-based | Runtime key lookup / interpolation |
| **Browser target** | WebAssembly | JS bundle | JS bundle | JS bundle |
| **Node.js target** | Native N-API addon | JS (Node runtime) | JS (Node runtime) | JS (Node runtime) |
| **Framework scope** | Framework-agnostic core + Nuxt 3 / Vite integrations | Framework-agnostic core + bindings for React, Vue, Angular, etc. | Next.js (App Router) only | Vue only |
| **Type-safe keys** | Auto-generated `i18n-schema.d.ts` from your locale JSON (Nuxt) | Possible via community tooling / codegen | Built-in with `next-intl`'s typed messages | Possible via community tooling |
| **Pluralization** | Native CLDR rules in Rust | Full ICU/CLDR support (via plugins) | Full ICU support | CLDR-based plural rules |
| **Media/asset localization** | Built-in (`addMedia`, `scanMediaFolderAsync`) | Not built-in | Not built-in | Not built-in |
| **Ecosystem maturity** | New, small | Very large — react-i18next alone sees over 9 million weekly npm downloads | Large, fast-growing in the Next.js ecosystem | Large within the Vue ecosystem |
| **Best fit** | Apps that want a shared, native-speed engine across SSR + browser with minimal per-render overhead | Broad framework support, mature plugin ecosystem, maximum community resources | Next.js App Router projects wanting first-class typed messages | Vue/Nuxt 2 projects |

> This table reflects general, publicly known characteristics of each project as of 2026 and isn't a benchmark. For the authoritative, up-to-date feature list and version history of this package, see the **[@encheco/i18npage on npm](https://www.npmjs.com/package/@encheco/i18n)**.

---

## 📄 License

MIT