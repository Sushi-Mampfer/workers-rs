use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    pub type DurableObjectStorageKV;

    #[wasm_bindgen(method, catch)]
    pub fn get(this: &DurableObjectStorageKV, key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn put(this: &DurableObjectStorageKV, key: &str, value: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn delete(this: &DurableObjectStorageKV, key: &str) -> Result<bool, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub fn list(this: &DurableObjectStorageKV) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name=list)]
    pub fn list_with_options(
        this: &DurableObjectStorageKV,
        options: js_sys::Object,
    ) -> Result<JsValue, JsValue>;
}
