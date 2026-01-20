pub mod options;

use js_raw::*;
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
    pub fn new(raw: JsObject) -> Self {
        Self { raw }
    }

    pub fn id(&self) -> JsUndefinedOption<SID> {
        let res = JsReflect::get(&self.raw, &"id".into()).unwrap().as_string();
        match res {
            None => JsUndefinedOption::Undefined,
            Some(s) => JsUndefinedOption::Some(SID(s)),
        }
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
        todo!()
    }

    pub fn connect(&self) {
        todo!()
    }

    pub fn send(&self, args: impl Into<JsValue>) {
        todo!()
    }

    pub fn send_with_ack<A>(&self, args: impl Into<JsValue>, ack: A)
    where
        A: FnOnce(JsValue),
    {
        todo!()
    }

    pub fn emit(&self, event_name: &str, args: impl Into<JsValue>) {
        todo!()
    }

    pub fn emit_with_ack<A>(&self, event_name: &str, args: impl Into<JsValue>, ack: A)
    where
        A: FnOnce(JsValue),
    {
        todo!()
    }

    pub fn on<T>(&self, event_name: &str, listener: JsListener) {
        todo!()
    }

    pub fn once<T>(&self, event_name: &str, listener: JsListener) {
        todo!()
    }

    pub fn off(&self, event_name: &str, listener_id: JsListenerID) {
        todo!()
    }

    pub fn off_all_listener(&self, event_name: &str) {
        todo!()
    }

    pub fn off_all(&self) {
        todo!()
    }

    pub fn listeners(&self, event_name: &str) -> Vec<JsListenerID> {
        todo!()
    }

    // todo
    // pub fn on_any<T>(&self, listener: T)
    // where
    //     T: FnMut(&str, JsValue) + 'static,
    // {
    //     todo!()
    // }

    // todo
    // pub fn prepend_any() {}

    // todo
    // pub fn off_any() {}

    // todo
    // pub fn listeners_any() {}

    // todo
    /*
    socket.onAnyOutgoing(callback)
    socket.prependAnyOutgoing(callback)
    socket.offAnyOutgoing(listener)
    socket.listenersAnyOutgoing()
    */

    pub fn compress(&self, value: bool) -> &Self {
        todo!()
    }

    pub fn timeout(&self, value: Duration) -> Socket {
        todo!()
    }

    pub fn disconnect(&self) {
        todo!()
    }

    pub fn volatile(&self) -> Socket {
        todo!()
    }
}

impl Socket {
    #[inline]
    fn get_on_method(&self) -> JsFunction {
        JsReflect::get(&self.raw, &"on".into())
            .unwrap()
            .dyn_into()
            .unwrap()
    }

    pub fn on_error<F>(&self, listener: JsListener) -> Result<(), JsValue>
    where
        F: FnMut(JsValue) + 'static,
    {
        self.get_on_method()
            .call2(&self.raw, &"error".into(), &listener.into_js_function())?;
        Ok(())
    }

    pub fn on_ping<F>(&self, listener: JsListener) -> Result<(), JsValue>
    where
        F: FnMut() + 'static,
    {
        self.get_on_method()
            .call2(&self.raw, &"ping".into(), &listener.into_js_function())?;
        Ok(())
    }

    pub fn on_reconnect<F>(&self, listener: JsListener) -> Result<(), JsValue>
    where
        F: FnMut(u32) + 'static,
    {
        self.get_on_method()
            .call2(&self.raw, &"reconnect".into(), &listener.into_js_function())?;
        Ok(())
    }

    pub fn on_reconnect_attempt<F>(&self, listener: JsListener) -> Result<(), JsValue>
    where
        F: FnMut(u32) + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"reconnect_attempt".into(),
            &listener.into_js_function(),
        )?;
        Ok(())
    }

    pub fn on_reconnect_error<F>(&self, listener: JsListener) -> Result<(), JsValue>
    where
        F: FnMut(JsValue) + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"reconnect_error".into(),
            &listener.into_js_function(),
        )?;
        Ok(())
    }

    pub fn on_reconnect_failed<F>(&self, listener: JsListener) -> Result<(), JsValue> {
        self.get_on_method().call2(
            &self.raw,
            &"reconnect_failed".into(),
            &listener.into_js_function(),
        )?;
        Ok(())
    }

    pub fn on_connect<F>(&self, listener: JsListener) -> Result<(), JsValue> {
        self.get_on_method()
            .call2(&self.raw, &"connect".into(), &listener.into_js_function())?;
        Ok(())
    }

    pub fn on_disconnect<F>(&self, listener: JsListener) -> Result<(), JsValue> {
        self.get_on_method().call2(
            &self.raw,
            &"disconnect".into(),
            &listener.into_js_function(),
        )?;
        Ok(())
    }
}

impl Into<JsValue> for Socket {
    fn into(self) -> JsValue {
        self.raw.into()
    }
}
