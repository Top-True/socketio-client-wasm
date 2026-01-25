use js_raw::*;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub ack_timeout: JsUndefinedOption<Duration>,
    pub auth: JsUndefinedOption<JsObject>,
    pub retries: JsUndefinedOption<u32>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        set_property(&result, "ackTimeout", &self.ack_timeout.millis_to_js());
        set_property(&result, "auth", &self.auth.to_js());
        set_property(&result, "retries", &self.retries.to_js());
        result.into()
    }
}

impl ToJs<JsValue> for Options {
    fn to_js(&self) -> JsValue {
        ToJs::<JsObject>::to_js(self).into()
    }
}
