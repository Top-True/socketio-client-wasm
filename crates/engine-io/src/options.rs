use js_raw::*;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub add_trailing_slash: Option<bool>,
    pub auto_unref: Option<bool>,
    pub close_on_beforeunload: Option<bool>,
    pub extra_headers: Option<JsObject>,
    pub force_base64: Option<bool>,
    pub path: Option<String>,
    pub protocols: Option<Vec<String>>,
    pub query: Option<JsObject>,
    pub remember_upgrade: Option<bool>,
    pub timestamp_param: Option<String>,
    pub timestamp_requests: Option<bool>,
    pub transport_options: Option<JsObject>,
    pub transports: Option<TransportOption>,
    pub try_all_transports: Option<bool>,
    pub upgrade: Option<bool>,
    pub with_credentials: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct TransportOption {
    pub polling: bool,
    pub websocket: bool,
    pub webtransport: bool,
}
