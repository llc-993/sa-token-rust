//! 权限检查宏

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// 检查权限的宏
/// 
/// 使用此宏标注的函数会在执行前检查用户是否拥有指定权限
/// 
/// # 参数
/// 
/// - `permission` - 权限标识符，如 "user:delete"、"admin:*"
/// 
/// # 示例
/// 
/// ```rust,ignore
/// #[sa_check_permission("user:delete")]
/// async fn delete_user(id: u64) -> impl Responder {
///     // 只有拥有 user:delete 权限的用户才能访问
///     "User deleted"
/// }
/// ```
pub fn sa_check_permission_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let permission = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_body = &input.block;
    let fn_attrs = &input.attrs;
    let fn_vis = &input.vis;
    let fn_asyncness = &input.sig.asyncness;
    let fn_generics = &input.sig.generics;
    let perm_value = permission.value();
    
    let expanded = quote! {
        #(#fn_attrs)*
        #[doc(hidden)]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_check = "permission")]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_permission = #perm_value)]
        #fn_vis #fn_asyncness fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            // 权限检查逻辑在中间件中执行
            // 所需权限: #perm_value
            
            #fn_body
        }
    };
    
    TokenStream::from(expanded)
}

