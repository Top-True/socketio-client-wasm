use scw_js_raw::*;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub force_new: JsOption<bool>,
    pub multiplex: JsOption<bool>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        self.force_new
            .if_some_then(|x| set_property(&result, "forceNew", &x));
        self.multiplex
            .if_some_then(|x| set_property(&result, "multiplex", &x));
        result
    }
}

impl ToJs<JsValue> for Options {
    fn to_js(&self) -> JsValue {
        ToJs::<JsObject>::to_js(self).into()
    }
}
