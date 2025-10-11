//! Actix-web请求/响应适配器

use std::collections::HashMap;
use actix_web::{HttpRequest, HttpResponse};
use sa_token_adapter::context::{SaRequest, SaResponse, CookieOptions};
use serde::Serialize;

/// Actix-web请求适配器
pub struct ActixRequestAdapter<'a> {
    request: &'a HttpRequest,
}

impl<'a> ActixRequestAdapter<'a> {
    pub fn new(request: &'a HttpRequest) -> Self {
        Self { request }
    }
}

impl<'a> SaRequest for ActixRequestAdapter<'a> {
    fn get_header(&self, name: &str) -> Option<String> {
        self.request.headers().get(name)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
    }
    
    fn get_cookie(&self, name: &str) -> Option<String> {
        self.request.cookie(name)
            .map(|c| c.value().to_string())
    }
    
    fn get_param(&self, name: &str) -> Option<String> {
        self.request.match_info().get(name)
            .map(|s| s.to_string())
    }
    
    fn get_path(&self) -> String {
        self.request.path().to_string()
    }
    
    fn get_method(&self) -> String {
        self.request.method().to_string()
    }
    
    fn get_client_ip(&self) -> Option<String> {
        self.request.peer_addr()
            .map(|addr| addr.ip().to_string())
    }
}

/// Actix-web响应适配器
pub struct ActixResponseAdapter {
    builder: HttpResponse,
}

impl ActixResponseAdapter {
    pub fn new() -> Self {
        Self {
            builder: HttpResponse::Ok(),
        }
    }
    
    pub fn build(self) -> HttpResponse {
        self.builder.finish()
    }
}

impl Default for ActixResponseAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl SaResponse for ActixResponseAdapter {
    fn set_header(&mut self, name: &str, value: &str) {
        self.builder.append_header((name, value));
    }
    
    fn set_cookie(&mut self, name: &str, value: &str, options: CookieOptions) {
        use actix_web::cookie::Cookie;
        
        let mut cookie = Cookie::new(name, value);
        
        if let Some(domain) = options.domain {
            cookie.set_domain(domain);
        }
        if let Some(path) = options.path {
            cookie.set_path(path);
        }
        if let Some(max_age) = options.max_age {
            cookie.set_max_age(actix_web::cookie::time::Duration::seconds(max_age));
        }
        cookie.set_http_only(options.http_only);
        cookie.set_secure(options.secure);
        
        self.builder.cookie(cookie);
    }
    
    fn set_status(&mut self, status: u16) {
        if let Some(status_code) = actix_web::http::StatusCode::from_u16(status).ok() {
            self.builder.status(status_code);
        }
    }
    
    fn set_json_body<T: Serialize>(&mut self, body: T) -> Result<(), serde_json::Error> {
        self.builder.json(body);
        Ok(())
    }
}

