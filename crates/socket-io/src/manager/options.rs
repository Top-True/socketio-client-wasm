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

impl Into<JsValue> for Options {
    fn into(self) -> JsValue {
        let result = JsObject::new();
        set_property(&result, "autoConnect", &self.auto_connect.into());
        // set_property(&result, "parser", self.parser.into());
        set_property(
            &result,
            "randomizationFactor",
            &self.randomization_factor.into(),
        );
        set_property(&result, "reconnection", &self.reconnection.into());
        set_property(
            &result,
            "reconnectionAttempts",
            &self.reconnection_attempts.into(),
        );
        set_property(
            &result,
            "reconnectionDelay",
            &self.reconnection_delay.millis_into_js_value(),
        );
        set_property(
            &result,
            "reconnectionDelayMax",
            &self.reconnection_delay_max.millis_into_js_value(),
        );
        set_property(&result, "timeout", &self.timeout.millis_into_js_value());
        result.into()
    }
}

#[derive(Debug, Clone)]
pub enum ReconnectionAttempts {
    Infinity,
    U32(u32),
}

impl Into<JsValue> for ReconnectionAttempts {
    fn into(self) -> JsValue {
        match self {
            ReconnectionAttempts::Infinity => JsNumber::POSITIVE_INFINITY.into(),
            ReconnectionAttempts::U32(x) => x.into(),
        }
    }
}
