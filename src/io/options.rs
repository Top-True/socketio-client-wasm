use js_raw::*;

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub force_new: JsUndefinedOption<bool>,
    pub multiplex: JsUndefinedOption<bool>,
}

impl Into<JsValue> for Options {
    fn into(self) -> JsValue {
        let result = JsObject::new();
        set_property(&result, "forceNew", &self.force_new.into());
        set_property(&result, "multiplex", &self.multiplex.into());
        result.into()
    }
}
