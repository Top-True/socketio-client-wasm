use crate::engine_io;
use component_emitter::*;
use js_raw::*;
use std::time::Duration;

pub mod options;

#[derive(Debug, Clone)]
pub struct Manager {
    pub(crate) raw: JsObject,
}

impl EmitterWithJsRaw for Manager {
    fn from_raw(raw: JsObject) -> Manager {
        Manager { raw }
    }

    fn raw(&self) -> &JsObject {
        &self.raw
    }
}

impl Manager {
    pub fn new(
        uri: &impl AsRef<str>,
        manager_options: &options::Options,
        engine_options: &engine_io::Options,
    ) -> Self {
        let options = JsObject::new();
        JsObject::assign2(&options, &manager_options.to_js(), &engine_options.to_js());
        let manager_class = JsReflect::get(&global_io(), &"Manager".into())
            .unwrap()
            .unchecked_into::<JsFunction>();
        let raw = JsReflect::construct(
            &manager_class,
            &JsArray::of2(&uri.as_ref().into(), &options.into()),
        )
        .unwrap()
        .unchecked_into();
        Self { raw }
    }

    pub fn reconnection(&self) -> bool {
        self.get_method("reconnection")
            .call0(&self.raw)
            .unwrap()
            .as_bool()
            .unwrap()
    }

    pub fn set_reconnection(&self, value: bool) -> &Self {
        self.get_method("reconnection")
            .call1(&self.raw, &value.into())
            .unwrap();
        self
    }

    pub fn reconnection_attempts(&self) -> options::ReconnectionAttempts {
        self.get_method("reconnectionAttempts")
            .call0(&self.raw)
            .unwrap()
            .unchecked_into::<JsNumber>()
            .into()
    }

    pub fn set_reconnection_attempts(&self, value: u32) -> &Self {
        self.get_method("reconnectionAttempts")
            .call1(&self.raw, &value.into())
            .unwrap();
        self
    }

    pub fn reconnection_delay(&self) -> Duration {
        Duration::from_millis(
            self.get_method("reconnectionDelay")
                .call0(&self.raw)
                .unwrap()
                .unchecked_into::<JsNumber>()
                .as_f64()
                .unwrap() as u64,
        )
    }

    pub fn set_reconnection_delay(&self, value: Duration) -> &Self {
        self.get_method("reconnectionDelay")
            .call1(&self.raw, &value.millis_to_js())
            .unwrap();
        self
    }

    pub fn reconnection_delay_max(&self) -> Duration {
        Duration::from_millis(
            self.get_method("reconnectionDelayMax")
                .call0(&self.raw)
                .unwrap()
                .unchecked_into::<JsNumber>()
                .as_f64()
                .unwrap() as u64,
        )
    }

    pub fn set_reconnection_delay_max(&self, value: Duration) -> &Self {
        self.get_method("reconnectionDelayMax")
            .call1(&self.raw, &value.millis_to_js())
            .unwrap();
        self
    }

    pub fn timeout(&self) -> Option<Duration> {
        match self
            .get_method("timeout")
            .call0(&self.raw)
            .unwrap()
            .as_f64()
        {
            None => None,
            Some(x) => Some(Duration::from_millis(x as u64)),
        }
    }

    pub fn set_timeout(&self, value: Option<Duration>) -> &Self {
        let value = match value {
            None => JsValue::from_bool(false),
            Some(x) => x.millis_to_js(),
        };
        self.get_method("timeout").call1(&self.raw, &value).unwrap();
        self
    }

    pub fn open(&self) -> &Self {
        self.get_method("open").call0(&self.raw).unwrap();
        self
    }

    pub fn open_with_fail<F>(&self, fail: F) -> &Self
    where
        F: FnOnce(JsError) + 'static,
    {
        self.get_method("open")
            .call1(&self.raw, &JsClosure::once_into_js(fail))
            .unwrap();
        self
    }
}

impl_emitter_macro::impl_reserved! {
    Manager {
        open(),
        error(JsError),
        ping(),
        close(JsString),
        reconnect_failed(),
        reconnect_attempt(u32),
        reconnect_error(JsError),
        reconnect(u32),
    }
}

impl Into<JsObject> for Manager {
    fn into(self) -> JsObject {
        self.raw
    }
}

impl Into<JsValue> for Manager {
    fn into(self) -> JsValue {
        self.raw.into()
    }
}

impl AsRef<JsObject> for Manager {
    fn as_ref(&self) -> &JsObject {
        &self.raw
    }
}
