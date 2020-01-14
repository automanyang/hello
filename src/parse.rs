// -- parse.rs --

use crate::utilities::snake_to_camel;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Error, Expr, FieldValue, FnArg, Ident, ItemTrait, Lit, Member, ReturnType, Signature,
    TraitItem, TraitItemMethod,
};

// --

#[cfg(feature = "invoke")]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct InvokeInterfaceAttributes {
    proxy: Option<String>,
    servant: Option<String>,
    persistency: Option<bool>,
    callback: Option<bool>,
}
#[cfg(feature = "invoke")]
impl Parse for InvokeInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            proxy: None,
            servant: None,
            persistency: None,
            callback: None,
        };

        let fields = Punctuated::<FieldValue, Comma>::parse_terminated(input)?;
        for x in fields.iter() {
            if let Member::Named(ref ident) = x.member {
                match ident.to_string().as_str() {
                    "proxy" => {
                        if let Expr::Lit(ref proxy) = x.expr {
                            if let Lit::Str(ref proxy) = proxy.lit {
                                r.proxy.replace(proxy.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"HelloProxy\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'proxy: \"HelloProxy\"'",
                            ));
                        }
                    }
                    "servant" => {
                        if let Expr::Lit(ref servant) = x.expr {
                            if let Lit::Str(ref servant) = servant.lit {
                                r.servant.replace(servant.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"HelloServant\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'servant: \"HelloServant\"'",
                            ));
                        }
                    }
                    "persistency" => {
                        if let Expr::Lit(ref persistency) = x.expr {
                            if let Lit::Bool(ref persistency) = persistency.lit {
                                r.persistency.replace(persistency.value);
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a Bool value, such as 'true'",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'persistency: true'",
                            ));
                        }
                    }
                    "callback" => {
                        if let Expr::Lit(ref callback) = x.expr {
                            if let Lit::Bool(ref callback) = callback.lit {
                                r.callback.replace(callback.value);
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a Bool value, such as 'true'",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'callback: true'",
                            ));
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            "expected `proxy, servant, persistency or callback` only.",
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    x.span(),
                    "named filed accept only, can't accept unnamed field like '0, 1, 2'",
                ));
            }
        }
        Ok(r)
    }
}

// --

#[cfg(feature = "query")]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct QueryInterfaceAttributes {
    proxy: Option<String>,
    servant: Option<String>,
}
#[cfg(feature = "query")]
impl Parse for QueryInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            proxy: None,
            servant: None,
        };

        let fields = Punctuated::<FieldValue, Comma>::parse_terminated(input)?;
        for x in fields.iter() {
            if let Member::Named(ref ident) = x.member {
                match ident.to_string().as_str() {
                    "proxy" => {
                        if let Expr::Lit(ref proxy) = x.expr {
                            if let Lit::Str(ref proxy) = proxy.lit {
                                r.proxy.replace(proxy.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"QueryProxy\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'proxy: \"QueryProxy\"'",
                            ));
                        }
                    }
                    "servant" => {
                        if let Expr::Lit(ref servant) = x.expr {
                            if let Lit::Str(ref servant) = servant.lit {
                                r.servant.replace(servant.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"QueryServant\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'servant: \"QueryServant\"'",
                            ));
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            "expected `proxy' or 'servant` only.",
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    x.span(),
                    "named filed accept only, can't accept unnamed field like '0, 1, 2'",
                ));
            }
        }
        Ok(r)
    }
}

// --

#[cfg(feature = "report")]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct ReportInterfaceAttributes {
    proxy: Option<String>,
    servant: Option<String>,
}
#[cfg(feature = "report")]
impl Parse for ReportInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            proxy: None,
            servant: None,
        };

        let fields = Punctuated::<FieldValue, Comma>::parse_terminated(input)?;
        for x in fields.iter() {
            if let Member::Named(ref ident) = x.member {
                match ident.to_string().as_str() {
                    "proxy" => {
                        if let Expr::Lit(ref proxy) = x.expr {
                            if let Lit::Str(ref proxy) = proxy.lit {
                                r.proxy.replace(proxy.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"ReportProxy\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'proxy: \"ReportProxy\"'",
                            ));
                        }
                    }
                    "servant" => {
                        if let Expr::Lit(ref servant) = x.expr {
                            if let Lit::Str(ref servant) = servant.lit {
                                r.servant.replace(servant.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"ReportServant\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'servant: \"ReportServant\"'",
                            ));
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            "expected `proxy' or 'servant` only.",
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    x.span(),
                    "named filed accept only, can't accept unnamed field like '0, 1, 2'",
                ));
            }
        }
        Ok(r)
    }
}

