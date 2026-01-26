use js_sys::wasm_bindgen::JsValue;

#[derive(Debug, Clone)]
pub enum JsOption<T> {
    Unset,
    Some(T),
}

impl<T> Default for JsOption<T> {
    fn default() -> Self {
        Self::Unset
    }
}

impl<T> JsOption<T>
where
    T: Into<JsValue> + Clone,
{
    pub fn if_some_then<F, R>(&self, then: F)
    where
        F: FnOnce(JsValue) -> R,
    {
        if let JsOption::Some(v) = self {
            then(v.clone().into());
        }
    }
}

impl<T> JsOption<T>
where
    T: crate::to_js::ToJs<JsValue>,
{
    pub fn if_some_then2<F, R>(&self, then: F)
    where
        F: FnOnce(JsValue) -> R,
    {
        if let JsOption::Some(v) = self {
            then(v.to_js());
        }
    }
}
