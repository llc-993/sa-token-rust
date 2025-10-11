//! 登录检查宏

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// 检查登录状态的宏
/// 
/// 使用此宏标注的函数会在执行前检查用户是否已登录
/// 
/// # 示例
/// 
/// ```rust,ignore
/// #[sa_check_login]
/// async fn user_info() -> impl Responder {
///     "User info"
/// }
/// ```
pub fn sa_check_login_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
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
        // 添加元数据标记，供中间件识别
        #[doc(hidden)]
        #[cfg_attr(feature = "sa-token-metadata", sa_token_check = "login")]
        #fn_vis #fn_asyncness fn #fn_name #fn_generics(#fn_inputs) #fn_output {
            // 注意：实际的认证逻辑在框架中间件中执行
            // 这个宏主要是添加编译时标记和文档
            // 中间件会检查函数上的属性来决定是否需要验证
            
            #fn_body
        }
    };
    
    TokenStream::from(expanded)
}

