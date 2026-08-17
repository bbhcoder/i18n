# i18n_flutter 🚀

[![pub package](https://img.shields.io/pub/v/i18n_flutter.svg)](https://pub.dev/packages/i18n_flutter)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/core-Rust-orange.svg)](https://www.rust-lang.org/)
[![powered by flutter_rust_bridge](https://img.shields.io/badge/bridge-flutter__rust__bridge-blueviolet.svg)](https://pub.dev/packages/flutter_rust_bridge)

A blazing fast, zero-lookup Internationalization (i18n) engine for Flutter, powered by a native Rust core.

`i18n_flutter` binds the same **`I18NEngine`** Rust engine that powers [`@encheco/i18n`](https://www.npmjs.com/package/@encheco/i18n) for Node.js directly into your Flutter app, via [`flutter_rust_bridge`](https://pub.dev/packages/flutter_rust_bridge). Translations are compiled once into an AST inside Rust, so every `t()` call is a direct, synchronous, zero-lookup render — no JSON parsing or regex on the hot path.

---

## Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Supported Platforms](#️-supported-platforms)
- [Quick Start](#-quick-start)
- [Diagnostics & Debugging](#-diagnostics--debugging)
- [Pluralization](#-pluralization)
- [API Reference](#-api-reference)
- [How It Works (The Rust Core)](#️-how-it-works-the-rust-core)
- [License](#-license)

---

## ✨ Features

* **Blazing Fast (Zero-Lookup):** Translation strings are compiled into memory tokens via a Rust AST the first time a bundle is added — no regex or string manipulation at render time.
* **Native Rust Core:** Runs through `flutter_rust_bridge`, calling into the same `I18NEngine` Rust engine used by the Node.js/WASM version of this library — same core, consistent behavior across platforms.
* **Fully Synchronous API:** Once the bridge is initialized, `t()`, `addBundle()`, `setLocale()` and friends are plain synchronous Dart calls — no `Future`/`await` in your widget tree.
* **Native Plurals:** CLDR-compliant pluralization resolved entirely inside the Rust core, covering all six plural categories (`zero`, `one`, `two`, `few`, `many`, `other`) across 60+ locales.
* **Built-in Diagnostics:** Toggle debug mode to catch missing keys, and pull a memory report of every cached bundle straight from the Rust backend.
* **Lightweight Surface:** A small, focused API — create an engine, register bundles, translate. No codegen step, no build-time schema generation required.

---

## 📦 Installation

```bash
flutter pub add i18n_flutter
```

Or add it manually to your `pubspec.yaml`:

```yaml
dependencies:
  i18n_flutter: ^0.1.4
```

The package ships with prebuilt Rust bindings wired up through `flutter_rust_bridge` — no local Rust toolchain is required to consume it.

---

## 🖥️ Supported Platforms

| Platform | Support |
|---|---|
| Android | ✅ |
| iOS | ✅ |
| Linux | ✅ |
| macOS | ✅ |
| Windows | ✅ |

---

## 🚀 Quick Start

Before using the engine, initialize the Rust bridge once — typically in `main()`:

```dart
import 'package:flutter/material.dart';
import 'package:i18n_flutter/src/rust/frb_generated.dart';
import 'package:i18n_flutter/src/rust/api/engine.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const MyApp());
}
```

Once initialized, create an engine, register a bundle, and translate:

```dart
final engine = I18NEngine(defaultLocale: 'en', fallbackLocale: 'en');

engine.addBundle(
  locale: 'en',
  namespace: 'home',
  jsonString: '{"welcome": "Hello {name}"}',
);

final greeting = engine.t(
  namespace: 'home',
  key: 'welcome',
  argsJson: '{"name": "Arsalan"}',
);

print(greeting); // "Hello Arsalan"
```

Switch the active locale at any time:

```dart
engine.setLocale(locale: 'fa');
```

> `RustLib.init()` only needs to be called once per app lifecycle, before the first `I18NEngine` is created.

---

## 🩺 Diagnostics & Debugging

Enable debug mode to catch missing translation keys, and print a memory report of everything cached in the Rust backend:

```dart
engine.setDebugMode(enabled: true);

print(engine.diagnostics());
```

---

## 🔢 Pluralization

Plural resolution runs entirely inside the Rust core — given a locale and a count embedded in your translation string, the engine resolves the correct CLDR category: `zero`, `one`, `two`, `few`, `many`, or `other`. There's no JSON plural table to maintain and no runtime ICU parsing on the Dart side — the rule for each locale is compiled straight into the native binary.

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

```dart
engine.addBundle(
  locale: 'ar',
  namespace: 'cart',
  jsonString:
      '{"items": "{count, plural, zero {سبد خالی است} one {یک محصول} two {دو محصول} few {# محصول} many {# محصول} other {# محصول}}"}',
);

// engine.t(namespace: 'cart', key: 'items', argsJson: '{"count": 2}')
// resolves the correct branch for the active locale's CLDR category.
```

> The exact plural-message syntax (e.g. ICU-style `{count, plural, ...}`) is handled by the template compiler — pass whichever `count` your bundle expects, and the Rust core resolves the correct branch for the active locale.

---

## 📚 API Reference

| Member | Description |
|---|---|
| `I18NEngine({required String defaultLocale, String? fallbackLocale})` | Creates a new engine instance. |
| `.setLocale({required String locale})` | Switches the engine's active locale. |
| `.addBundle({required String locale, required String namespace, required String jsonString})` | Registers a namespace's translations for a locale from a JSON string. |
| `.t({required String namespace, required String key, String? argsJson})` | Renders a translation, optionally interpolating a JSON-encoded args object. |
| `.setDebugMode({required bool enabled})` | Toggles verbose logging for missing keys. |
| `.diagnostics()` | Returns a string report of cached bundles/keys in the Rust backend. |
| `RustLib.init()` | Initializes the native Rust bridge. Call once, before creating an `I18NEngine`. |

---

## ⚙️ How It Works (The Rust Core)

Unlike traditional i18n libraries that parse JSON and run string replacements on every render, `i18n_flutter` uses a custom Rust parser to convert translation strings into an AST (Abstract Syntax Tree) the first time a bundle is added via `addBundle()`.

These tokens are cached inside the `I18NEngine` Rust struct. When a translation is requested via `.t()`, the call crosses the Dart↔Rust bridge and the engine directly renders the pre-compiled tokens — bypassing string lookups and regex entirely, whether running natively on Android/iOS/desktop.

This is the same engine used by [`@encheco/i18n`](https://www.npmjs.com/package/@encheco/i18n) on the Node.js/WebAssembly side, so translation behavior stays consistent if you share a backend or SSR layer with a JavaScript app.

---

## 📄 License

MIT