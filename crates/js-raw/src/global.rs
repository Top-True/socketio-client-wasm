use js_sys::{Function, Object, Reflect, global as raw_global, wasm_bindgen::JsCast};
use std::sync::LazyLock;
#[allow(non_upper_case_globals)]
pub static global: LazyLock<fn() -> Object> = LazyLock::new(|| {
    Reflect::get(&raw_global(), &"eval".into())
        .unwrap()
        .unchecked_into::<Function>()
        .call1(
            &raw_global(),
            &include_str!(concat!(env!("OUT_DIR"), "/socket.io.min.js")).into(),
        )
        .unwrap();
    raw_global
});

pub fn global_io() -> Function {
    Reflect::get(&global(), &"io".into())
        .unwrap()
        .unchecked_into()
}
