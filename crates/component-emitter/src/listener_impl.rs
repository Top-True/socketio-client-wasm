use super::{OnAnyListener, OnListener, OnceListener};
use js_raw::*;
use std::sync::{Arc, Mutex};

macro_rules! listener_impl {
    ($($i: ident=>$t: ty),*) => {
        impl<L> OnListener<($($t,)*), false> for L
        where
            L: FnMut($($t),*) + 'static,
        {
            fn into_js_function(self) -> JsFunction {
                JsClosure::new(self).into_js_value().unchecked_into()
            }
        }
        impl<L> OnListener<($($t,)*), true> for L
        where
            L: AsyncFnMut($($t),*) + 'static,
        {
            fn into_js_function(self) -> JsFunction {
                let raw = Arc::new(Mutex::new(self));
                let layer = move |$($i),*| {
                    let raw = raw.clone();
                    future_to_promise(async move {
                        raw.lock().unwrap()($($i),*).await;
                        Ok(JsValue::undefined())
                    })
                };
                JsClosure::<dyn FnMut($($t),*) -> JsPromise>::new(layer).into_js_value()
                    .unchecked_into()
            }
        }
        impl<L> OnceListener<($($t,)*), false> for L
        where
            L: FnOnce($($t),*) + 'static,
        {
            fn into_js_function(self) -> JsFunction {
                JsClosure::once_into_js(self).unchecked_into()
            }
        }
        impl<L> OnceListener<($($t,)*), true> for L
        where
            L: AsyncFnOnce($($t),*) + 'static,
        {
            fn into_js_function(self) -> JsFunction {
                let layer = move |$($i),*| {
                    future_to_promise(async move {
                        self($($i),*).await;
                        Ok(JsValue::undefined())
                    })
                };
                JsClosure::once_into_js(layer).unchecked_into()
            }
        }
    };
}

listener_impl!();
listener_impl!(a => JsValue);
listener_impl!(a => JsValue, b => JsValue);
listener_impl!(a => JsValue, b => JsValue, c => JsValue);
listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue);
listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue, f => JsValue);
listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue, f => JsValue, g => JsValue);
listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue, f => JsValue, g => JsValue, h => JsValue);
listener_impl!(a => JsValue, b => JsValue, c => JsValue, d => JsValue, f => JsValue, g => JsValue, h => JsValue, i => JsValue);

macro_rules! any_listener_impl {
    ($($i:ident => $t: ty),*) => {
        impl<L> OnAnyListener<($($t,)*), false> for L
        where
            L: FnMut(&str, $($t),*) + 'static,
        {
            fn into_js_function(mut self) -> JsFunction {
                JsClosure::new(move |event: JsString, $($i: $t),*| {
                    self(event.as_string().unwrap().as_str(), $($i),*);
                }).into_js_value().unchecked_into()
            }
        }
        impl<L> OnAnyListener<($($t,)*), true> for L
        where
            L: AsyncFnMut(&str, $($t),*) + 'static,
        {
            fn into_js_function(self) -> JsFunction {
                let raw = Arc::new(Mutex::new(self));
                let layer = move |event: JsString, $($i: $t),*| {
                    let raw = raw.clone();
                    future_to_promise(async move {
                        raw.lock().unwrap()(event.as_string().unwrap().as_str(), $($i),*).await;
                        Ok(JsValue::undefined())
                    })
                };
                JsClosure::<dyn FnMut(JsString, $($t),*) -> JsPromise>::new(layer).into_js_value()
                    .unchecked_into()
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
