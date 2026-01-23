use js_raw::*;
use std::time::Duration;

pub mod options;
pub mod parser;

#[derive(Debug, Clone)]
pub struct Manager {
    pub(crate) raw: JsObject,
}

impl Manager {
    pub fn new(url: &impl AsRef<str>, options: options::Options) -> Self {
        let manager_class = JsReflect::get(&global_io(), &"Manager".into())
            .unwrap()
            .unchecked_into::<JsFunction>();
        let raw = JsReflect::construct(
            &manager_class,
            &JsArray::of2(&url.as_ref().into(), &options.into()),
        )
        .unwrap()
        .unchecked_into();
        Self { raw }
    }

    pub fn reconnection(&self) -> bool {
        todo!()
    }

    pub fn set_reconnection(&self, value: bool) -> &Self {
        todo!()
    }

    pub fn reconnection_attempts(&self) -> u32 {
        todo!()
    }

    pub fn set_reconnection_attempts(&self, value: u32) -> &Self {
        todo!()
    }

    pub fn reconnection_delay(&self) -> Duration {
        todo!()
    }

    pub fn set_reconnection_delay(&self, value: Duration) -> &Self {
        todo!()
    }

    pub fn reconnection_delay_max(&self) -> Duration {
        todo!()
    }

    pub fn set_reconnection_delay_max(&self, value: Duration) -> &Self {
        todo!()
    }

    pub fn timeout(&self) -> Duration {
        todo!()
    }

    pub fn set_timeout(&self, timeout: Duration) -> &Self {
        todo!()
    }

    pub fn open(&self) -> JsUndefinedOption<JsFunction> {
        todo!()
    }

    pub fn set_open<S, F>(&self, succeed: S, fail: F) -> &Self
    where
        S: FnOnce(),
        F: FnOnce(JsError),
    {
        todo!()
    }
}

impl Manager {
    pub fn on_ping<F>(&self, listener: F) -> JsFunction
    where
        F: FnMut() + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"ping".into(), &func)
            .unwrap();
        func
    }

    pub fn on_reconnect<F>(&self, listener: F) -> JsFunction
    where
        F: FnMut(u32) + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"reconnect".into(), &func)
            .unwrap();
        func
    }

    pub fn on_reconnect_attempt<F>(&self, listener: F) -> JsFunction
    where
        F: FnMut(u32) + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"reconnect_attempt".into(), &func)
            .unwrap();
        func
    }

    pub fn on_reconnect_error<F>(&self, listener: F) -> JsFunction
    where
        F: FnMut(JsValue) + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"reconnect_error".into(), &func)
            .unwrap();
        func
    }

    pub fn on_reconnect_failed<F>(&self, listener: F) -> JsFunction
    where
        F: FnMut(JsValue) + 'static,
    {
        let func = JsClosure::new(listener)
            .into_js_value()
            .unchecked_into::<JsFunction>();
        self.get_method("on")
            .call2(&self.raw, &"reconnect_failed".into(), &func)
            .unwrap();
        func
    }
}

impl Manager {
    #[inline]
    fn get_method(&self, name: &str) -> JsFunction {
        JsReflect::get(&self.raw, &name.into())
            .unwrap()
            .unchecked_into::<JsFunction>()
    }
}

impl Into<JsObject> for Manager {
    fn into(self) -> JsObject {
        self.raw
    }
}

impl AsRef<JsObject> for Manager {
    fn as_ref(&self) -> &JsObject {
        &self.raw
    }
}
