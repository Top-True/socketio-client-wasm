pub mod options;

use js_raw::*;

#[derive(Debug, Clone)]
pub struct Socket {
    pub(crate) raw: JsObject,
}

impl Socket {
    pub fn new(raw: JsObject) -> Self {
        Self { raw }
    }

    pub fn id(&self) -> String {
        JsReflect::get(&self.raw, &"id".into())
            .unwrap()
            .as_string()
            .unwrap()
    }

    pub fn connected(&self) -> bool {
        JsReflect::get(&self.raw, &"connected".into())
            .unwrap()
            .as_bool()
            .unwrap()
    }

    pub fn connect(&self) {}

    pub fn disconnect(&self) {}

    pub fn on(&self) {}

    pub fn once(&self) {}

    pub fn emit(&self) {}

    pub fn emit_with_ack(&self) {}
}

impl Socket {
    #[inline]
    fn get_on_method(&self) -> JsFunction {
        JsReflect::get(&self.raw, &"on".into())
            .unwrap()
            .dyn_into()
            .unwrap()
    }

    pub fn on_error<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(JsValue) + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"error".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }

    pub fn on_ping<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut() + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"ping".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }

    pub fn on_reconnect<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(u32) + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"reconnect".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }

    pub fn on_reconnect_attempt<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(u32) + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"reconnect_attempt".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }

    pub fn on_reconnect_error<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut(JsValue) + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"reconnect_error".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }

    pub fn on_reconnect_failed<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut() + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"reconnect_failed".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }

    pub fn on_connect<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut() + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"connect".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }

    pub fn on_disconnect<F>(&self, listener: F) -> Result<(), JsValue>
    where
        F: FnMut() + 'static,
    {
        self.get_on_method().call2(
            &self.raw,
            &"disconnect".into(),
            &JsClosure::new(listener).into_js_value(),
        )?;
        Ok(())
    }
}
