//! Warp 请求/响应适配器

use warp::http::{HeaderMap, HeaderValue, Response, StatusCode};
use warp::hyper::body::Bytes;
use sa_token_adapter::context::{SaRequest, SaResponse, CookieOptions};
use serde::Serialize;
use std::collections::HashMap;

/// Warp 请求适配器
pub struct WarpRequestAdapter<'a> {
    headers: &'a HeaderMap,
    query: &'a str,
}

impl<'a> WarpRequestAdapter<'a> {
    pub fn new(headers: &'a HeaderMap, query: &'a str) -> Self {
        Self { headers, query }
    }
}

impl<'a> SaRequest for WarpRequestAdapter<'a> {
    fn get_header(&self, name: &str) -> Option<String> {
        self.headers.get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }
    
    fn get_cookie(&self, name: &str) -> Option<String> {
        // Warp 中 Cookie 通常从 Header 中解析
        if let Some(cookie_header) = self.headers.get("cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                return parse_cookie(cookie_str, name);
            }
        }
        None
    }
    
    fn get_param(&self, name: &str) -> Option<String> {
        parse_query_string(self.query)
            .get(name)
            .cloned()
    }
    
    fn get_path(&self) -> String {
        // Warp 中需要从外部传入
        String::new()
    }
    
    fn get_method(&self) -> String {
        // Warp 中需要从外部传入
        String::new()
    }
    
    fn get_client_ip(&self) -> Option<String> {
        // Warp 中需要从外部传入或从 Header 获取
        self.get_header("x-forwarded-for")
            .or_else(|| self.get_header("x-real-ip"))
    }
}

/// Warp 响应适配器
pub struct WarpResponseAdapter {
    status: StatusCode,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl WarpResponseAdapter {
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: Vec::new(),
            body: None,
        }
    }
    
    /// 构建 Warp Response
    pub fn build(self) -> Response<Bytes> {
        let mut builder = Response::builder().status(self.status);
        
        for (name, value) in self.headers {
            builder = builder.header(name, value);
        }
        
        let body = self.body.unwrap_or_default();
        builder.body(Bytes::from(body)).unwrap()
    }
}

impl Default for WarpResponseAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SaResponse for WarpResponseAdapter {
    fn set_header(&mut self, name: &str, value: &str) {
        self.headers.push((name.to_string(), value.to_string()));
    }
    
    fn set_cookie(&mut self, name: &str, value: &str, options: CookieOptions) {
        let mut cookie = format!("{}={}", name, value);
        
        if let Some(domain) = options.domain {
            cookie.push_str(&format!("; Domain={}", domain));
        }
        if let Some(path) = options.path {
            cookie.push_str(&format!("; Path={}", path));
        }
        if let Some(max_age) = options.max_age {
            cookie.push_str(&format!("; Max-Age={}", max_age));
        }
        if options.http_only {
            cookie.push_str("; HttpOnly");
        }
        if options.secure {
            cookie.push_str("; Secure");
        }
        if let Some(same_site) = options.same_site {
            use sa_token_adapter::context::SameSite;
            let ss = match same_site {
                SameSite::Strict => "Strict",
                SameSite::Lax => "Lax",
                SameSite::None => "None",
            };
            cookie.push_str(&format!("; SameSite={}", ss));
        }
        
        self.headers.push(("Set-Cookie".to_string(), cookie));
    }
    
    fn set_status(&mut self, status: u16) {
        if let Ok(status_code) = StatusCode::from_u16(status) {
            self.status = status_code;
        }
    }
    
    fn set_json_body<T: Serialize>(&mut self, body: T) -> Result<(), serde_json::Error> {
        let json = serde_json::to_string(&body)?;
        self.body = Some(json);
        self.headers.push(("Content-Type".to_string(), "application/json".to_string()));
        Ok(())
    }
}

/// 解析查询字符串
fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if let Ok(decoded_value) = urlencoding::decode(value) {
                params.insert(key.to_string(), decoded_value.to_string());
            }
        }
    }
    params
}

/// 解析 Cookie
fn parse_cookie(cookie_str: &str, name: &str) -> Option<String> {
    for pair in cookie_str.split(';') {
        let pair = pair.trim();
        if let Some((key, value)) = pair.split_once('=') {
            if key == name {
                return Some(value.to_string());
            }
        }
    }
    None
}

