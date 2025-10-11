// Author: 金书记
//
//! 多权限检查宏（AND逻辑）

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, Token, parse::Parser};

/// 同时检查多个权限（AND逻辑）
/// 
/// 用户必须拥有所有指定的权限才能访问
/// 
/// # 示例
/// 
/// ```rust,ignore
/// #[sa_check_permissions_and("user:read", "user:write")]
/// async fn update_user() -> impl Responder {
///     // 需要同时拥有 user:read 和 user:write 权限
///     "User updated"
/// }
/// ```
pub fn sa_check_permissions_and_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    // 解析多个权限参数
    let parser = syn::punctuated::Punctuated::<LitStr, Token![,]>::parse_terminated;
    let permissions = parser.parse(attr).unwrap_or_default();
    let perm_values: Vec<String> = permissions.iter().map(|p| p.value()).collect();
    let perm_str = perm_values.join(",");
    
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
        #[cfg_attr(feature = "sa-token-metadata", sa_token_check = "permissions_and")]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_permissions = #perm_str)]
        #fn_vis #fn_asyncness fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            #fn_body
        }
    };
    
    expanded.into()
}
