use js_raw::*;
use std::time::Duration;

pub mod options;
pub mod parser;

#[derive(Debug, Clone)]
pub struct Manager {
    raw: JsObject,
}

impl Manager {
    pub fn new(url: &impl AsRef<str>, options: options::Options) -> Self {
        let manager_class = JsReflect::get(&global_io(), &"Manager".into())
            .unwrap()
            .dyn_into::<JsFunction>()
            .unwrap();
        let raw = JsReflect::construct(
            &manager_class,
            &JsArray::of2(&url.as_ref().into(), &options.into()),
        )
        .unwrap()
        .dyn_into()
        .unwrap();
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
    pub fn on_error<CB>(&self, cb: CB)
    where
        CB: FnMut(JsError),
    {
        todo!()
    }

    pub fn on_reconnect<CB>(&self, cb: CB)
    where
        CB: FnMut(u32),
    {
        todo!()
    }

    pub fn on_reconnect_attempt<CB>(&self, cb: CB)
    where
        CB: FnMut(u32),
    {
        todo!()
    }

    pub fn on_reconnect_error<CB>(&self, cb: CB)
    where
        CB: FnMut(JsError),
    {
        todo!()
    }

    pub fn on_reconnect_failed<CB>(&self, cb: CB)
    where
        CB: FnMut(),
    {
        todo!()
    }

    pub fn on_ping<CB>(&self, cb: CB)
    where
        CB: FnMut(),
    {
        todo!()
    }
}

impl Into<JsValue> for Manager {
    fn into(self) -> JsValue {
        self.raw.into()
    }
}
