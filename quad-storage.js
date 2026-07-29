var ctx = null;
var memory;

params_set_mem = function (wasm_memory, _wasm_exports) {
    memory = wasm_memory;
    ctx = {};
}

params_register_js_plugin = function (importObject) {
    importObject.env.quad_storage_length = function () {
        return localStorage.length;
    }
    importObject.env.quad_storage_has_key = function (i) {
        return +(localStorage.key(i) != null);
    }
    importObject.env.quad_storage_key = function (i) {
        return js_object(localStorage.key(i));
    }
    importObject.env.quad_storage_has_value = function (key) {
        return +(localStorage.getItem(get_js_object(key)) != null);
    }
    importObject.env.quad_storage_get = function (key) {
        return js_object(localStorage.getItem(get_js_object(key)));
    }
    importObject.env.quad_storage_set = function (key, value) {
        localStorage.setItem(get_js_object(key), get_js_object(value));
    }
    importObject.env.quad_storage_remove = function (key) {
        localStorage.removeItem(get_js_object(key));
    }
    importObject.env.quad_storage_clear = function () {
        localStorage.clear();
    }
}

// `version` is compared with strict equality against the number returned by the
// wasm export `quad_storage_crate_version`, which lives in the *quad-storage-sys*
// crate (not quad-storage) and is encoded as (major << 24) + (minor << 16) + patch.
// quad-storage-sys 0.1.0 => (0 << 24) + (1 << 16) + 0 => 65536.
// Upstream ships a semver *string* here, which can never equal that number.
miniquad_add_plugin({
    register_plugin: params_register_js_plugin,
    on_init: params_set_mem,
    name: "quad_storage",
    version: 65536
});
