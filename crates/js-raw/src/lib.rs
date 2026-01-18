pub use js_sys::Array as JsArray;
pub use js_sys::Function as JsFunction;
pub use js_sys::Object as JsObject;
pub use js_sys::Reflect as JsReflect;
pub use js_sys::wasm_bindgen::{JsCast, JsValue, closure::Closure as JsClosure};
use std::sync::LazyLock;

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
