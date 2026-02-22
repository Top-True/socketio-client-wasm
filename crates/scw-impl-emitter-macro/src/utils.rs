use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::{ExprClosure, Pat, Token, Type};

pub fn summon_idents(n: usize) -> Punctuated<Ident, Token![,]> {
    let mut punctuated = Punctuated::new();
    for i in 0..n {
        punctuated.push(Ident::new_raw(
            format!("i{}", i).as_str(),
            Span::call_site(),
        ));
    }
    punctuated
}
/// 高级类型提取器，支持多种闭包参数模式
pub struct ClosureTypeExtractor {
    pub types: Punctuated<Type, Token![,]>,
}

impl ClosureTypeExtractor {
    pub fn new() -> Self {
        Self {
            types: Punctuated::new(),
        }
    }

    /// 从闭包中提取参数类型
    pub fn extract_from_closure(&mut self, closure: &ExprClosure) {
        for (i, input) in closure.inputs.iter().enumerate() {
            self.extract_from_pat(input);

            if i < closure.inputs.len() - 1 {
                self.types.push_punct(Token![,](Span::call_site()));
            }
        }
    }

    /// 从模式中提取类型
    fn extract_from_pat(&mut self, pat: &Pat) {
        match pat {
            // 基础类型注解
            Pat::Type(pat_type) => {
                self.types.push_value(pat_type.ty.as_ref().clone());
            }
            // 元组模式
            Pat::Tuple(tuple) => {
                for pat in tuple.elems.iter() {
                    self.extract_from_pat(pat);
                }
            }
            // 引用模式（包含可变引用）
            Pat::Reference(reference) => {
                // 创建引用类型
                let mutability = if reference.mutability.is_some() {
                    Some(Token![mut](Span::call_site()))
                } else {
                    None
                };

                // 对于引用模式，我们可能需要特殊的处理
                // 这里我们只提取内部类型，然后构造引用类型
                let inner_pat = reference.pat.as_ref();
                let inner_types = Punctuated::new();
                let mut extractor = ClosureTypeExtractor { types: inner_types };
                extractor.extract_from_pat(inner_pat);

                if !extractor.types.is_empty() {
                    let inner_ty = extractor.types.into_iter().next().unwrap();
                    let ref_ty = Type::Reference(syn::TypeReference {
                        and_token: syn::token::And(Span::call_site()),
                        lifetime: None,
                        mutability,
                        elem: Box::new(inner_ty),
                    });
                    self.types.push_value(ref_ty);
                }
            }
            // 切片模式
            Pat::Slice(slice) => {
                // 切片类型：[T]
                if let Some(first) = slice.elems.first() {
                    let mut inner_extractor = ClosureTypeExtractor::new();
                    inner_extractor.extract_from_pat(first);

                    if !inner_extractor.types.is_empty() {
                        let elem_ty = inner_extractor.types.into_iter().next().unwrap();
                        let slice_ty = Type::Slice(syn::TypeSlice {
                            bracket_token: syn::token::Bracket(Span::call_site()),
                            elem: Box::new(elem_ty),
                        });
                        self.types.push_value(slice_ty);
                    }
                }
            }
            // 其他模式：使用默认占位符
            _ => {
                // 插入空类型作为占位符
                self.types
                    .push_value(Type::Verbatim(proc_macro2::TokenStream::new()));
            }
        }
    }
}
