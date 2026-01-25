use js_raw::*;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub auto_connect: JsUndefinedOption<bool>,
    // todo
    // pub parser: JsUndefinedOption<super::parser::Parser>,
    pub randomization_factor: JsUndefinedOption<f64>,
    pub reconnection: JsUndefinedOption<bool>,
    pub reconnection_attempts: JsUndefinedOption<ReconnectionAttempts>,
    pub reconnection_delay: JsUndefinedOption<Duration>,
    pub reconnection_delay_max: JsUndefinedOption<Duration>,
    pub timeout: JsUndefinedOption<Duration>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        set_property(&result, "autoConnect", &self.auto_connect.to_js());
        // set_property(&result, "parser", self.parser.into());
        set_property(
            &result,
            "randomizationFactor",
            &self.randomization_factor.to_js(),
        );
        set_property(&result, "reconnection", &self.reconnection.to_js());
        set_property(
            &result,
            "reconnectionAttempts",
            &self.reconnection_attempts.to_js(),
        );
        set_property(
            &result,
            "reconnectionDelay",
            &self.reconnection_delay.millis_to_js(),
        );
        set_property(
            &result,
            "reconnectionDelayMax",
            &self.reconnection_delay_max.millis_to_js(),
        );
        set_property(&result, "timeout", &self.timeout.millis_to_js());
        result.into()
    }
}

impl ToJs<JsValue> for Options {
    fn to_js(&self) -> JsValue {
        ToJs::<JsObject>::to_js(self).into()
    }
}

#[derive(Debug, Clone)]
pub enum ReconnectionAttempts {
    Infinity,
    U32(u32),
}

impl From<JsNumber> for ReconnectionAttempts {
    fn from(value: JsNumber) -> Self {
        if JsNumber::is_finite(&value) {
            ReconnectionAttempts::U32(value.as_f64().unwrap() as u32)
        } else {
            ReconnectionAttempts::Infinity
        }
    }
}

impl ToJs<JsNumber> for ReconnectionAttempts {
    fn to_js(&self) -> JsNumber {
        match self {
            ReconnectionAttempts::Infinity => js_eval("Number.POSITIVE_INFINITY")
                .unwrap()
                .unchecked_into(),
            ReconnectionAttempts::U32(x) => (*x).into(),
        }
    }
}

impl ToJs<JsValue> for ReconnectionAttempts {
    fn to_js(&self) -> JsValue {
        ToJs::<JsNumber>::to_js(self).into()
    }
}
