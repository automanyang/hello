// -- parse.rs --

use crate::utilities::snake_to_camel;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    punctuated::Punctuated, token::Comma, FieldValue, FnArg, Ident, ItemTrait, Expr, Lit,
    Member, ReturnType, Signature, TraitItem, TraitItemMethod,
};

// --

#[cfg_attr(test, derive(Debug))]
struct Attributes {
    proxy: Option<String>,
    servant: Option<String>,
    persistency: Option<bool>,
    callback: Option<bool>,
}
impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            proxy: None,
            servant: None,
            persistency: None,
            callback: None,
        };
        // if input.is_empty() {
        //     return Ok(r);
        // }

        let fields = Punctuated::<FieldValue, Comma>::parse_terminated(input)?;
        fields.iter().for_each(|x| {
            if let Member::Named(ref ident) = x.member {
                match ident.to_string().as_str() {
                    "proxy" => {
                        if let Expr::Lit(ref proxy) = x.expr {
                            if let Lit::Str(ref proxy) = proxy.lit {
                                r.proxy.replace(proxy.value());
                            }
                        }
                    }
                    "servant" => {
                        if let Expr::Lit(ref servant) = x.expr {
                            if let Lit::Str(ref servant) = servant.lit {
                                r.servant.replace(servant.value());
                            }
                        }
                    }
                    "persistency" => {
                        if let Expr::Lit(ref persistency) = x.expr {
                            if let Lit::Bool(ref persistency) = persistency.lit {
                                r.persistency.replace(persistency.value);
                            }
                        }
                    }
                    "callback" => {
                        if let Expr::Lit(ref callback) = x.expr {
                            if let Lit::Bool(ref callback) = callback.lit {
                                r.callback.replace(callback.value);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
        Ok(r)
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct TraitContext {
    item_trait: ItemTrait,
    fn_ident_vec: Vec<Ident>,
    fn_ident_camel_vec: Vec<Ident>,
    args_vec: Vec<Vec<TokenStream2>>,
    inputs_vec: Vec<Vec<TokenStream2>>,
    method_vec: Vec<TokenStream2>,
    output_vec: Vec<TokenStream2>,
    request_ident_vec: Vec<Ident>,
    request_ident: Ident,
    servant_ident: Ident,
    proxy_ident: Ident,
}

impl Parse for TraitContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let item_trait: ItemTrait = input.parse()?;
        let trait_ident = item_trait.ident.clone();

        let methods: Vec<TraitItemMethod> = item_trait
            .items
            .iter()
            .map(|i| {
                if let TraitItem::Method(m) = i {
                    Some(m)
                } else {
                    None
                }
            })
            .filter(|i| i.is_some())
            .map(|x| x.unwrap().clone())
            .collect();
        let idents_collected: Vec<_> = methods
            .iter()
            .map(|x| {
                let TraitItemMethod {
                    attrs,
                    sig,
                    default,
                    semi_token,
                } = x;
                let Signature {
                    constness,
                    asyncness,
                    unsafety,
                    abi,
                    fn_token,
                    ident,
                    generics,
                    // paren_token,
                    inputs,
                    variadic,
                    output,
                    ..
                } = sig;
                let fn_ident = ident;

                let output_type = match output.clone() {
                    ReturnType::Default => quote! {()},
                    ReturnType::Type(_, t) => quote! {#t},
                };
                let fn_ident_camel =
                    Ident::new(&snake_to_camel(&fn_ident.to_string()), fn_ident.span());
                let args: Vec<_> = inputs
                    .iter()
                    .map(|i| {
                        if let FnArg::Typed(pat) = i {
                            Some(pat)
                        } else {
                            None
                        }
                    })
                    .filter(|i| i.is_some())
                    .map(|x| {
                        let x = &x.unwrap().pat;
                        quote! {#x,}
                    })
                    .collect();
                let input_receiver: Vec<_> = inputs
                    .iter()
                    .map(|i| {
                        if let FnArg::Receiver(receiver) = i {
                            Some(receiver)
                        } else {
                            None
                        }
                    })
                    .filter(|i| i.is_some())
                    .map(|x| {
                        let x = x.unwrap();
                        quote! {#x,}
                    })
                    .collect();
                let inputs: Vec<_> = inputs
                    .iter()
                    .map(|i| {
                        if let FnArg::Typed(pat) = i {
                            Some(pat)
                        } else {
                            None
                        }
                    })
                    .filter(|i| i.is_some())
                    .map(|x| {
                        let x = x.unwrap();
                        quote! {#x,}
                    })
                    .collect();
                let method = quote! {
                    #(#attrs)*
                    #constness #asyncness #unsafety #abi #fn_token #fn_ident #generics (
                        #(#input_receiver)* ctx: Option<servant::Context>,
                        #(#inputs)* #variadic
                    ) #output
                    #default #semi_token
                };
                (fn_ident, fn_ident_camel, args, inputs, method, output_type)
            })
            .collect();
        let fn_ident_vec: Vec<_> = idents_collected.iter().map(|i| i.0.clone()).collect();
        let fn_ident_camel_vec: Vec<_> = idents_collected.iter().map(|i| i.1.clone()).collect();
        let args_vec: Vec<_> = idents_collected.iter().map(|i| i.2.clone()).collect();
        let inputs_vec: Vec<_> = idents_collected.iter().map(|i| i.3.clone()).collect();
        let method_vec: Vec<_> = idents_collected.iter().map(|i| i.4.clone()).collect();
        let output_vec: Vec<_> = idents_collected.iter().map(|i| i.5.clone()).collect();

        let request_ident = format_ident!("{}Request", trait_ident);
        let request_ident_vec: Vec<_> = idents_collected
            .iter()
            .map(|_| request_ident.clone())
            .collect();
        let servant_ident = format_ident!("{}Servant", trait_ident);
        let proxy_ident = format_ident!("{}Proxy", trait_ident);

        Ok(Self {
            item_trait,
            fn_ident_vec,
            fn_ident_camel_vec,
            args_vec,
            inputs_vec,
            method_vec,
            output_vec,
            request_ident_vec,
            request_ident,
            servant_ident,
            proxy_ident,
        })
    }
}

impl TraitContext {
    pub fn render_invoke_interface(&self, attr: TokenStream) -> TokenStream {
        let attributes = parse_macro_input!(attr as Attributes);
        let TraitContext {
            item_trait,
            fn_ident_vec,
            fn_ident_camel_vec,
            args_vec,
            inputs_vec,
            method_vec,
            output_vec,
            request_ident_vec,
            request_ident,
            servant_ident,
            proxy_ident,
        } = self;
        let ItemTrait {
            attrs,
            vis,
            unsafety,
            auto_token,
            trait_token,
            ident,
            generics,
            colon_token,
            supertraits,
            // brace_token,
            // items,
            ..
        } = item_trait;
        let trait_ident = ident;

        let output1 = if cfg!(any(feature = "adapter", feature = "terminal")) {
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                enum #request_ident {
                    #(#fn_ident_camel_vec { #(#inputs_vec)* },)*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output_persistence = match attributes.persistency {
            Some(p) if p => quote! {
                impl<S> servant::Servant for #servant_ident<S>
                where
                    S: serde::Serialize + #ident + 'static,
                {
                    fn name(&self) -> &str {
                        &self.name
                    }
                    fn dump(&self) -> servant::ServantResult<Vec<u8>> {
                        bincode::serialize(&self.entity).map_err(|e| e.to_string().into())
                    }
                    fn serve(&mut self, ctx: Option<servant::Context>, req: Vec<u8>) -> Vec<u8> {
                        let req: #request_ident = bincode::deserialize(&req).unwrap();
                        let reps = match req {
                            #(
                                #request_ident_vec::#fn_ident_camel_vec{ #(#args_vec)* } =>
                                    bincode::serialize(&self.entity.#fn_ident_vec(ctx, #(#args_vec)*)),
                            )*
                        }
                        .unwrap();
                        reps
                    }
                }
            },
            _ => quote! {
                impl<S> servant::Servant for #servant_ident<S>
                where
                    S: #trait_ident + 'static,
                {
                    fn name(&self) -> &str {
                        &self.name
                    }
                    fn serve(&mut self, ctx: Option<servant::Context>, req: Vec<u8>) -> Vec<u8> {
                        let req: #request_ident = bincode::deserialize(&req).unwrap();
                        let reps = match req {
                            #(
                                #request_ident_vec::#fn_ident_camel_vec{ #(#args_vec)* } =>
                                    bincode::serialize(&self.entity.#fn_ident_vec(ctx, #(#args_vec)*)),
                            )*
                        }
                        .unwrap();
                        reps
                    }
                }
            }
        };
        let output2 = if cfg!(feature = "adapter") {
            quote! {
                #( #attrs )*
                #vis #unsafety #auto_token #trait_token #trait_ident #generics #colon_token #supertraits {
                    #(#method_vec)*
                }
                pub struct #servant_ident<S>
                {
                    name: String,
                    entity: S,
                }
                impl<S> #servant_ident<S> {
                    pub fn new(name: &str, entity: S) -> Self {
                        Self { name: name.to_string(), entity }
                    }
                    pub fn category() -> &'static str {
                        stringify!(#trait_ident)
                    }
                }
                #output_persistence
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output3 = if cfg!(feature = "terminal") {
            quote! {
                #[derive(Clone)]
                pub struct #proxy_ident(servant::Context, servant::Oid, servant::Terminal);

                impl #proxy_ident {
                    pub fn new(ctx: servant::Context, name: &str, t: &servant::Terminal) -> Self {
                        let oid = servant::Oid::new(name, stringify!(#trait_ident));
                        Self(ctx, oid, t.clone())
                    }
                    pub fn category() -> &'static str {
                        stringify!(#trait_ident)
                    }

                    #(
                    pub async fn #fn_ident_vec(
                        &mut self,
                        #(#inputs_vec)*
                    ) -> servant::ServantResult<#output_vec> {
                        let request =  #request_ident_vec::#fn_ident_camel_vec { #(#args_vec)* };
                        let response = self
                            .2
                            .invoke(Some(self.0.clone()), Some(self.1.clone()), bincode::serialize(&request).unwrap())
                            .await;
                        response.map(|x| bincode::deserialize(&x).unwrap())
                    }
                    )*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let output5 = match attributes.callback {
            Some(cb) if cb => {
                quote! {
                    fn cb_true() {}
                }
            }
            _ => quote! {
                fn cb_false() {}
            }
        };

        let output = quote! {
            #output1
            #output2
            #output3
            #output5
        };
        output.into()
    }
    // pub fn render_query_interface(&self, _attr: TokenStream) -> TokenStream {
    // }
    // pub fn render_report_interface(&self, _attr: TokenStream) -> TokenStream {
    // }
    // pub fn render_notify_interface(&self, _attr: TokenStream) -> TokenStream {
    // }
}

// --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_parse() {
        // let attr: TokenStream = quote! {
        //     proxy="ppp", servant = "ssss", persistency = true, callback = false
        // }.into();
        // proxy="ppp", servant = "ssss", persistency = true, callback = false
        let attributes: Attributes = parse_quote! {
             proxy: "ppp", callback: false, persistency: true, servant: "ssss"
        };
        dbg!(&attributes);
        // dbg!(1);
    }
}
