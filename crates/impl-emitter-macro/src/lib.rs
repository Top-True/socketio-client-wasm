use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::LitStr;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{ExprClosure, Token, Type};
use syn::{braced, parenthesized};

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
            if t.capture.is_some() {
                panic!("capture function should not have captured here");
            }
            t.capture = Some(Token![move](Span::call_site()));
            layer = Some(t);
        }
        Ok(Self { name, args, layer })
    }
}

#[proc_macro]
pub fn impl_reserved(input: TokenStream) -> TokenStream {
    let parsed = syn::parse::<ImplReserved>(input).expect("Syntax Error: impl_reserved");
    let name = parsed.name;
    let mut on_impls = Vec::with_capacity(parsed.events.len());
    let mut once_impls = Vec::with_capacity(parsed.events.len());
    for event in parsed.events {
        let event_name = LitStr::new(event.name.to_string().as_str(), event.name.span());
        let name = Ident::new(
            event.name.to_string().to_snake_case().as_str(),
            event.name.span(),
        );
        let args = event.args;
        match event.layer {
            None => {
                on_impls.push({
                    let fn_name = format_ident!("on_{}", name);
                    quote! {
                        pub fn #fn_name<F>(&self, listener: F) -> ::js_raw::JsFunction
                        where
                            F: FnMut(#args) + 'static
                        {
                            let func = ::js_raw::JsClosure::new(listener)
                                .into_js_value()
                                .unchecked_into::<::js_raw::JsFunction>();
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
                        pub fn #fn_name<F>(&self, listener: F) -> ::js_raw::JsFunction
                        where
                            F: FnOnce(#args) + 'static
                        {
                            let func = ::js_raw::JsClosure::once_into_js(listener).unchecked_into::<::js_raw::JsFunction>();
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
                        pub fn #fn_name<F>(&self, mut listener: F) -> ::js_raw::JsFunction
                        where
                            F: FnMut(#args) + 'static
                        {
                            let func = ::js_raw::JsClosure::new(#layer)
                                .into_js_value()
                                .unchecked_into::<::js_raw::JsFunction>();
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
                        pub fn #fn_name<F>(&self, listener: F) -> ::js_raw::JsFunction
                        where
                            F: FnOnce(#args) + 'static
                        {
                            let func = ::js_raw::JsClosure::once_into_js(#layer).unchecked_into::<::js_raw::JsFunction>();
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
