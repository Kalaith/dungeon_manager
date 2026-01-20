//! WASM localStorage wrapper using sapp-jsutils
//! Only compiled for wasm32 target

use sapp_jsutils::JsObject;

extern "C" {
    fn storage_set_extern(key: JsObject, value: JsObject);
    fn storage_get_extern(key: JsObject) -> JsObject;
    fn storage_remove_extern(key: JsObject);
    fn storage_exists_extern(key: JsObject) -> bool;
}

pub fn storage_set(key: &str, value: &str) {
    let js_key = JsObject::string(key);
    let js_value = JsObject::string(value);
    unsafe { storage_set_extern(js_key, js_value) };
}

pub fn storage_get(key: &str) -> Option<String> {
    let js_key = JsObject::string(key);
    let result = unsafe { storage_get_extern(js_key) };
    if result.is_nil() {
        None
    } else {
        let mut buf = String::new();
        result.to_string(&mut buf);
        Some(buf)
    }
}

pub fn storage_remove(key: &str) {
    let js_key = JsObject::string(key);
    unsafe { storage_remove_extern(js_key) };
}

pub fn storage_exists(key: &str) -> bool {
    let js_key = JsObject::string(key);
    unsafe { storage_exists_extern(js_key) }
}
