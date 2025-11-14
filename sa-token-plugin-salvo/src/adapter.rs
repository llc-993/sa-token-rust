// Author: 金书记
//
// 中文 | English
// Salvo 请求/响应适配器 | Salvo request/response adapter

use salvo::prelude::*;
use sa_token_adapter::{SaRequest, SaResponse, CookieOptions, build_cookie_string};
use serde::Serialize;

/// 中文 | English
/// Salvo 请求适配器 | Salvo request adapter
pub struct SalvoRequestAdapter<'a> {
    request: &'a Request,
}

impl<'a> SalvoRequestAdapter<'a> {
    /// 中文 | English
    /// 创建新的请求适配器 | Create a new request adapter
    pub fn new(request: &'a Request) -> Self {
        Self { request }
    }
}

impl<'a> SaRequest for SalvoRequestAdapter<'a> {
    fn get_header(&self, name: &str) -> Option<String> {
        self.request
            .headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }
    
    fn get_cookie(&self, name: &str) -> Option<String> {
        // 先尝试使用 cookie 方法 | First try using cookie method
        if let Some(cookie) = self.request.cookie(name) {
            return Some(cookie.value().to_string());
        }
        
        // 如果没有找到，手动解析 Cookie 头 | If not found, manually parse Cookie header
        if let Some(cookie_header) = self.request.headers().get("cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                let cookies = sa_token_adapter::utils::parse_cookies(cookie_str);
                if let Some(value) = cookies.get(name) {
                    return Some(value.to_string());
                }
            }
        }
        
        None
    }
    
    fn get_param(&self, name: &str) -> Option<String> {
        // 先尝试使用 query 方法 | First try using query method
        if let Some(value) = self.request.query::<String>(name) {
            return Some(value);
        }
        
        // 如果没有找到，手动解析查询字符串 | If not found, manually parse query string
        if let Some(query) = self.request.uri().query() {
            let params = sa_token_adapter::utils::parse_query_string(query);
            if let Some(value) = params.get(name) {
                return Some(value.to_string());
            }
        }
        
        None
    }
    
    fn get_path(&self) -> String {
        self.request.uri().path().to_string()
    }
    
    fn get_method(&self) -> String {
        self.request.method().to_string()
    }
}

/// 中文 | English
/// Salvo 响应适配器 | Salvo response adapter
pub struct SalvoResponseAdapter<'a> {
    response: &'a mut Response,
}

impl<'a> SalvoResponseAdapter<'a> {
    /// 中文 | English
    /// 创建新的响应适配器 | Create a new response adapter
    pub fn new(response: &'a mut Response) -> Self {
        Self { response }
    }
}

impl<'a> SaResponse for SalvoResponseAdapter<'a> {
    fn set_header(&mut self, name: &str, value: &str) {
        if let Ok(header_name) = http::header::HeaderName::from_bytes(name.as_bytes()) {
            if let Ok(header_value) = http::header::HeaderValue::from_str(value) {
                self.response.headers_mut().insert(header_name, header_value);
            }
        }
    }
    
    fn set_cookie(&mut self, name: &str, value: &str, options: CookieOptions) {
        let cookie_string = build_cookie_string(name, value, options);
        self.set_header("Set-Cookie", &cookie_string);
    }
    
    fn set_status(&mut self, status: u16) {
        if let Ok(status_code) = http::StatusCode::from_u16(status) {
            self.response.status_code(status_code);
        }
    }
    
    fn set_json_body<U: Serialize>(&mut self, body: U) -> Result<(), serde_json::Error> {
        match serde_json::to_string(&body) {
            Ok(json) => {
                self.response.render(Text::Json(json));
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

