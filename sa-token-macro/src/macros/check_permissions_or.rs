//! 多权限检查宏（OR逻辑）

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, Token};

/// 同时检查多个权限（OR逻辑）
/// 
/// 用户只需拥有任意一个指定的权限即可访问
/// 
/// # 示例
/// 
/// ```rust,ignore
/// #[sa_check_permissions_or("user:admin", "user:super")]
/// async fn manage_user() -> impl Responder {
///     // 拥有 user:admin 或 user:super 任一权限即可
///     "User managed"
/// }
/// ```
pub fn sa_check_permissions_or_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
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
    
    let expanded = quote! {
        #(#fn_attrs)*
        #[doc(hidden)]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_check = "permissions_or")]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_permissions = #perm_str)]
        #fn_vis #fn_asyncness fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            #fn_body
        }
    };
    
    TokenStream::from(expanded)
}

