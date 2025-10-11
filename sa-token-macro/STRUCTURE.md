# sa-token-macro 文件结构说明

## 重构后的目录结构

```
sa-token-macro/
├── src/
│   ├── lib.rs                          # 宏库入口，导出所有宏
│   ├── utils.rs                        # 工具函数
│   └── macros/                         # 宏实现目录
│       ├── mod.rs                      # 模块声明
│       ├── check_login.rs              # 登录检查宏
│       ├── check_permission.rs         # 权限检查宏
│       ├── check_role.rs               # 角色检查宏
│       ├── check_permissions_and.rs    # 多权限AND检查宏
│       ├── check_permissions_or.rs     # 多权限OR检查宏
│       ├── check_roles_and.rs          # 多角色AND检查宏
│       ├── check_roles_or.rs           # 多角色OR检查宏
│       └── ignore.rs                   # 忽略认证宏
├── examples/
│   └── basic_usage.rs                  # 基础使用示例
├── Cargo.toml
├── README.md
├── MACRO_COMPLETE.md
└── STRUCTURE.md                        # 本文件
```

## 每个文件的职责

### lib.rs
- 宏库的入口文件
- 导出所有的过程宏
- 负责将宏实现函数封装为 `#[proc_macro_attribute]`

### macros/mod.rs
- 声明所有宏实现模块
- 组织模块结构

### macros/check_login.rs
- 实现 `#[sa_check_login]` 宏
- 为函数添加登录检查的元数据标记

### macros/check_permission.rs
- 实现 `#[sa_check_permission]` 宏
- 接收权限标识符参数
- 为函数添加权限检查的元数据标记

### macros/check_role.rs
- 实现 `#[sa_check_role]` 宏
- 接收角色名称参数
- 为函数添加角色检查的元数据标记

### macros/check_permissions_and.rs
- 实现 `#[sa_check_permissions_and]` 宏
- 接收多个权限参数（逗号分隔）
- 生成AND逻辑的权限检查标记

### macros/check_permissions_or.rs
- 实现 `#[sa_check_permissions_or]` 宏
- 接收多个权限参数（逗号分隔）
- 生成OR逻辑的权限检查标记

### macros/check_roles_and.rs
- 实现 `#[sa_check_roles_and]` 宏
- 接收多个角色参数（逗号分隔）
- 生成AND逻辑的角色检查标记

### macros/check_roles_or.rs
- 实现 `#[sa_check_roles_or]` 宏
- 接收多个角色参数（逗号分隔）
- 生成OR逻辑的角色检查标记

### macros/ignore.rs
- 实现 `#[sa_ignore]` 宏
- 可应用于函数、结构体、impl块
- 标记为忽略所有认证检查

### utils.rs
- 提供宏实现的工具函数
- 包含通用的代码生成逻辑

## 重构的优势

### 1. 清晰的文件组织
每个宏单独一个文件，职责明确，易于维护和扩展。

### 2. 易于导航
开发者可以快速定位到特定宏的实现，不需要在一个大文件中搜索。

### 3. 模块化设计
每个宏都是独立的模块，可以单独测试和修改。

### 4. 便于扩展
添加新的宏时，只需：
1. 在 `macros/` 目录下创建新文件
2. 在 `macros/mod.rs` 中声明模块
3. 在 `lib.rs` 中导出宏

### 5. 代码复用
通用的逻辑可以提取到 `utils.rs` 中，避免重复代码。

## 添加新宏的步骤

假设要添加一个新的 `#[sa_check_safe]` 宏：

1. 创建文件 `src/macros/check_safe.rs`：
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn sa_check_safe_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    // 实现宏逻辑...
    TokenStream::from(quote! { #input })
}
```

2. 在 `src/macros/mod.rs` 中添加声明：
```rust
pub mod check_safe;
```

3. 在 `src/lib.rs` 中导入并导出：
```rust
use macros::check_safe::sa_check_safe_impl;

#[proc_macro_attribute]
pub fn sa_check_safe(attr: TokenStream, item: TokenStream) -> TokenStream {
    sa_check_safe_impl(attr, item)
}
```

就这么简单！

## 与旧结构的对比

### 旧结构（单文件）
```
sa-token-macro/
└── src/
    └── lib.rs  (所有宏都在一个文件中，~500行)
```

**缺点**：
- 单个文件过大，不易维护
- 代码混在一起，难以导航
- 修改一个宏可能影响其他宏

### 新结构（模块化）
```
sa-token-macro/
└── src/
    ├── lib.rs           (入口，~80行)
    ├── utils.rs         (工具函数)
    └── macros/          (宏实现目录)
        ├── mod.rs       (模块声明)
        ├── check_login.rs         (~40行)
        ├── check_permission.rs    (~50行)
        ├── check_role.rs          (~50行)
        ├── check_permissions_and.rs  (~50行)
        ├── check_permissions_or.rs   (~50行)
        ├── check_roles_and.rs        (~50行)
        ├── check_roles_or.rs         (~50行)
        └── ignore.rs                 (~80行)
```

**优点**：
- 每个文件职责单一，易于理解
- 模块化设计，便于测试
- 易于扩展和维护
- 团队协作时减少冲突

## 总结

重构后的结构更加清晰、模块化，符合 Rust 的最佳实践。每个宏都有自己的文件，便于维护和扩展。

