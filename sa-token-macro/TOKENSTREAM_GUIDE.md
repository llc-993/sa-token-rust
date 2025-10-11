# TokenStream 使用指南

## 统一的命名策略

在重构后的代码中，我们采用了清晰的类型别名策略来区分两种 `TokenStream`：

## 两种 TokenStream

### 1. `proc_macro::TokenStream`
- **用途**：过程宏的公开接口（函数签名）
- **命名**：直接使用 `TokenStream`
- **来源**：编译器提供

### 2. `proc_macro2::TokenStream`
- **用途**：内部实现，配合 `quote!` 宏
- **命名**：使用类型别名 `TokenStream2`
- **来源**：`proc-macro2` crate

## 统一的模式

所有宏文件现在都遵循这个清晰的模式：

```rust
//! 宏实现文件

// 导入部分
use proc_macro::TokenStream;           // 用于函数签名
use proc_macro2::TokenStream as TokenStream2;  // 用于内部实现
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// 宏文档
pub fn some_macro_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    
    // 使用 TokenStream2 来明确标注类型
    let expanded: TokenStream2 = quote! {
        // 生成的代码
    };
    
    // 通过 .into() 转换为 proc_macro::TokenStream
    expanded.into()
}
```

## 优势

### ✅ 1. 类型清晰
```rust
// ❌ 之前：类型不明确
let expanded = quote! { ... };
TokenStream::from(expanded)

// ✅ 现在：类型明确
let expanded: TokenStream2 = quote! { ... };
expanded.into()
```

### ✅ 2. 易于理解
看到 `TokenStream2` 就知道这是 `proc_macro2` 的类型，与 `quote!` 配合使用。

### ✅ 3. 减少困惑
新手不会疑惑"为什么有两个 TokenStream？"，因为命名清楚地区分了它们。

### ✅ 4. IDE 友好
类型注解帮助 IDE 更好地进行类型推断和错误检查。

## 实例对比

### check_login.rs（重构后）

```rust
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn sa_check_login_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    // ... 其他提取
    
    // 明确标注类型
    let expanded: TokenStream2 = quote! {
        #[sa_token_check = "login"]
        fn #fn_name() { ... }
    };
    
    // 清晰的转换
    expanded.into()
}
```

## 为什么需要两个 TokenStream？

### proc_macro::TokenStream
- 编译器 API 的一部分
- 只能在过程宏 crate 中使用
- 不能在单元测试中使用
- 功能有限

### proc_macro2::TokenStream
- 独立的 crate
- 可以在任何地方使用
- 支持单元测试
- 功能丰富
- 与 `quote!` 和 `syn` 完美集成

## 类型转换

两种 `TokenStream` 之间可以自动转换：

```rust
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

// proc_macro2 -> proc_macro
let ts1: TokenStream = ts2.into();

// proc_macro -> proc_macro2
let ts2: TokenStream2 = ts1.into();
```

## 所有文件的统一结构

```
sa-token-macro/src/macros/
├── check_login.rs          ✅ 使用统一模式
├── check_permission.rs     ✅ 使用统一模式
├── check_role.rs           ✅ 使用统一模式
├── check_permissions_and.rs ✅ 使用统一模式
├── check_permissions_or.rs  ✅ 使用统一模式
├── check_roles_and.rs      ✅ 使用统一模式
├── check_roles_or.rs       ✅ 使用统一模式
└── ignore.rs               ✅ 使用统一模式
```

## utils.rs 的特殊情况

`utils.rs` 中只使用 `proc_macro2::TokenStream`，因为：
1. 它不是宏的直接入口
2. 它的函数返回值被内部使用
3. 不需要编译器的 `proc_macro::TokenStream`

```rust
// utils.rs
use proc_macro2::TokenStream;  // 只需要这个
use quote::quote;

pub fn generate_code() -> TokenStream {
    quote! { /* code */ }
}
```

## 最佳实践

1. **函数签名**：使用 `proc_macro::TokenStream`
2. **内部实现**：使用 `proc_macro2::TokenStream as TokenStream2`
3. **明确类型**：为 `quote!` 的结果添加类型注解
4. **转换方式**：使用 `.into()` 而不是 `TokenStream::from()`

## 总结

通过统一的命名策略：
- ✅ 代码结构清晰
- ✅ 类型一目了然
- ✅ 减少新手困惑
- ✅ IDE 支持更好
- ✅ 易于维护和扩展

这是 Rust 过程宏开发的最佳实践！

