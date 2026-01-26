use js_raw::*;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub add_trailing_slash: JsOption<bool>,
    pub auto_unref: JsOption<bool>,
    pub close_on_beforeunload: JsOption<bool>,
    pub extra_headers: JsOption<JsObject>,
    pub force_base64: JsOption<bool>,
    pub path: JsOption<String>,
    pub protocols: JsOption<Vec<String>>,
    pub query: JsOption<JsObject>,
    pub remember_upgrade: JsOption<bool>,
    pub timestamp_param: JsOption<String>,
    pub timestamp_requests: JsOption<bool>,
    pub transport_options: JsOption<JsObject>,
    pub transports: JsOption<TransportOption>,
    pub try_all_transports: JsOption<bool>,
    pub upgrade: JsOption<bool>,
    pub with_credentials: JsOption<bool>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        self.add_trailing_slash
            .if_some_then(|x| set_property(&result, "addTrailingSlash", &x));
        self.auto_unref
            .if_some_then(|x| set_property(&result, "autoUnref", &x));
        self.close_on_beforeunload
            .if_some_then(|x| set_property(&result, "closeOnBeforeunload", &x));
        self.extra_headers
            .if_some_then(|x| set_property(&result, "extraHeaders", &x));
        self.force_base64
            .if_some_then(|x| set_property(&result, "forceBase64", &x));
        self.path
            .if_some_then(|x| set_property(&result, "path", &x));
        self.protocols
            .if_some_then(|x| set_property(&result, "protocols", &x));
        self.query
            .if_some_then(|x| set_property(&result, "query", &x));
        self.remember_upgrade
            .if_some_then(|x| set_property(&result, "rememberUpgrade", &x));
        self.timestamp_param
            .if_some_then(|x| set_property(&result, "timestampParam", &x));
        self.timestamp_requests
            .if_some_then(|x| set_property(&result, "timestampRequests", &x));
        self.transport_options
            .if_some_then(|x| set_property(&result, "transportOptions", &x));
        self.transports
            .if_some_then2(|x| set_property(&result, "transports", &x));
        self.try_all_transports
            .if_some_then(|x| set_property(&result, "tryAllTransports", &x));
        self.upgrade
            .if_some_then(|x| set_property(&result, "upgrade", &x));
        self.with_credentials
            .if_some_then(|x| set_property(&result, "withCredentials", &x));
        result
    }
}

impl ToJs<JsValue> for Options {
    fn to_js(&self) -> JsValue {
        ToJs::<JsObject>::to_js(self).into()
    }
}

#[derive(Debug, Clone)]
pub struct TransportOption {
    pub polling: bool,
    pub websocket: bool,
    pub webtransport: bool,
}

impl ToJs<JsObject> for TransportOption {
    fn to_js(&self) -> JsObject {
        let result = JsArray::new_with_length(3);
        if self.polling {
            result.set(0, "polling".into());
        }
        if self.websocket {
            result.set(1, "websocket".into());
        }
        if self.webtransport {
            result.set(2, "webtransport".into());
        }
        result.into()
    }
}

impl ToJs<JsValue> for TransportOption {
    fn to_js(&self) -> JsValue {
        ToJs::<JsObject>::to_js(self).into()
    }
}
