// -- parse.rs --

use crate::utilities::snake_to_camel;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Error, FnArg, Ident, ItemTrait, Lit, MetaNameValue, ReturnType, Signature, TraitItem,
    TraitItemMethod,
};

// --

const PROXY_STR: &str = "proxy";
const SERVANT_STR: &str = "servant";
const PERSISTENCY_STR: &str = "persistency";
const CALLBACK_STR: &str = "callback";
const RECEIVER_STR: &str = "receiver";
const NOTIFIER_STR: &str = "notifier";

const VALUE_EXPECT_STR: &str = "value expected '&\'static str' only.";
const VALUE_EXPECT_BOOL: &str = "value expected 'bool' only.";

// --

#[cfg_attr(test, derive(Debug))]
pub(crate) struct InvokeInterfaceAttributes {
    proxy: Option<String>,
    servant: Option<String>,
    persistency: Option<bool>,
    callback: Option<bool>,
}
impl Parse for InvokeInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            proxy: None,
            servant: None,
            persistency: None,
            callback: None,
        };

        let args = Punctuated::<MetaNameValue, Comma>::parse_terminated(input)?;
        for MetaNameValue {
            path,
            // eq_token,
            lit,
            ..
        } in args.iter()
        {
            let err_str = Error::new(lit.span(), VALUE_EXPECT_STR);
            let err_bool = Error::new(lit.span(), VALUE_EXPECT_BOOL);

            if path.is_ident(PROXY_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.proxy.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else if path.is_ident(SERVANT_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.servant.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else if path.is_ident(CALLBACK_STR) {
                if let Lit::Bool(lit_bool) = lit {
                    let v = lit_bool.value;
                    r.callback.replace(v);
                } else {
                    Err(err_bool)?;
                }
            } else if path.is_ident(PERSISTENCY_STR) {
                if let Lit::Bool(lit_bool) = lit {
                    let v = lit_bool.value;
                    r.persistency.replace(v);
                } else {
                    Err(err_bool)?;
                }
            } else {
                Err(Error::new(
                    path.span(),
                    format!(
                        "name expected '{}', '{}', '{}' or '{}' only.",
                        PROXY_STR, SERVANT_STR, PERSISTENCY_STR, CALLBACK_STR
                    ),
                ))?;
            }
        }
        Ok(r)
    }
}

// --

#[cfg_attr(test, derive(Debug))]
pub(crate) struct WatchInterfaceAttributes {
    proxy: Option<String>,
    servant: Option<String>,
}
impl Parse for WatchInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            proxy: None,
            servant: None,
        };

        let args = Punctuated::<MetaNameValue, Comma>::parse_terminated(input)?;
        for MetaNameValue {
            path,
            // eq_token,
            lit,
            ..
        } in args.iter()
        {
            let err_str = Error::new(lit.span(), VALUE_EXPECT_STR);

            if path.is_ident(PROXY_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.proxy.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else if path.is_ident(SERVANT_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.servant.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else {
                Err(Error::new(
                    path.span(),
                    format!("name expected '{}' or '{}' only.", PROXY_STR, SERVANT_STR),
                ))?;
            }
        }
        Ok(r)
    }
}

// --

#[cfg_attr(test, derive(Debug))]
pub(crate) struct ReportInterfaceAttributes {
    proxy: Option<String>,
    servant: Option<String>,
}
impl Parse for ReportInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            proxy: None,
            servant: None,
        };

        let args = Punctuated::<MetaNameValue, Comma>::parse_terminated(input)?;
        for MetaNameValue {
            path,
            // eq_token,
            lit,
            ..
        } in args.iter()
        {
            let err_str = Error::new(lit.span(), VALUE_EXPECT_STR);

            if path.is_ident(PROXY_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.proxy.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else if path.is_ident(SERVANT_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.servant.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else {
                Err(Error::new(
                    path.span(),
                    format!("name expected '{}' or '{}' only.", PROXY_STR, SERVANT_STR),
                ))?;
            }
        }
        Ok(r)
    }
}

// --