// --

#[cfg(feature = "notify")]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct NotifyInterfaceAttributes {
    receiver: Option<String>,
    notifier: Option<String>,
}
#[cfg(feature = "notify")]
impl Parse for NotifyInterfaceAttributes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut r = Self {
            receiver: None,
            notifier: None,
        };

        let fields = Punctuated::<FieldValue, Comma>::parse_terminated(input)?;
        for x in fields.iter() {
            if let Member::Named(ref ident) = x.member {
                match ident.to_string().as_str() {
                    "receiver" => {
                        if let Expr::Lit(ref l) = x.expr {
                            if let Lit::Str(ref s) = l.lit {
                                r.receiver.replace(s.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"HelloReceiver\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'receiver: \"HelloReceiver\"'",
                            ));
                        }
                    }
                    "notifier" => {
                        if let Expr::Lit(ref l) = x.expr {
                            if let Lit::Str(ref s) = l.lit {
                                r.notifier.replace(s.value());
                            } else {
                                return Err(Error::new(
                                    ident.span(),
                                    "expected a &'static str value, such as \"HelloNotifier\"",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                "expected a value, such as 'servant: \"HelloNotifier\"'",
                            ));
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ident.span(),
                            "expected `proxy' or 'servant` only.",
                        ))
                    }
                }
            } else {
                return Err(Error::new(
                    x.span(),
                    "named filed accept only, can't accept unnamed field like '0, 1, 2'",
                ));
            }
        }
        Ok(r)
    }
}

// --
#[cfg(any(feature = "invoke", feature = "query", feature = "report", feature = "notify"))]
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

#[cfg(any(feature = "invoke", feature = "query", feature = "report", feature = "notify"))]
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
    #[cfg(feature = "invoke")]
    pub(crate) fn render_invoke_interface(&self, attributes: &InvokeInterfaceAttributes) -> TokenStream {
        let TraitContext {
            item_trait,
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
            },
        };
        let output2 = if cfg!(feature = "adapter") {
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
                        let request = #request_ident_vec::#fn_ident_camel_vec { #(#args_vec)* };
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
            },
        };

        let output = quote! {
            #output1
            #output2
            #output3
            #output5
        };
        output.into()
    }

    #[cfg(feature = "query")]
    pub(crate) fn render_query_interface(&self, attributes: &QueryInterfaceAttributes) -> TokenStream {
        let TraitContext {
            item_trait,
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

        let output2 = if cfg!(feature = "adapter") {
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
                impl<S> servant::QueryServant for #servant_ident<S>
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

        let output3 = if cfg!(feature = "terminal") {
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

    #[cfg(feature = "report")]
    pub(crate) fn render_report_interface(&self, attributes: &ReportInterfaceAttributes) -> TokenStream {
        let TraitContext {
            item_trait,
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
        let servant_ident = if let Some(ref s) = attributes.servant {
            Ident::new(&s, trait_ident.span())
        } else {
            format_ident!("{}ReportServant", trait_ident)
        };
        let proxy_ident = if let Some(ref p) = attributes.proxy {
            Ident::new(&p, trait_ident.span())
        } else {
            format_ident!("{}ReportProxy", trait_ident)
        };

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

        let output2 = if cfg!(feature = "adapter") {
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

        let output3 = if cfg!(feature = "terminal") {
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

    #[cfg(feature = "notify")]
    pub(crate) fn render_notify_interface(&self, attributes: &NotifyInterfaceAttributes) -> TokenStream {
        let TraitContext {
            item_trait,
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

        let output2 = if cfg!(feature = "terminal") {
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

        let output3 = if cfg!(feature = "adapter") {
            quote! {
                pub struct #notifier_ident(&'static servant::AdapterRegister);

                impl #notifier_ident {
                    pub fn instance() -> &'static #notifier_ident {
                        static mut NOTIFIER: Option<#notifier_ident> = None;
                        static INIT: std::sync::Once = std::sync::Once::new();
                        unsafe {
                            INIT.call_once(|| {
                                NOTIFIER = Some(#notifier_ident(servant::AdapterRegister::instance()));
                            });
                            NOTIFIER.as_ref().unwrap()
                        }
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
