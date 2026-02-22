use js_sys::wasm_bindgen::{JsCast, JsValue};
use std::time::Duration;

pub trait ToJs<T: JsCast> {
    fn to_js(&self) -> T;
}

pub trait DurationToJs<T: JsCast> {
    fn millis_to_js(&self) -> T;
}

impl DurationToJs<js_sys::Number> for Duration {
    fn millis_to_js(&self) -> js_sys::Number {
        (self.as_millis() as f64).into()
    }
}

impl DurationToJs<JsValue> for Duration {
    fn millis_to_js(&self) -> JsValue {
        DurationToJs::<js_sys::Number>::millis_to_js(self).into()
    }
}

impl<T> ToJs<JsValue> for T
where
    T: Into<JsValue> + Clone,
    JsValue: From<T>,
{
    fn to_js(&self) -> JsValue {
        JsValue::from(self.clone())
    }
}
