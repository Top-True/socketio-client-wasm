use super::{OnAnyListener, OnListener, OnceListener};
use js_raw::*;

macro_rules! listener_impl {
    ($($t: ty),*) => {
        impl<L> OnListener<($($t,)*)> for L
        where
            L: FnMut($($t),*) + 'static,
        {
            fn into_js_function(self) -> JsFunction {
                JsClosure::new(self).into_js_value().unchecked_into()
            }
        }
        impl<L> OnceListener<($($t,)*)> for L
        where
            L: FnOnce($($t),*) + 'static,
        {
            fn into_js_function(self) -> JsFunction {
                JsClosure::once_into_js(self).unchecked_into()
            }
        }
    };
}

listener_impl!(JsValue);
listener_impl!(JsValue, JsValue);
listener_impl!(JsValue, JsValue, JsValue);
listener_impl!(JsValue, JsValue, JsValue, JsValue);
listener_impl!(JsValue, JsValue, JsValue, JsValue, JsValue);
listener_impl!(JsValue, JsValue, JsValue, JsValue, JsValue, JsValue);
listener_impl!(
    JsValue, JsValue, JsValue, JsValue, JsValue, JsValue, JsValue
);
listener_impl!(
    JsValue, JsValue, JsValue, JsValue, JsValue, JsValue, JsValue, JsValue
);

macro_rules! any_listener_impl {
    ($($i:ident => $t: ty),*) => {
        impl<L> OnAnyListener<($($t,)*)> for L
        where
            L: FnMut(&str, $($t),*) + 'static,
        {
            fn into_js_function(mut self) -> JsFunction {
                JsClosure::new(move |event: JsString, $($i: $t),*| {
                    self(event.as_string().unwrap().as_str(), $($i),*);
                }).into_js_value().unchecked_into()
            }
        }
    };
}

any_listener_impl!();
any_listener_impl!(a => JsValue);
any_listener_impl!(a => JsValue, b => JsValue);
any_listener_impl!(a => JsValue, b => JsValue, c => JsValue);
any_listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue);
any_listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue, e => JsValue);
any_listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue, e => JsValue, f => JsValue);
any_listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue, e => JsValue, f => JsValue, g => JsValue);
