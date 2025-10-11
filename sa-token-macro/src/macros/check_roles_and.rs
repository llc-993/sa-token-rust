//! 多角色检查宏（AND逻辑）

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, Token};

/// 同时检查多个角色（AND逻辑）
/// 
/// 用户必须拥有所有指定的角色才能访问
/// 
/// # 示例
/// 
/// ```rust,ignore
/// #[sa_check_roles_and("admin", "super")]
/// async fn super_admin_panel() -> impl Responder {
///     // 需要同时拥有 admin 和 super 角色
///     "Super admin panel"
/// }
/// ```
pub fn sa_check_roles_and_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
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
    
    let expanded = quote! {
        #(#fn_attrs)*
        #[doc(hidden)]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_check = "roles_and")]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_roles = #role_str)]
        #fn_vis #fn_asyncness fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            #fn_body
        }
    };
    
    TokenStream::from(expanded)
}

