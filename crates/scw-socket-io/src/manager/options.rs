use scw_js_raw::*;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub auto_connect: JsOption<bool>,
    pub randomization_factor: JsOption<f64>,
    pub reconnection: JsOption<bool>,
    pub reconnection_attempts: JsOption<ReconnectionAttempts>,
    pub reconnection_delay: JsOption<Duration>,
    pub reconnection_delay_max: JsOption<Duration>,
    pub timeout: JsOption<Duration>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        self.auto_connect
            .if_some_then(|x| set_property(&result, "autoConnect", &x));
        self.randomization_factor
            .if_some_then(|x| set_property(&result, "randomizationFactor", &x));
        self.reconnection
            .if_some_then(|x| set_property(&result, "reconnection", &x));
        self.reconnection_attempts
            .if_some_then2(|x| set_property(&result, "reconnectionAttempts", &x));
        if let JsOption::Some(x) = self.reconnection_delay {
            set_property(&result, "reconnectionDelay", &x.millis_to_js());
        }
        if let JsOption::Some(x) = self.reconnection_delay_max {
            set_property(&result, "reconnectionDelayMax", &x.millis_to_js());
        }
        if let JsOption::Some(x) = self.timeout {
            set_property(&result, "timeout", &x.millis_to_js());
        }
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
