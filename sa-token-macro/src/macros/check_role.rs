//! 角色检查宏

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// 检查角色的宏
/// 
/// 使用此宏标注的函数会在执行前检查用户是否拥有指定角色
/// 
/// # 参数
/// 
/// - `role` - 角色名称，如 "admin"、"user"、"vip"
/// 
/// # 示例
/// 
/// ```rust,ignore
/// #[sa_check_role("admin")]
/// async fn admin_panel() -> impl Responder {
///     // 只有 admin 角色才能访问
///     "Admin panel"
/// }
/// ```
pub fn sa_check_role_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let role = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_body = &input.block;
    let fn_attrs = &input.attrs;
    let fn_vis = &input.vis;
    let fn_asyncness = &input.sig.asyncness;
    let fn_generics = &input.sig.generics;
    let role_value = role.value();
    
    let expanded = quote! {
        #(#fn_attrs)*
        #[doc(hidden)]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_check = "role")]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_role = #role_value)]
        #fn_vis #fn_asyncness fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            // 角色检查逻辑在中间件中执行
            // 所需角色: #role_value
            
            #fn_body
        }
    };
    
    TokenStream::from(expanded)
}

