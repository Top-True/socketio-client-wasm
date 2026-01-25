use js_raw::*;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub force_new: JsUndefinedOption<bool>,
    pub multiplex: JsUndefinedOption<bool>,
}

impl ToJs<JsObject> for Options {
    fn to_js(&self) -> JsObject {
        let result = JsObject::new();
        set_property(&result, "forceNew", &self.force_new.to_js());
        set_property(&result, "multiplex", &self.multiplex.to_js());
        result
    }
}

impl ToJs<JsValue> for Options {
    fn to_js(&self) -> JsValue {
        ToJs::<JsObject>::to_js(self).into()
    }
}
