# i18n 🚀

[![npm version](https://img.shields.io/npm/v/@encheco/i18n.svg)](https://www.npmjs.com/package/@encheco/i18n)
[![pub package](https://img.shields.io/pub/v/i18n_flutter.svg)](https://pub.dev/packages/i18n_flutter)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/core-Rust-orange.svg)](https://www.rust-lang.org/)

A single, native-speed **`I18NEngine`** written in Rust, shared across every platform: Node.js, the browser (WebAssembly), and Flutter.

Translation strings are compiled once into an AST inside Rust — no regex, no runtime JSON parsing, no per-render string lookups — and every binding in this repo calls into that same compiled core, so behavior (pluralization, fallback resolution, template rendering) stays identical no matter which platform you ship to.

---

## Table of Contents

- [Packages in this repo](#-packages-in-this-repo)
- [Why one Rust core](#-why-one-rust-core)
- [Repository structure](#-repository-structure)
- [Getting started](#-getting-started)
- [Pluralization](#-pluralization)
- [Contributing](#-contributing)
- [License](#-license)

---

## 📦 Packages in this repo

| Package | Platform | Registry |
|---|---|---|
| **[`@encheco/i18n`](./i18n-node)** | Node.js (N-API) & browser (WASM), with Nuxt 3 / Vite integrations | [npm](https://www.npmjs.com/package/@encheco/i18n) |
| **[`i18n_flutter`](./i18n_flutter)** | Flutter — Android, iOS, Linux, macOS, Windows, via `flutter_rust_bridge` | [pub.dev](https://pub.dev/packages/i18n_flutter) |

Both packages are thin bindings over the same [`i18n-core`](./i18n-core) Rust crate — if you're running a JS backend/frontend *and* a Flutter app off the same translation bundles, behavior is guaranteed to match because it's the same compiled engine underneath, not two separate reimplementations.

---

## ⚙️ Why one Rust core

Most i18n libraries reimplement pluralization, template interpolation, and fallback logic separately for every language/framework they support. This repo takes a different approach:

* `i18n-core` — the Rust engine (`I18NEngine`): AST-based template compilation, CLDR pluralization for 60+ locales, bundle/namespace management, media resolution.
* `i18n-node` — N-API bindings (native Node addon) + a WASM build for browsers, plus Nuxt 3 and Vite integrations.
* `i18n-wasm` — the WebAssembly build target consumed by `i18n-node`'s browser export.
* `i18n_flutter` — Dart bindings generated with `flutter_rust_bridge_codegen`, exposing the same engine natively inside Flutter apps.

Add a translation once, and every platform renders it with the exact same rules.

---

## 🗂 Repository structure

```
.
├── i18n-core/       # Rust engine — I18NEngine, AST compiler, CLDR plural rules
├── i18n-node/       # Node.js (N-API) + browser (WASM) bindings, npm package
├── i18n-wasm/       # WebAssembly build target
├── i18n_flutter/    # Flutter package (flutter_rust_bridge bindings), pub.dev package
├── Cargo.toml       # Workspace root
└── Cargo.lock
```

---

## 🚀 Getting started

**Node.js / browser:**

```bash
npm install @encheco/i18n
```
→ See the [`i18n-node` README](./i18n-node/README.md) for full usage (Node, ESM, browser/WASM, Nuxt 3, Vite).

**Flutter:**

```bash
flutter pub add i18n_flutter
```
→ See the [`i18n_flutter` README](./i18n_flutter/README.md) for full usage (engine setup, bundles, diagnostics).

---

## 🔢 Pluralization

Every binding shares the same CLDR-compliant plural engine, resolving all six standard categories (`zero`, `one`, `two`, `few`, `many`, `other`) across 60+ locales — from simple one/other splits to the full 6-category systems needed for Arabic, Slavic, Baltic, and Celtic languages. The rule for each locale is compiled directly into the Rust core, so there's no plural table to keep in sync across platforms.

---

## 🤝 Contributing

Issues and pull requests are welcome. If you're adding a new platform binding, it should call into `i18n-core` rather than reimplementing engine logic, so all bindings stay behaviorally consistent.

---

## 📄 License

MIT
