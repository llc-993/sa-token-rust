// Author: 金书记
//
//! 多角色检查宏（OR逻辑）

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, Token, parse::Parser};

/// 同时检查多个角色（OR逻辑）
/// 
/// 用户只需拥有任意一个指定的角色即可访问
/// 
/// # 示例
/// 
/// ```rust,ignore
/// #[sa_check_roles_or("admin", "moderator")]
/// async fn moderate_content() -> impl Responder {
///     // 拥有 admin 或 moderator 任一角色即可
///     "Content moderated"
/// }
/// ```
pub fn sa_check_roles_or_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    let parser = syn::punctuated::Punctuated::<LitStr, Token![,]>::parse_terminated;
    let roles = parser.parse(attr).unwrap_or_default();
    let role_values: Vec<String> = roles.iter().map(|r| r.value()).collect();
    let role_str = role_values.join(",");
    
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_body = &input.block;
    let fn_attrs = &input.attrs;
    let fn_vis = &input.vis;
    let fn_asyncness = &input.sig.asyncness;
    let fn_generics = &input.sig.generics;
    
    let expanded: TokenStream2 = quote! {
        #(#fn_attrs)*
        #[doc(hidden)]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_check = "roles_or")]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_roles = #role_str)]
        #fn_vis #fn_asyncness fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            #fn_body
        }
    };
    
    expanded.into()
}
