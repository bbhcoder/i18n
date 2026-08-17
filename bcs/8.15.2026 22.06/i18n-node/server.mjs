// i18n-node/server.mjs
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const nativeBinding = require('./server.js');

// استخراج خالص و دقیقاً مشابه با فایل WASM مرورگر
export const I18nEngine = nativeBinding.I18nEngine;
export const JsCompiledTemplate = nativeBinding.JsCompiledTemplate;