use scw_js_raw::*;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub ack_timeout: JsOption<Duration>,
    pub auth: JsOption<JsObject>,
    pub retries: JsOption<u32>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        if let JsOption::Some(x) = self.ack_timeout {
            set_property(&result, "ackTimeout", &x.millis_to_js());
        }
        self.auth
            .if_some_then(|x| set_property(&result, "auth", &x));
        self.retries
            .if_some_then(|x| set_property(&result, "retries", &x));
        result.into()
    }
}

impl ToJs<JsValue> for Options {
    fn to_js(&self) -> JsValue {
        ToJs::<JsObject>::to_js(self).into()
    }
}
