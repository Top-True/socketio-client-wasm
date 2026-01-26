pub mod options;
pub mod reason;

use component_emitter::*;
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

impl EmitterWithJsRaw for Socket {
    fn from_raw(raw: JsObject) -> Socket {
        Socket { raw }
    }

    fn raw(&self) -> &JsObject {
        &self.raw
    }
}

impl EmitterWithAck for Socket {
    fn emit_with_ack(&self, ev: &str) -> JsFuture {
        let promise = self
            .get_method("emitWithAck")
            .call1(&self.raw, &ev.into())
            .unwrap()
            .unchecked_into::<JsPromise>();
        JsFuture::from(promise)
    }

    fn emit1_with_ack(&self, ev: &str, arg: impl Into<JsValue>) -> JsFuture {
        let promise = self
            .get_method("emitWithAck")
            .call2(&self.raw, &ev.into(), &arg.into())
            .unwrap()
            .unchecked_into::<JsPromise>();
        JsFuture::from(promise)
    }

    fn emit_some_with_ack(&self, ev: &str, args: impl IntoIterator<Item = JsValue>) -> JsFuture {
        let apply = JsArray::of1(&ev.into());
        for arg in args {
            apply.push(&arg);
        }
        let promise = self
            .get_method("emitWithAck")
            .apply(&self.raw, &apply)
            .unwrap()
            .unchecked_into::<JsPromise>();
        JsFuture::from(promise)
    }
}

impl Socket {
    pub fn new(
        io: crate::manager::Manager,
        nsp: &impl AsRef<str>,
        opts: &options::Options,
    ) -> Self {
        let socket_class = JsReflect::get(&js_global(), &"Socket".into())
            .unwrap()
            .unchecked_into::<JsFunction>();
        let raw = JsReflect::construct(
            &socket_class,
            &JsArray::of3(io.as_ref(), &nsp.as_ref().into(), &opts.to_js()),
        )
        .unwrap()
        .unchecked_into();
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
        crate::manager::Manager::from_raw(
            JsReflect::get(&self.raw, &"io".into())
                .unwrap()
                .unchecked_into(),
        )
    }

    pub fn connect(&self) -> &Self {
        self.get_method("connect").call0(&self.raw).unwrap();
        self
    }

    pub fn on_any<A>(&self, listener: impl OnAnyListener<A>) -> JsFunction {
        let func = listener.into_js_function();
        self.get_method("onAny").call1(&self.raw, &func).unwrap();
        func
    }

    pub fn prepend_any<A>(&self, listener: impl OnAnyListener<A>) -> JsFunction {
        let func = listener.into_js_function();
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

    pub fn on_any_outgoing<A>(&self, listener: impl OnAnyListener<A>) -> JsFunction {
        let func = listener.into_js_function();
        self.get_method("onAnyOutgoing")
            .call1(&self.raw, &func)
            .unwrap();
        func
    }

    pub fn prepend_any_outgoing<A>(&self, listener: impl OnAnyListener<A>) -> JsFunction {
        let func = listener.into_js_function();
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
        self.get_method("offAnyOutgoing").call0(&self.raw).unwrap();
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

impl_emitter_macro::impl_reserved! {
    Socket {
        connect(),
        connect_error(JsError),
        disconnect(reason::DisconnectReason) => |reason: JsString| {
            let reason =
                reason::DisconnectReason::from_str(reason.as_string().unwrap().as_str()).unwrap();
            listener(reason)
        },
        newListener(&str, JsFunction) => |event_name: JsString, new_listener: JsFunction| {
            let event_name = event_name.as_string().unwrap();
            listener(event_name.as_str(), new_listener)
        },
        removeListener(&str, JsFunction) => |event_name: JsString, new_listener: JsFunction| {
            let event_name = event_name.as_string().unwrap();
            listener(event_name.as_str(), new_listener)
        },
    }
}

impl Into<JsObject> for Socket {
    fn into(self) -> JsObject {
        self.raw
    }
}

impl Into<JsValue> for Socket {
    fn into(self) -> JsValue {
        self.raw.into()
    }
}

impl AsRef<JsObject> for Socket {
    fn as_ref(&self) -> &JsObject {
        &self.raw
    }
}
