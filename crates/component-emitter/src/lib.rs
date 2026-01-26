use js_raw::*;

pub mod listener_impl;

pub trait OnListener<A> {
    fn into_js_function(self) -> JsFunction;
}
pub trait OnceListener<A> {
    fn into_js_function(self) -> JsFunction;
}
pub trait OnAnyListener<A> {
    fn into_js_function(self) -> JsFunction;
}

pub trait Emitter {
    fn on<L, A>(&self, ev: &str, listener: L) -> JsFunction
    where
        L: OnListener<A>;
    fn once<L, A>(&self, ev: &str, listener: L) -> JsFunction
    where
        L: OnceListener<A>;
    fn off(&self, listener: &JsFunction) -> &Self;
    fn remove_all_listeners_from(&self, ev: &str) -> &Self;
    fn remove_all_listeners(&self) -> &Self;
    fn emit(&self, ev: &str) -> &Self;
    fn emit1(&self, ev: &str, arg: impl Into<JsValue>) -> &Self;
    fn emit_some(&self, ev: &str, args: impl IntoIterator<Item = JsValue>) -> &Self;
    fn listeners(&self, ev: &str) -> Vec<JsFunction>;
    fn has_listeners(&self, ev: &str) -> bool;
}

pub trait EmitterWithAck: Emitter {
    fn emit_with_ack(&self, ev: &str) -> JsFuture;
    fn emit1_with_ack(&self, ev: &str, arg: impl Into<JsValue>) -> JsFuture;
    fn emit_some_with_ack(&self, ev: &str, args: impl IntoIterator<Item = JsValue>) -> JsFuture;
}

pub trait EmitterWithJsRaw {
    fn from_raw(raw: JsObject) -> Self;
    fn raw(&self) -> &JsObject;
    #[inline]
    fn get_method(&self, name: &str) -> JsFunction {
        JsReflect::get(self.raw(), &name.into())
            .unwrap()
            .unchecked_into::<JsFunction>()
    }
}

impl<T> Emitter for T
where
    T: EmitterWithJsRaw,
{
    fn on<L, A>(&self, ev: &str, listener: L) -> JsFunction
    where
        L: OnListener<A>,
    {
        let func = listener.into_js_function();
        self.get_method("on")
            .call2(self.raw(), &ev.into(), &func)
            .unwrap();
        func
    }

    fn once<L, A>(&self, ev: &str, listener: L) -> JsFunction
    where
        L: OnceListener<A>,
    {
        let func = listener.into_js_function();
        self.get_method("once")
            .call2(self.raw(), &ev.into(), &func)
            .unwrap();
        func
    }

    fn off(&self, listener: &JsFunction) -> &Self {
        self.get_method("off").call1(self.raw(), listener).unwrap();
        self
    }

    fn remove_all_listeners_from(&self, ev: &str) -> &Self {
        self.get_method("off")
            .call1(self.raw(), &ev.into())
            .unwrap();
        self
    }

    fn remove_all_listeners(&self) -> &Self {
        self.get_method("off").call0(self.raw()).unwrap();
        self
    }

    fn emit(&self, ev: &str) -> &Self {
        self.get_method("emit")
            .call1(self.raw(), &ev.into())
            .unwrap();
        self
    }

    fn emit1(&self, ev: &str, arg: impl Into<JsValue>) -> &Self {
        self.get_method("emit")
            .call2(self.raw(), &ev.into(), &arg.into())
            .unwrap();
        self
    }

    fn emit_some(&self, ev: &str, args: impl IntoIterator<Item = JsValue>) -> &Self {
        let apply = JsArray::of1(&ev.into());
        for arg in args {
            apply.push(&arg);
        }
        self.get_method("emit").apply(self.raw(), &apply).unwrap();
        self
    }

    fn listeners(&self, ev: &str) -> Vec<JsFunction> {
        let l = self
            .get_method("listeners")
            .call1(self.raw(), &ev.into())
            .unwrap()
            .unchecked_into::<JsArray>();
        let mut result = Vec::with_capacity(l.length() as usize);
        for i in l {
            result.push(i.unchecked_into::<JsFunction>());
        }
        result
    }

    fn has_listeners(&self, ev: &str) -> bool {
        self.get_method("hasListeners")
            .call1(self.raw(), &ev.into())
            .unwrap()
            .as_bool()
            .unwrap()
    }
}
