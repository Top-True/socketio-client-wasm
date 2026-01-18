use js_raw::*;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub ack_timeout: Option<Duration>,
    pub auth: Option<JsObject>,
    pub retries: Option<u32>,
}
