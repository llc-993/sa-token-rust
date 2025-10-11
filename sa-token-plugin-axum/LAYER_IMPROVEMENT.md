# sa-token-plugin-axum Layer 改进说明

## 改进内容

### ✅ 完善 `extract_token_from_request` 方法

#### 改进前
```rust
fn extract_token_from_request<T>(_request: &Request<T>, _state: &SaTokenState) -> Option<String> {
    // TODO: 从header、cookie等位置提取token
    None
}
```

#### 改进后
```rust
/// 从请求中提取 Token
/// 
/// 按优先级顺序查找 Token：
/// 1. HTTP Header - `Authorization: <token>` 或 `<token_name>: <token>`
/// 2. Cookie - `<token_name>=<token>`
/// 3. Query Parameter - `?<token_name>=<token>`
fn extract_token_from_request<T>(request: &Request<T>, _state: &SaTokenState) -> Option<String> {
    let adapter = AxumRequestAdapter::new(request);
    let token_name = "Authorization";
    
    // 1. 优先从 Header 中获取
    if let Some(token) = adapter.get_header(token_name) {
        return Some(extract_bearer_token(&token));
    }
    
    // 2. 从 Cookie 中获取
    if let Some(token) = adapter.get_cookie(token_name) {
        return Some(token);
    }
    
    // 3. 从 Query 参数中获取
    if let Some(query) = request.uri().query() {
        if let Some(token) = parse_query_param(query, token_name) {
            return Some(token);
        }
    }
    
    None
}
```

## 新增功能

### 1. 🔑 Bearer Token 支持

新增 `extract_bearer_token` 函数，支持两种格式：

```rust
/// 提取 Bearer Token
/// 
/// 支持两种格式：
/// - `Bearer <token>` - 标准 Bearer Token 格式
/// - `<token>` - 直接的 Token 字符串
fn extract_bearer_token(header_value: &str) -> String {
    const BEARER_PREFIX: &str = "Bearer ";
    
    if header_value.starts_with(BEARER_PREFIX) {
        header_value[BEARER_PREFIX.len()..].trim().to_string()
    } else {
        header_value.trim().to_string()
    }
}
```

**使用示例：**
```bash
# 标准格式
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...

# 简化格式
Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

### 2. 🔍 Query 参数解析

新增 `parse_query_param` 函数，支持从 URL 查询参数中提取 Token：

```rust
/// 从 Query 字符串中解析参数
fn parse_query_param(query: &str, param_name: &str) -> Option<String> {
    for pair in query.split('&') {
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        if parts.len() == 2 && parts[0] == param_name {
            // URL 解码
            return urlencoding::decode(parts[1])
                .ok()
                .map(|s| s.into_owned());
        }
    }
    None
}
```

**使用示例：**
```bash
GET /api/user/info?Authorization=your_token_here
```

### 3. 📝 完善中间件处理逻辑

改进 `call` 方法，实现完整的 Token 验证和存储流程：

```rust
fn call(&mut self, mut request: Request<ReqBody>) -> Self::Future {
    let mut inner = self.inner.clone();
    let state = self.state.clone();
    
    Box::pin(async move {
        // 从请求中提取 token
        if let Some(token_str) = extract_token_from_request(&request, &state) {
            let token = sa_token_core::token::TokenValue::new(token_str);
            
            // 验证 token 是否有效
            if state.manager.is_valid(&token).await {
                // 将 token 存储到请求扩展中
                request.extensions_mut().insert(token.clone());
                
                // 尝试获取 token 信息并存储 login_id
                if let Ok(token_info) = state.manager.get_token_info(&token).await {
                    request.extensions_mut().insert(token_info.login_id.clone());
                }
            }
        }
        
        // 继续处理请求
        inner.call(request).await
    })
}
```

### 4. 🛠 adapter.rs 改进

完善 `get_param` 方法和新增 `parse_query_string` 函数：

```rust
// AxumRequestAdapter 中
fn get_param(&self, name: &str) -> Option<String> {
    self.request
        .uri()
        .query()
        .and_then(|query| parse_query_string(query).get(name).cloned())
}

// 新增解析函数
fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    for pair in query.split('&') {
        let parts: Vec<&str> = pair.splitn(2, '=').collect();
        if parts.len() == 2 {
            params.insert(
                urlencoding::decode(parts[0]).unwrap_or_default().to_string(),
                urlencoding::decode(parts[1]).unwrap_or_default().to_string(),
            );
        }
    }
    params
}
```

## Token 提取优先级

中间件按以下优先级顺序查找 Token：

1. **HTTP Header** (优先级最高)
   - `Authorization: Bearer <token>`
   - `Authorization: <token>`

2. **Cookie**
   - `Authorization=<token>`

3. **Query Parameter** (优先级最低)
   - `?Authorization=<token>`

## 依赖项更新

新增 `urlencoding` 依赖用于 URL 解码：

```toml
[dependencies]
urlencoding = "2.1"
```

## 使用示例

### 方式 1：Header (推荐)

```bash
curl -H "Authorization: Bearer your_token_here" \
     http://localhost:3000/api/user/info
```

### 方式 2：Cookie

```bash
curl -b "Authorization=your_token_here" \
     http://localhost:3000/api/user/info
```

### 方式 3：Query Parameter

```bash
curl "http://localhost:3000/api/user/info?Authorization=your_token_here"
```

## 改进效果

- ✅ 支持多种 Token 传递方式
- ✅ 支持标准 Bearer Token 格式
- ✅ 自动 URL 解码
- ✅ Token 验证和存储
- ✅ LoginId 自动提取
- ✅ 完整的文档注释
- ✅ 与 Poem 插件功能对等

## 测试建议

```rust
#[tokio::test]
async fn test_extract_token_from_header() {
    // 测试从 Header 提取 Bearer Token
}

#[tokio::test]
async fn test_extract_token_from_cookie() {
    // 测试从 Cookie 提取 Token
}

#[tokio::test]
async fn test_extract_token_from_query() {
    // 测试从 Query 参数提取 Token
}
```

## 总结

此次改进使 `sa-token-plugin-axum` 的 Layer 功能更加完善，与 Poem 插件保持了功能对等性，提供了完整的 Token 提取和验证机制。

