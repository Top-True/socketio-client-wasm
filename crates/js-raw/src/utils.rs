use js_sys::{Object, Reflect, wasm_bindgen::JsValue};

#[inline(always)]
pub fn set_property(obj: &Object, key: &str, value: &JsValue) -> bool {
    Reflect::set(obj, &key.into(), value)
        .expect(format!("obj({:?}) is not a Js Object ", obj).as_str())
}
