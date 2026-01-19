use js_raw::*;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub ack_timeout: JsUndefinedOption<Duration>,
    pub auth: JsUndefinedOption<JsObject>,
    pub retries: JsUndefinedOption<u32>,
}

impl Into<JsValue> for Options {
    fn into(self) -> JsValue {
        let result = JsObject::new();
        set_property(&result, "ackTimeout", &self.ack_timeout.millis_into_js_value());
        set_property(&result, "auth", &self.auth.into());
        set_property(&result, "retries", &self.retries.into());
        result.into()
    }
}