#[cfg_attr(test, derive(Debug))]
pub(crate) struct NotifyInterfaceAttributes {
    receiver: Option<String>,
    notifier: Option<String>,
}
impl Parse for NotifyInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            receiver: None,
            notifier: None,
        };

        let args = Punctuated::<MetaNameValue, Comma>::parse_terminated(input)?;
        for MetaNameValue {
            path,
            // eq_token,
            lit,
            ..
        } in args.iter()
        {
            let err_str = Error::new(lit.span(), "value expected '&\'static str' only.");

            if path.is_ident(NOTIFIER_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.notifier.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else if path.is_ident(RECEIVER_STR) {
                if let Lit::Str(lit_str) = lit {
                    let v = lit_str.value();
                    r.receiver.replace(v);
                } else {
                    Err(err_str)?;
                }
            } else {
                Err(Error::new(
                    path.span(),
                    format!(
                        "name expected '{}' or '{}' only.",
                        NOTIFIER_STR, RECEIVER_STR
                    ),
                ))?;
            }
        }
        Ok(r)
    }
}

// --

// #[allow(unused)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct TraitContext {
    item_trait: ItemTrait,
    fn_ident_vec: Vec<Ident>,
    fn_ident_camel_vec: Vec<Ident>,
    args_vec: Vec<Vec<TokenStream2>>,
    inputs_vec: Vec<Vec<TokenStream2>>,
    method_with_context_vec: Vec<TokenStream2>,
    method_vec: Vec<TraitItemMethod>,
    output_vec: Vec<TokenStream2>,
    request_ident_vec: Vec<Ident>,
    request_ident: Ident,
}

impl Parse for TraitContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let item_trait: ItemTrait = input.parse()?;
        let trait_ident = item_trait.ident.clone();

        let method_vec: Vec<TraitItemMethod> = item_trait
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
        let idents_collected: Vec<_> = method_vec
            .iter()
            .map(|x| {
                let TraitItemMethod {
                    attrs,
                    sig:
                        Signature {
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
                        },
                    default,
                    semi_token,
                } = x;
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
                let method_with_context = quote! {
                    #(#attrs)*
                    #constness #asyncness #unsafety #abi #fn_token #fn_ident #generics (
                        #(#input_receiver)* ctx: Option<servant::Context>,
                        #(#inputs)* #variadic
                    ) #output
                    #default #semi_token
                };
                (
                    fn_ident,
                    fn_ident_camel,
                    args,
                    inputs,
                    method_with_context,
                    output_type,
                )
            })
            .collect();
        let fn_ident_vec: Vec<_> = idents_collected.iter().map(|i| i.0.clone()).collect();
        let fn_ident_camel_vec: Vec<_> = idents_collected.iter().map(|i| i.1.clone()).collect();
        let args_vec: Vec<_> = idents_collected.iter().map(|i| i.2.clone()).collect();
        let inputs_vec: Vec<_> = idents_collected.iter().map(|i| i.3.clone()).collect();
        let method_with_context_vec: Vec<_> =
            idents_collected.iter().map(|i| i.4.clone()).collect();
        let output_vec: Vec<_> = idents_collected.iter().map(|i| i.5.clone()).collect();

        let request_ident = format_ident!("{}Request", trait_ident);
        let request_ident_vec: Vec<_> = idents_collected
            .iter()
            .map(|_| request_ident.clone())
            .collect();

        Ok(Self {
            item_trait,
            fn_ident_vec,
            fn_ident_camel_vec,
            args_vec,
            inputs_vec,
            method_with_context_vec,
            method_vec,
            output_vec,
            request_ident_vec,
            request_ident,
        })
    }
}

