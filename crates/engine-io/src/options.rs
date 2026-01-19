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

impl Into<JsValue> for Options {
    fn into(self) -> JsValue {
        let result = JsObject::new();
        set_property(&result, "addTrailingSlash", &self.add_trailing_slash.into());
        set_property(&result, "autoUnref", &self.auto_unref.into());
        set_property(
            &result,
            "closeOnBeforeunload",
            &self.close_on_beforeunload.into(),
        );
        set_property(&result, "extraHeaders", &self.extra_headers.into());
        set_property(&result, "forceBase64", &self.force_base64.into());
        set_property(&result, "path", &self.path.into());
        set_property(&result, "protocols", &self.protocols.into());
        set_property(&result, "query", &self.query.into());
        set_property(&result, "rememberUpgrade", &self.remember_upgrade.into());
        set_property(&result, "timestampParam", &self.timestamp_param.into());
        set_property(
            &result,
            "timestampRequests",
            &self.timestamp_requests.into(),
        );
        set_property(&result, "transportOptions", &self.transport_options.into());
        set_property(&result, "transports", &self.transports.into());
        set_property(&result, "tryAllTransports", &self.try_all_transports.into());
        set_property(&result, "upgrade", &self.upgrade.into());
        set_property(&result, "withCredentials", &self.with_credentials.into());
        result.into()
    }
}

#[derive(Debug, Clone)]
pub struct TransportOption {
    pub polling: bool,
    pub websocket: bool,
    pub webtransport: bool,
}

impl Into<JsValue> for TransportOption {
    fn into(self) -> JsValue {
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
