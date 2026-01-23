pub mod options;
pub mod reason;

use js_raw::*;
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone)]
pub struct SID(pub(crate) String);

impl AsRef<str> for SID {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for SID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl std::fmt::Debug for SID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SID").field(&self.0).finish()
    }
}

#[derive(Debug, Clone)]
pub struct Socket {
    pub(crate) raw: JsObject,
}

impl Socket {
    pub unsafe fn new(raw: JsObject) -> Self {
        Self { raw }
    }

    pub fn id(&self) -> Option<SID> {
        let res = JsReflect::get(&self.raw, &"id".into()).unwrap().as_string();
        res.and_then(|s| Some(SID(s)))
    }

    pub fn connected(&self) -> bool {
        JsReflect::get(&self.raw, &"connected".into())
            .unwrap()
            .as_bool()
            .unwrap()
    }

    pub fn disconnected(&self) -> bool {
        JsReflect::get(&self.raw, &"disconnected".into())
            .unwrap()
            .as_bool()
            .unwrap()
    }

    pub fn io(&self) -> crate::manager::Manager {
        crate::manager::Manager {
            raw: JsReflect::get(&self.raw, &"io".into())
                .unwrap()
                .unchecked_into(),
        }
    }

    pub fn connect(&self) -> &Self {
        self.get_method("connect").call0(&self.raw).unwrap();
        self
    }

    pub fn send(&self, args: impl Into<JsValue>) -> &Self {
        self.get_method("send")
            .call1(&self.raw, &args.into())
            .unwrap();
        self
    }

    pub fn send_with_ack<A>(&self, args: impl Into<JsValue>, ack: A) -> &Self
    where
        A: FnOnce(JsValue) + 'static,
    {
        self.get_method("send")
            .call2(&self.raw, &args.into(), &JsClosure::once_into_js(ack))
            .unwrap();
        self
    }

    pub fn emit(&self, event_name: &str, args: impl Into<JsValue>) -> &Self {
        self.get_method("emit")
            .call2(&self.raw, &event_name.into(), &args.into())
            .unwrap();
        self
    }

    pub fn emit_with_ack<A>(&self, event_name: &str, args: impl Into<JsValue>, ack: A) -> &Self
    where
        A: FnOnce(JsValue) + 'static,
    {
        self.get_method("emit")
            .call3(
                &self.raw,
                &event_name.into(),
                &args.into(),
                &JsClosure::once_into_js(ack),
            )
            .unwrap();
        self
    }

    pub fn on<F>(&self, event_name: &str, listener: F) -> JsFunction
    where
        F: FnMut(JsValue) + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &event_name.into(), &func)
            .unwrap();
        func
    }

    pub fn once<F>(&self, event_name: &str, listener: F) -> JsFunction
    where
        F: FnOnce(JsValue) + 'static,
    {
        let func = JsClosure::once_into_js(listener).unchecked_into::<JsFunction>();
        self.get_method("once")
            .call2(&self.raw, &event_name.into(), &func)
            .unwrap();
        func
    }

    pub fn off(&self, listener: &JsFunction) -> &Self {
        self.get_method("off").call1(&self.raw, listener).unwrap();
        self
    }

    pub fn off_all_listener(&self, event_name: &str) -> &Self {
        self.get_method("off")
            .call1(&self.raw, &event_name.into())
            .unwrap();
        self
    }

    pub fn off_all(&self) -> &Self {
        self.get_method("off").call0(&self.raw).unwrap();
        self
    }

    pub fn listeners(&self, event_name: &str) -> Vec<JsFunction> {
        let l = self
            .get_method("listeners")
            .call1(&self.raw, &event_name.into())
            .unwrap()
            .unchecked_into::<JsArray>();
        let mut result = Vec::with_capacity(l.length() as usize);
        for i in l {
            result.push(i.unchecked_into::<JsFunction>());
        }
        result
    }

    pub fn on_any<T>(&self, mut listener: T) -> JsFunction
    where
        T: FnMut(&str, JsValue) + 'static,
    {
        let func = JsClosure::new(move |event: JsString, args: JsValue| {
            listener(event.as_string().unwrap().as_str(), args);
        })
        .into_js_value()
        .unchecked_into::<JsFunction>();
        self.get_method("onAny").call1(&self.raw, &func).unwrap();
        func
    }

    pub fn prepend_any<T>(&self, mut listener: T) -> JsFunction
    where
        T: FnMut(&str, JsValue) + 'static,
    {
        let func = JsClosure::new(move |event: JsString, args: JsValue| {
            listener(event.as_string().unwrap().as_str(), args);
        })
        .into_js_value()
        .unchecked_into::<JsFunction>();
        self.get_method("prependAny")
            .call1(&self.raw, &func)
            .unwrap();
        func
    }

    pub fn off_any(&self, listener: &JsFunction) -> &Self {
        self.get_method("offAny")
            .call1(&self.raw, listener)
            .unwrap();
        self
    }

    pub fn off_any_all(&self) -> &Self {
        self.get_method("offAny").call0(&self.raw).unwrap();
        self
    }

    pub fn listeners_any(&self) -> Vec<JsFunction> {
        let l = self
            .get_method("listenersAny")
            .call0(&self.raw)
            .unwrap()
            .unchecked_into::<JsArray>();
        let mut result = Vec::with_capacity(l.length() as usize);
        for i in l {
            result.push(i.unchecked_into::<JsFunction>());
        }
        result
    }

    pub fn on_any_outgoing<F>(&self, mut callback: F) -> JsFunction
    where
        F: FnMut(&str, JsValue) + 'static,
    {
        let func = JsClosure::new(move |event: JsString, args: JsValue| {
            callback(event.as_string().unwrap().as_str(), args);
        })
        .into_js_value()
        .unchecked_into::<JsFunction>();
        self.get_method("onAnyOutgoing")
            .call1(&self.raw, &func)
            .unwrap();
        func
    }

    pub fn prepend_any_outgoing<T>(&self, mut listener: T) -> JsFunction
    where
        T: FnMut(&str, JsValue) + 'static,
    {
        let func = JsClosure::new(move |event: JsString, args: JsValue| {
            listener(event.as_string().unwrap().as_str(), args);
        })
        .into_js_value()
        .unchecked_into::<JsFunction>();
        self.get_method("prependAnyOutgoing")
            .call1(&self.raw, &func)
            .unwrap();
        func
    }

    pub fn off_any_outgoing(&self, listener: &JsFunction) -> &Self {
        self.get_method("offAnyOutgoing")
            .call1(&self.raw, listener)
            .unwrap();
        self
    }

    pub fn off_any_outgoing_all(&self) -> &Self {
        self.get_method("offAny").call0(&self.raw).unwrap();
        self
    }

    pub fn listeners_any_outgoing(&self) -> Vec<JsFunction> {
        let l = self
            .get_method("listenersAnyOutgoing")
            .call0(&self.raw)
            .unwrap()
            .unchecked_into::<JsArray>();
        let mut result = Vec::with_capacity(l.length() as usize);
        for i in l {
            result.push(i.unchecked_into::<JsFunction>());
        }
        result
    }

    pub fn compress(&self, value: bool) -> &Self {
        self.get_method("compress")
            .call1(&self.raw, &value.into())
            .unwrap();
        self
    }

    pub fn timeout(&self, value: Duration) -> &Self {
        self.get_method("timeout")
            .call1(&self.raw, &(value.as_millis() as f64).into())
            .unwrap();
        self
    }

    pub fn disconnect(&self) -> &Self {
        self.get_method("disconnect").call0(&self.raw).unwrap();
        self
    }

    pub fn volatile(&self) -> &Self {
        JsReflect::get(&self.raw, &"volatile".into()).unwrap();
        self
    }
}

