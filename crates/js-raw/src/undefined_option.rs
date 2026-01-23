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
