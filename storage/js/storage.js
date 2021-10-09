let ctx = null
let memory

storage_init = (wasm_memory, _wasm_exports) => {
	memory = wasm_memory
	ctx = {}
}

storage_register = importObject => {
	importObject.env.storage_contains = key => localStorage.getItem(get_js_object(key)) == null ? 0 : 1
	importObject.env.storage_get = key => js_object(localStorage.getItem(get_js_object(key)))
	importObject.env.storage_set = (key, data) => localStorage.setItem(get_js_object(key), get_js_object(data))
}

miniquad_add_plugin({
	register_plugin: storage_register,
	on_init: storage_init,
	name: "mcg_storage",
	version: "0.1.0"
})