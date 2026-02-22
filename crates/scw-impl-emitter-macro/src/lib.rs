use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::LitStr;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{ExprClosure, Token, Type, parse_macro_input};
use syn::{braced, parenthesized};

mod utils;

struct ImplReserved {
    name: Ident,
    events: Punctuated<Events, Token![,]>,
}

impl Parse for ImplReserved {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        Ok(ImplReserved {
            name,
            events: content.parse_terminated(Events::parse, Token![,])?,
        })
    }
}

struct Events {
    name: Ident,
    args: Punctuated<Type, Token![,]>,
    layer: Option<ExprClosure>,
}

impl Parse for Events {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let args;
        parenthesized!(args in input);
        let args = args.parse_terminated(Type::parse, Token![,])?;
        let mut layer = None;
        if input.parse::<Token![=>]>().is_ok() {
            let mut t = ExprClosure::parse(input).expect("failed to parse closure");
            if let Some(c) = t.capture {
                return Err(syn::Error::new(
                    c.span,
                    "capture function should not have captured here",
                ));
            }
            t.capture = Some(Token![move](Span::call_site()));
            layer = Some(t);
        }
        Ok(Self { name, args, layer })
    }
}

#[proc_macro]
pub fn impl_reserved(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ImplReserved);
    let name = parsed.name;
    let mut on_impls = Vec::with_capacity(parsed.events.len() * 2);
    let mut once_impls = Vec::with_capacity(parsed.events.len() * 2);
    for event in parsed.events {
        let event_name = LitStr::new(event.name.to_string().as_str(), event.name.span());
        let name = Ident::new(
            event.name.to_string().to_snake_case().as_str(),
            event.name.span(),
        );
        let args = event.args;
        let idents = utils::summon_idents(args.len());
        match event.layer {
            None => {
                on_impls.push({
                    let fn_name = format_ident!("on_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: FnMut(#args) + 'static
                        {
                            let func = ::scw_js_raw::JsClosure::new(listener)
                                .into_js_value()
                                .unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("on")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
                on_impls.push({
                    let fn_name = format_ident!("async_on_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: AsyncFnMut(#args) + 'static
                        {
                            let raw = ::std::sync::Arc::new(::std::sync::Mutex::new(listener));
                            let func = ::scw_js_raw::JsClosure::<dyn FnMut(#args) -> ::scw_js_raw::JsPromise>::new(move |#idents| {
                                let raw = raw.clone();
                                ::scw_js_raw::future_to_promise(async move {
                                    raw.lock().unwrap()(#idents).await;
                                    Ok(JsValue::undefined())
                                })})
                                .into_js_value()
                                .unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("on")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
                once_impls.push({
                    let fn_name = format_ident!("once_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: FnOnce(#args) + 'static
                        {
                            let func = ::scw_js_raw::JsClosure::once_into_js(listener).unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("once")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
                once_impls.push({
                    let fn_name = format_ident!("async_once_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: AsyncFnOnce(#args) + 'static
                        {
                            let func = ::scw_js_raw::JsClosure::once_into_js(move |#idents| {
                            future_to_promise(async move {
                                listener(#idents).await;
                                Ok(::scw_js_raw::JsValue::undefined())
                            })
                        }).unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("once")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
            }
            Some(layer) => {
                on_impls.push({
                    let fn_name = format_ident!("on_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, mut listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: FnMut(#args) + 'static
                        {
                            let func = ::scw_js_raw::JsClosure::new(#layer)
                                .into_js_value()
                                .unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("on")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
                on_impls.push({
                    let fn_name = format_ident!("async_on_{}", name);
                    let mut extractor = utils::ClosureTypeExtractor::new();
                    extractor.extract_from_closure(&layer);
                    let pat = extractor.types;
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: AsyncFnMut(#args) + 'static
                        {
                            let raw = ::std::sync::Arc::new(::std::sync::Mutex::new(listener));
                            let listener = move |#idents| {
                                let raw = raw.clone();
                                ::scw_js_raw::future_to_promise(async move {
                                    raw.lock().unwrap()(#idents).await;
                                    Ok(::scw_js_raw::JsValue::undefined())
                                })
                            };
                            let func = ::scw_js_raw::JsClosure::<dyn FnMut(#pat) -> ::scw_js_raw::JsPromise>::new(#layer)
                                .into_js_value()
                                .unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("on")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
                once_impls.push({
                    let fn_name = format_ident!("once_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: FnOnce(#args) + 'static
                        {
                            let func = ::scw_js_raw::JsClosure::once_into_js(#layer).unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("once")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
                once_impls.push({
                    let fn_name = format_ident!("async_once_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::scw_js_raw::JsFunction
                        where
                            F: AsyncFnOnce(#args) + 'static
                        {
                            let listener = move |#idents| {
                                ::scw_js_raw::future_to_promise(async move {
                                    listener(#idents).await;
                                    Ok(JsValue::undefined())
                                })
                            };
                            let func = ::scw_js_raw::JsClosure::once_into_js(#layer).unchecked_into::<::scw_js_raw::JsFunction>();
                            self.get_method("once")
                                .call2(&self.raw, &#event_name.into(), &func)
                                .unwrap();
                            func
                        }
                    }
                });
            }
        }
    }
    quote! {
        impl #name {
            #(#on_impls)*
            #(#once_impls)*
        }
    }
    .into()
}
