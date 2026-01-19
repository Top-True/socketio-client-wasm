use js_raw::*;

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
        gloo::console::log!(&raw);
        Self { raw }
    }

    pub fn reconnection(&self) -> bool {
        todo!()
    }

    pub fn set_reconnection(self, value: bool) -> Self {
        todo!()
    }
}
