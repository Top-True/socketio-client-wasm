pub use js_sys::Array as JsArray;
pub use js_sys::Function as JsFunction;
pub use js_sys::Number as JsNumber;
pub use js_sys::Object as JsObject;
pub use js_sys::Reflect as JsReflect;
pub use js_sys::wasm_bindgen::closure::WasmClosure;
pub use js_sys::wasm_bindgen::{JsCast, JsValue, closure::Closure as JsClosure};
pub use js_sys::eval as js_eval;
pub use gloo;   // todo
use std::sync::LazyLock;
use std::time::Duration;

#[allow(non_upper_case_globals)]
pub static js_global: LazyLock<fn() -> JsObject> = LazyLock::new(|| {
    use js_sys::global;
    JsReflect::get(&global(), &"eval".into())
        .unwrap()
        .dyn_into::<JsFunction>()
        .unwrap()
        .call1(
            &global(),
            &include_str!(concat!(env!("OUT_DIR"), "/socket.io.min.js")).into(),
        )
        .unwrap();
    global
});

pub fn global_io() -> JsFunction {
    JsReflect::get(&js_global(), &"io".into())
        .unwrap()
        .dyn_into()
        .unwrap()
}

#[inline(always)]
pub fn set_property(obj: &JsObject, key: &str, value: &JsValue) -> bool {
    JsReflect::set(obj, &key.into(), value)
        .expect(format!("obj({:?}) is not a Js Object ", obj).as_str())
}

#[derive(Debug, Clone)]
pub enum JsUndefinedOption<T> {
    Undefined,
    Some(T),
}

impl<T> Default for JsUndefinedOption<T> {
    fn default() -> Self {
        Self::Undefined
    }
}

impl<T: Into<JsValue>> Into<JsValue> for JsUndefinedOption<T> {
    fn into(self) -> JsValue {
        match self {
            JsUndefinedOption::Undefined => JsValue::undefined(),
            JsUndefinedOption::Some(x) => x.into(),
        }
    }
}

impl JsUndefinedOption<Duration> {
    pub fn millis_into_js_value(self) -> JsValue {
        match self {
            JsUndefinedOption::Undefined => JsValue::undefined(),
            JsUndefinedOption::Some(x) => (x.as_millis() as f64).into(),
        }
    }
}
