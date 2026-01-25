use js_raw::*;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub add_trailing_slash: JsUndefinedOption<bool>,
    pub auto_unref: JsUndefinedOption<bool>,
    pub close_on_beforeunload: JsUndefinedOption<bool>,
    pub extra_headers: JsUndefinedOption<JsObject>,
    pub force_base64: JsUndefinedOption<bool>,
    pub path: JsUndefinedOption<String>,
    pub protocols: JsUndefinedOption<Vec<String>>,
    pub query: JsUndefinedOption<JsObject>,
    pub remember_upgrade: JsUndefinedOption<bool>,
    pub timestamp_param: JsUndefinedOption<String>,
    pub timestamp_requests: JsUndefinedOption<bool>,
    pub transport_options: JsUndefinedOption<JsObject>,
    pub transports: JsUndefinedOption<TransportOption>,
    pub try_all_transports: JsUndefinedOption<bool>,
    pub upgrade: JsUndefinedOption<bool>,
    pub with_credentials: JsUndefinedOption<bool>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        set_property(&result, "addTrailingSlash", &self.add_trailing_slash.to_js());
        set_property(&result, "autoUnref", &self.auto_unref.to_js());
        set_property(
            &result,
            "closeOnBeforeunload",
            &self.close_on_beforeunload.to_js(),
        );
        set_property(&result, "extraHeaders", &self.extra_headers.to_js());
        set_property(&result, "forceBase64", &self.force_base64.to_js());
        set_property(&result, "path", &self.path.to_js());
        set_property(&result, "protocols", &self.protocols.to_js());
        set_property(&result, "query", &self.query.to_js());
        set_property(&result, "rememberUpgrade", &self.remember_upgrade.to_js());
        set_property(&result, "timestampParam", &self.timestamp_param.to_js());
        set_property(
            &result,
            "timestampRequests",
            &self.timestamp_requests.to_js(),
        );
        set_property(&result, "transportOptions", &self.transport_options.to_js());
        set_property(&result, "transports", &self.transports.to_js());
        set_property(&result, "tryAllTransports", &self.try_all_transports.to_js());
        set_property(&result, "upgrade", &self.upgrade.to_js());
        set_property(&result, "withCredentials", &self.with_credentials.to_js());
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