impl Socket {
    pub fn on_connect<F>(&self, listener: F) -> JsFunction
    where
        F: FnMut() + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"connect".into(), &func)
            .unwrap();
        func
    }

    pub fn once_connect<F>(&self, listener: F) -> JsFunction
    where
        F: FnOnce() + 'static,
    {
        let func = JsClosure::once_into_js(listener).unchecked_into::<JsFunction>();
        self.get_method("once")
            .call2(&self.raw, &"connect".into(), &func)
            .unwrap();
        func
    }

    pub fn on_disconnect<F>(&self, mut listener: F) -> JsFunction
    where
        F: FnMut(reason::DisconnectReason) + 'static,
    {
        let func = JsClosure::new(move |reason: JsString| {
            let reason =
                reason::DisconnectReason::from_str(reason.as_string().unwrap().as_str()).unwrap();
            listener(reason)
        })
        .into_js_value()
        .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"disconnect".into(), &func)
            .unwrap();
        func
    }

    pub fn once_disconnect<F>(&self, listener: F) -> JsFunction
    where
        F: FnOnce(reason::DisconnectReason) + 'static,
    {
        let func = JsClosure::once_into_js(move |reason: JsString| {
            let reason =
                reason::DisconnectReason::from_str(reason.as_string().unwrap().as_str()).unwrap();
            listener(reason)
        })
        .unchecked_into::<JsFunction>();
        self.get_method("once")
            .call2(&self.raw, &"disconnect".into(), &func)
            .unwrap();
        func
    }

    pub fn on_error<F>(&self, listener: F) -> JsFunction
    where
        F: FnMut(JsError) + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"error".into(), &func)
            .unwrap();
        func
    }

    pub fn once_error<F>(&self, listener: F) -> JsFunction
    where
        F: FnOnce(JsError) + 'static,
    {
        let func = JsClosure::once_into_js(listener).unchecked_into::<JsFunction>();
        self.get_method("once")
            .call2(&self.raw, &"error".into(), &func)
            .unwrap();
        func
    }
}

impl Socket {
    #[inline]
    fn get_method(&self, name: &str) -> JsFunction {
        JsReflect::get(&self.raw, &name.into())
            .unwrap()
            .unchecked_into::<JsFunction>()
    }
}

impl AsRef<JsObject> for Socket {
    fn as_ref(&self) -> &JsObject {
        &self.raw
    }
}

impl Into<JsObject> for Socket {
    fn into(self) -> JsObject {
        self.raw.into()
    }
}
