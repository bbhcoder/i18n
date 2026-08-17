/* @ts-self-types="./i18n_wasm.d.ts" */
import * as wasm from "./i18n_wasm_bg.wasm";
import { __wbg_set_wasm } from "./i18n_wasm_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    I18nEngine, WasmCompiledTemplate
} from "./i18n_wasm_bg.js";
