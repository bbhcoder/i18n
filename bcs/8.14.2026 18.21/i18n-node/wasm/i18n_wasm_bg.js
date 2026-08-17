export class I18nEngine {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        I18nEngineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_i18nengine_free(ptr, 0);
    }
    /**
     * @param {string} locale
     * @param {string} namespace
     * @param {string} json_string
     */
    addBundle(locale, namespace, json_string) {
        const ptr0 = passStringToWasm0(locale, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(namespace, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(json_string, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        wasm.i18nengine_addBundle(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
    }
    /**
     * @param {string} locale
     * @param {string} namespace
     * @param {string} key
     * @param {string} url
     */
    addMedia(locale, namespace, key, url) {
        const ptr0 = passStringToWasm0(locale, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(namespace, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(url, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len3 = WASM_VECTOR_LEN;
        wasm.i18nengine_addMedia(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    }
    /**
     * @param {string} namespace
     * @param {string} key
     * @returns {string | undefined}
     */
    getMedia(namespace, key) {
        const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.i18nengine_getMedia(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        let v3;
        if (ret[0] !== 0) {
            v3 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        }
        return v3;
    }
    /**
     * @param {string} default_locale
     * @param {string | null} [fallback_locale]
     */
    constructor(default_locale, fallback_locale) {
        const ptr0 = passStringToWasm0(default_locale, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(fallback_locale) ? 0 : passStringToWasm0(fallback_locale, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.i18nengine_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret;
        I18nEngineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} locale
     */
    setFallbackLocale(locale) {
        const ptr0 = passStringToWasm0(locale, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.i18nengine_setFallbackLocale(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} locale
     */
    setLocale(locale) {
        const ptr0 = passStringToWasm0(locale, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.i18nengine_setLocale(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} namespace
     * @param {string} key
     * @param {string | null} [args_json]
     * @returns {string}
     */
    t(namespace, key, args_json) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(namespace, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            var ptr2 = isLikeNone(args_json) ? 0 : passStringToWasm0(args_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len2 = WASM_VECTOR_LEN;
            const ret = wasm.i18nengine_t(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2);
            deferred4_0 = ret[0];
            deferred4_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred4_0, deferred4_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    version() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.i18nengine_version(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) I18nEngine.prototype[Symbol.dispose] = I18nEngine.prototype.free;

/**
 * هندل وب‌اسمبلی برای رندرینگ فوق‌سريع در مرورگر
 */
export class WasmCompiledTemplate {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmCompiledTemplateFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcompiledtemplate_free(ptr, 0);
    }
    /**
     * @param {string | null} [args_json]
     * @returns {string}
     */
    render(args_json) {
        let deferred2_0;
        let deferred2_1;
        try {
            var ptr0 = isLikeNone(args_json) ? 0 : passStringToWasm0(args_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            const ret = wasm.wasmcompiledtemplate_render(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
}
if (Symbol.dispose) WasmCompiledTemplate.prototype[Symbol.dispose] = WasmCompiledTemplate.prototype.free;
export function __wbg___wbindgen_throw_344f42d3211c4765(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_error_744744ff0c9861e6(arg0) {
    console.error(arg0);
}
export function __wbindgen_cast_0000000000000001(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}
const I18nEngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_i18nengine_free(ptr, 1));
const WasmCompiledTemplateFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcompiledtemplate_free(ptr, 1));

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}
