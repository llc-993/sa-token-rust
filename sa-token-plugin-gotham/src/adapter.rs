use gotham::hyper::{HeaderMap, Uri};
use sa_token_adapter::context::{SaRequest, SaResponse, CookieOptions};
use serde::Serialize;

/// 中文: Gotham 请求适配器，实现 SaRequest 接口
/// English: Gotham request adapter implementing SaRequest trait
pub struct GothamRequestAdapter<'a> {
    headers: &'a HeaderMap,
    uri: &'a Uri,
}

impl<'a> GothamRequestAdapter<'a> {
    /// 中文: 通过 HeaderMap 和 Uri 构造适配器
    /// English: Constructs adapter from HeaderMap and Uri
    pub fn new(headers: &'a HeaderMap, uri: &'a Uri) -> Self {
        Self { headers, uri }
    }
}

impl<'a> SaRequest for GothamRequestAdapter<'a> {
    /// 中文: 读取指定 Header
    /// English: Retrieves specified header
    fn get_header(&self, name: &str) -> Option<String> {
        self.headers.get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }

    /// 中文: 解析原始 Cookie 字符串
    /// English: Parses raw cookie string
    fn get_cookie(&self, name: &str) -> Option<String> {
        self.headers.get("cookie")
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

    /// 中文: 查找查询参数
    /// English: Looks up query parameter
    fn get_param(&self, name: &str) -> Option<String> {
        self.uri.query()
            .and_then(|query| {
                query.split('&')
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

    /// 中文: 返回路径
    /// English: Returns path
    fn get_path(&self) -> String {
        self.uri.path().to_string()
    }

    /// 中文: Gotham 在此阶段无法直接获得 Method
    /// English: Method not directly available in this context
    fn get_method(&self) -> String {
        "GET".to_string()
    }

    /// 中文: Gotham 状态中默认无法获取客户端 IP
    /// English: Client IP not available by default in Gotham state
    fn get_client_ip(&self) -> Option<String> {
        None
    }
}

/// 中文: Gotham 响应适配器，实现 SaResponse 接口
/// English: Gotham response adapter implementing SaResponse trait
pub struct GothamResponseAdapter {
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl GothamResponseAdapter {
    /// 中文: 创建空响应适配器
    /// English: Creates an empty response adapter
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            body: None,
        }
    }
}

impl Default for GothamResponseAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SaResponse for GothamResponseAdapter {
    /// 中文: 设置返回头
    /// English: Sets response header
    fn set_header(&mut self, name: &str, value: &str) {
        self.headers.push((name.to_string(), value.to_string()));
    }

    /// 中文: 追加 Set-Cookie
    /// English: Appends Set-Cookie header
    fn set_cookie(&mut self, name: &str, value: &str, _options: CookieOptions) {
        self.headers.push(("Set-Cookie".to_string(), format!("{}={}", name, value)));
    }

    /// 中文: Gotham 响应构建时再处理状态码
    /// English: Status code handled when building Gotham response
    fn set_status(&mut self, _status: u16) {}

    /// 中文: 序列化 JSON 并保存到 body
    /// English: Serializes JSON payload and stores it
    fn set_json_body<T: Serialize>(&mut self, body: T) -> Result<(), serde_json::Error> {
        let json = serde_json::to_string(&body)?;
        self.body = Some(json);
        self.headers.push(("Content-Type".to_string(), "application/json".to_string()));
        Ok(())
    }
}

