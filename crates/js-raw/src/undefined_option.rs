use crate::to_js::{DurationToJs, ToJs};
use js_sys::wasm_bindgen::JsValue;
use std::time::Duration;

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

impl<T> ToJs<JsValue> for JsUndefinedOption<T>
where
    T: ToJs<JsValue>,
{
    fn to_js(&self) -> JsValue {
        match self {
            JsUndefinedOption::Undefined => JsValue::undefined(),
            JsUndefinedOption::Some(x) => x.to_js(),
        }
    }
}

impl DurationToJs<JsValue> for JsUndefinedOption<Duration> {
    fn millis_to_js(&self) -> JsValue {
        match self {
            JsUndefinedOption::Undefined => JsValue::undefined(),
            JsUndefinedOption::Some(x) => x.millis_to_js(),
        }
    }
}
