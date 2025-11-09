use ntex::web::HttpRequest;
use sa_token_adapter::context::{SaRequest, SaResponse, CookieOptions};
use serde::Serialize;

/// 中文: 将 Ntex HttpRequest 封装为 SaRequest 适配器
/// English: Adapter wrapping Ntex HttpRequest to implement SaRequest
pub struct NtexRequestAdapter<'a> {
    request: &'a HttpRequest,
}

impl<'a> NtexRequestAdapter<'a> {
    /// 中文: 创建适配器实例
    /// English: Creates a new adapter instance
    pub fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> SaRequest for NtexRequestAdapter<'a> {
    /// 中文: 读取指定 Header
    /// English: Retrieves specified header value
    fn get_header(&self, name: &str) -> Option<String> {
        self.request.headers().get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }

    /// 中文: 解析 Cookie 并返回指定名称的值
    /// English: Parses cookies and returns the value by name
    fn get_cookie(&self, name: &str) -> Option<String> {
        self.request.headers().get("cookie")
            .and_then(|v| v.to_str().ok())
            .and_then(|cookies| {
                cookies.split(';')
                    .find_map(|cookie| {
                        let mut parts = cookie.trim().splitn(2, '=');
                        match (parts.next(), parts.next()) {
                            (Some(k), Some(v)) if k == name => Some(v.to_string()),
                            _ => None,
                        }
                    })
            })
    }

    /// 中文: 支持路径参数和查询参数读取
    /// English: Supports reading both path and query parameters
    fn get_param(&self, name: &str) -> Option<String> {
        self.request.match_info().get(name)
            .map(|s| s.to_string())
            .or_else(|| {
                self.request.query_string()
                    .split('&')
                    .find_map(|pair| {
                        let mut parts = pair.splitn(2, '=');
                        match (parts.next(), parts.next()) {
                            (Some(k), Some(v)) if k == name => 
                                urlencoding::decode(v).ok().map(|s| s.to_string()),
                            _ => None,
                        }
                    })
            })
    }

    /// 中文: 返回请求路径
    /// English: Returns request path
    fn get_path(&self) -> String {
        self.request.path().to_string()
    }

    /// 中文: 返回请求方法
    /// English: Returns request method
    fn get_method(&self) -> String {
        self.request.method().to_string()
    }

    /// 中文: 提取客户端 IP
    /// English: Extracts client IP address
    fn get_client_ip(&self) -> Option<String> {
        self.request.peer_addr()
            .map(|addr| addr.ip().to_string())
    }
}

/// 中文: 响应适配器，用于设置响应头和 JSON 内容
/// English: Response adapter for setting headers and JSON body
pub struct NtexResponseAdapter {
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl NtexResponseAdapter {
    /// 中文: 初始化空响应适配器
    /// English: Initializes an empty response adapter
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            body: None,
        }
    }
}

impl Default for NtexResponseAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SaResponse for NtexResponseAdapter {
    /// 中文: 设置响应头
    /// English: Sets response header
    fn set_header(&mut self, name: &str, value: &str) {
        self.headers.push((name.to_string(), value.to_string()));
    }

    /// 中文: 写入 Cookie（简单实现，可按需扩展）
    /// English: Stores cookie header (simple implementation, extend as needed)
    fn set_cookie(&mut self, name: &str, value: &str, _options: CookieOptions) {
        self.headers.push(("Set-Cookie".to_string(), format!("{}={}", name, value)));
    }

    /// 中文: 状态码在 Ntex 响应构建阶段处理
    /// English: Status code handled during Ntex response building
    fn set_status(&mut self, _status: u16) {}

    /// 中文: 设置 JSON 响应
    /// English: Sets JSON response body
    fn set_json_body<T: Serialize>(&mut self, body: T) -> Result<(), serde_json::Error> {
        let json = serde_json::to_string(&body)?;
        self.body = Some(json);
        self.headers.push(("Content-Type".to_string(), "application/json".to_string()));
        Ok(())
    }
}