impl TraitContext {
    pub(crate) fn render_invoke_interface(
        &self,
        attributes: &InvokeInterfaceAttributes,
    ) -> TokenStream {
        let TraitContext {
            item_trait:
                ItemTrait {
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
                },
            fn_ident_vec,
            fn_ident_camel_vec,
            args_vec,
            inputs_vec,
            method_with_context_vec,
            // method_vec,
            output_vec,
            request_ident_vec,
            request_ident,
            ..
        } = self;

        let fn_ident_callback_vec: Vec<_> = fn_ident_vec
            .iter()
            .map(|x| format_ident!("{}_with_callback", x))
            .collect();
        let trait_ident = ident;
        let servant_ident = if let Some(ref s) = attributes.servant {
            Ident::new(&s, trait_ident.span())
        } else {
            format_ident!("{}Servant", trait_ident)
        };
        let proxy_ident = if let Some(ref p) = attributes.proxy {
            Ident::new(&p, trait_ident.span())
        } else {
            format_ident!("{}Proxy", trait_ident)
        };

        let output1 = if cfg!(any(feature = "server", feature = "client")) {
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                enum #request_ident {
                    #(#fn_ident_camel_vec { #(#inputs_vec)* },)*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output_persistence = if attributes.persistency.unwrap_or(false) {
            quote! {
                impl<S> servant::Servant for #servant_ident<S>
                where
                    S: serde::Serialize + #trait_ident + 'static,
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
            }
        } else {
            quote! {
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
        let output2 = if cfg!(feature = "server") {
            quote! {
                #( #attrs )*
                #vis #unsafety #auto_token #trait_token #trait_ident #generics #colon_token #supertraits {
                    #(#method_with_context_vec)*
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

        let output_callback = if attributes.callback.unwrap_or(false) {
            quote! {
                #(
                    pub async fn #fn_ident_callback_vec<F>(
                        &mut self,
                        #(#inputs_vec)*
                        f_f_f_f_f_20101008_f: F
                    ) -> servant::ServantResult<()>
                    where F: 'static + Fn(servant::ServantResult<#output_vec>) + Send,
                    {
                        let request = #request_ident_vec::#fn_ident_camel_vec { #(#args_vec)* };
                        self.2
                            .invoke_with_callback(Some(self.0.clone()), Some(self.1.clone()),
                                bincode::serialize(&request).unwrap(), move |oid, v| {
                                    f_f_f_f_f_20101008_f(v.map(|x| bincode::deserialize(&x).unwrap()));
                                })
                            .await
                    }
                )*
            }
        } else {
            proc_macro2::TokenStream::new()
        };
        let output3 = if cfg!(feature = "client") {
            quote! {
                #[derive(Clone)]
                pub struct #proxy_ident(servant::Context, servant::Oid, servant::Terminal);

                impl #proxy_ident {
                    pub fn new(ctx: servant::Context, name: &str, t: &servant::Terminal) -> Self {
                        let oid = servant::Oid::new(name, Self::category());
                        Self(ctx, oid, t.clone())
                    }
                    pub fn category() -> &'static str {
                        stringify!(#trait_ident)
                    }
                    pub fn context_mut(&mut self) -> &mut servant::Context {
                        &mut self.0
                    }
                    pub fn terminal(&self) -> servant::Terminal {
                        self.2.clone()
                    }

                    #(
                        pub async fn #fn_ident_vec(
                            &mut self,
                            #(#inputs_vec)*
                        ) -> servant::ServantResult<#output_vec> {
                            let request = #request_ident_vec::#fn_ident_camel_vec { #(#args_vec)* };
                            let response = self
                                .2
                                .invoke(Some(self.0.clone()), Some(self.1.clone()), bincode::serialize(&request).unwrap())
                                .await;
                            response.map(|v| bincode::deserialize(&v).unwrap())
                        }
                    )*

                    #output_callback
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output = quote! {
            #output1
            #output2
            #output3
        };
        output.into()
    }

    pub(crate) fn render_watch_interface(
        &self,
        attributes: &WatchInterfaceAttributes,
    ) -> TokenStream {
        let TraitContext {
            item_trait:
                ItemTrait {
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
                },
            fn_ident_vec,
            fn_ident_camel_vec,
            args_vec,
            inputs_vec,
            // method_with_context_vec,
            method_vec,
            output_vec,
            request_ident_vec,
            request_ident,
            ..
        } = self;

        let trait_ident = ident;
        let servant_ident = if let Some(ref s) = attributes.servant {
            Ident::new(&s, trait_ident.span())
        } else {
            format_ident!("{}Servant", trait_ident)
        };
        let proxy_ident = if let Some(ref p) = attributes.proxy {
            Ident::new(&p, trait_ident.span())
        } else {
            format_ident!("{}Proxy", trait_ident)
        };

        let output1 = if cfg!(any(feature = "server", feature = "client")) {
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                enum #request_ident {
                    #(#fn_ident_camel_vec { #(#inputs_vec)* },)*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output2 = if cfg!(feature = "server") {
            quote! {
                #( #attrs )*
                #vis #unsafety #auto_token #trait_token #trait_ident #generics #colon_token #supertraits {
                    #(#method_vec)*
                }
                pub struct #servant_ident<S> {
                    entity: S,
                }
                impl<S> #servant_ident<S> {
                    pub fn new(entity: S) -> Self {
                        Self { entity }
                    }
                    pub fn category() -> &'static str {
                        stringify!(#trait_ident)
                    }
                }
                impl<S> servant::WatchServant for #servant_ident<S>
                where
                    S: #trait_ident + 'static,
                {
                    fn serve(&mut self, req: Vec<u8>) -> Vec<u8> {
                        let req: #request_ident = bincode::deserialize(&req).unwrap();
                        let reps = match req {
                            #(
                                #request_ident_vec::#fn_ident_camel_vec{ #(#args_vec)* } =>
                                    bincode::serialize(&self.entity.#fn_ident_vec(#(#args_vec)*)),
                            )*
                        }
                        .unwrap();
                        reps
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output3 = if cfg!(feature = "client") {
            quote! {
                #[derive(Clone)]
                pub struct #proxy_ident(servant::Terminal);

                impl #proxy_ident {
                    pub fn new(t: &servant::Terminal) -> Self {
                        Self(t.clone())
                    }
                    pub fn category() -> &'static str {
                        stringify!(#ident)
                    }

                    #(
                    pub async fn #fn_ident_vec(
                        &mut self,
                        #(#inputs_vec)*
                    ) -> servant::ServantResult<#output_vec> {
                        let request = #request_ident_vec::#fn_ident_camel_vec { #(#args_vec)* };
                        let response = self
                            .0
                            .invoke(None, None, bincode::serialize(&request).unwrap())
                            .await;
                        response.map(|x| bincode::deserialize(&x).unwrap())
                    }
                    )*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output = quote! {
            #output1
            #output2
            #output3
        };
        output.into()
    }

    pub(crate) fn render_report_interface(
        &self,
        attributes: &ReportInterfaceAttributes,
    ) -> TokenStream {
        let TraitContext {
            item_trait:
                ItemTrait {
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
                },
            fn_ident_vec,
            fn_ident_camel_vec,
            args_vec,
            inputs_vec,
            // method_with_context_vec,
            method_vec,
            // output_vec,
            request_ident_vec,
            request_ident,
            ..
        } = self;

        let trait_ident = ident;
        let servant_ident = if let Some(ref s) = attributes.servant {
            Ident::new(&s, trait_ident.span())
        } else {
            format_ident!("{}Officer", trait_ident)
        };
        let proxy_ident = if let Some(ref p) = attributes.proxy {
            Ident::new(&p, trait_ident.span())
        } else {
            format_ident!("{}Staff", trait_ident)
        };

        let output1 = if cfg!(any(feature = "server", feature = "client")) {
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                enum #request_ident {
                    #(#fn_ident_camel_vec { #(#inputs_vec)* },)*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output2 = if cfg!(feature = "server") {
            quote! {
                #( #attrs )*
                #vis #unsafety #auto_token #trait_token #trait_ident #generics #colon_token #supertraits {
                    #(#method_vec)*
                }
                pub struct #servant_ident<S> {
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
                impl<S> servant::ReportServant for #servant_ident<S>
                where
                    S: #trait_ident + 'static,
                {
                    fn name(&self) -> &str {
                        &self.name
                    }
                    fn serve(&mut self, req: Vec<u8>) {
                        let req: #request_ident = bincode::deserialize(&req).unwrap();
                        match req {
                            #(
                                #request_ident_vec::#fn_ident_camel_vec{ #(#args_vec)* } =>
                                    self.entity.#fn_ident_vec(#(#args_vec)*),
                            )*
                        }
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output3 = if cfg!(feature = "client") {
            quote! {
                #[derive(Clone)]
                pub struct #proxy_ident(servant::Oid, servant::Terminal);

                impl #proxy_ident {
                    pub fn new(name: &str, t: &servant::Terminal) -> Self {
                        let oid = servant::Oid::new(name, stringify!(#trait_ident));
                        Self(oid, t.clone())
                    }
                    pub fn category() -> &'static str {
                        stringify!(#trait_ident)
                    }

                    #(
                    pub async fn #fn_ident_vec(
                        &mut self,
                        #(#inputs_vec)*
                    ) -> servant::ServantResult<()> {
                        let request = #request_ident_vec::#fn_ident_camel_vec { #(#args_vec)* };
                        let response = self
                            .1
                            .report(self.0.clone(), bincode::serialize(&request).unwrap())
                            .await;
                        response
                    }
                    )*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output = quote! {
            #output1
            #output2
            #output3
        };
        output.into()
    }

    pub(crate) fn render_notify_interface(
        &self,
        attributes: &NotifyInterfaceAttributes,
    ) -> TokenStream {
        let TraitContext {
            item_trait:
                ItemTrait {
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
                },
            fn_ident_vec,
            fn_ident_camel_vec,
            args_vec,
            inputs_vec,
            // method_with_context_vec,
            method_vec,
            // output_vec,
            request_ident_vec,
            request_ident,
            ..
        } = self;

        let trait_ident = ident;
        let receiver_ident = if let Some(ref s) = attributes.receiver {
            Ident::new(&s, trait_ident.span())
        } else {
            format_ident!("{}Receiver", trait_ident)
        };
        let notifier_ident = if let Some(ref p) = attributes.notifier {
            Ident::new(&p, trait_ident.span())
        } else {
            format_ident!("{}Notifier", trait_ident)
        };

        let output1 = if cfg!(any(feature = "server", feature = "client")) {
            quote! {
                #[derive(serde::Serialize, serde::Deserialize)]
                enum #request_ident {
                    #(#fn_ident_camel_vec { #(#inputs_vec)* },)*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output2 = if cfg!(feature = "client") {
            quote! {
                #( #attrs )*
                #vis #unsafety #auto_token #trait_token #ident #generics #colon_token #supertraits {
                    #(#method_vec)*
                }
                pub struct #receiver_ident<S> {
                    entity: S,
                }
                impl<S> #receiver_ident<S> {
                    pub fn new(entity: S) -> Self {
                        Self { entity }
                    }
                }
                impl<S> servant::NotifyServant for #receiver_ident<S>
                where
                    S: #ident + 'static + Send,
                {
                    fn serve(&mut self, req: Vec<u8>) {
                        let req: #request_ident = bincode::deserialize(&req).unwrap();
                        match req {
                            #(
                                #request_ident_vec::#fn_ident_camel_vec{ #(#args_vec)* } =>
                                    self.entity.#fn_ident_vec(#(#args_vec)*),
                            )*
                        }
                    }
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output3 = if cfg!(feature = "server") {
            quote! {
                #[derive(Clone)]
                pub struct #notifier_ident(servant::AdapterRegister);
                impl #notifier_ident {
                    pub fn new(ar: servant::AdapterRegister) -> Self {
                        Self(ar)
                    }
                    #(
                    pub async fn #fn_ident_vec(
                        &self,
                        #(#inputs_vec)*
                    )  {
                        let request = #request_ident_vec::#fn_ident_camel_vec { #(#args_vec)* };
                        self
                            .0
                            .send(bincode::serialize(&request).unwrap())
                            .await
                    }
                    )*
                }
            }
        } else {
            proc_macro2::TokenStream::new()
        };

        let output = quote! {
            #output1
            #output2
            #output3
        };
        output.into()
    }
}

// --
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_parse() {
        let attributes: InvokeInterfaceAttributes = parse_quote! {
             proxy1: "ppp", callback: false, persistency: true, servant: "ssss"
        };
        dbg!(&attributes);
    }

    #[test]
    fn test_invoke_parse() {
        let attributes: InvokeInterfaceAttributes = parse_quote! {
            proxy: "ppp", callback: false, persistency: true, servant: "ssss"
        };
        dbg!(&attributes);
        let trait_context: TraitContext = parse_quote! {
            pub trait Hello3 {
                type Item;
                fn hi(&self, name: String) -> String;
            }
        };
        dbg!(&trait_context);

        // let o = trait_context.render_invoke_interface(attributes);
        // dbg!(&o);
    }
}
*/
