// Author: 金书记
//
//! Rocket 请求/响应适配器

use rocket::{Request, Response};
use rocket::http::{Header, Cookie, Status, ContentType};
use sa_token_adapter::context::{SaRequest, SaResponse, CookieOptions};
use serde::Serialize;
use std::collections::HashMap;

/// Rocket 请求适配器
pub struct RocketRequestAdapter<'a> {
    request: &'a Request<'a>,
}

impl<'a> RocketRequestAdapter<'a> {
    pub fn new(request: &'a Request<'a>) -> Self {
        Self { request }
    }
}

impl<'a> SaRequest for RocketRequestAdapter<'a> {
    fn get_header(&self, name: &str) -> Option<String> {
        self.request.headers().get_one(name)
            .map(|s| s.to_string())
    }
    
    fn get_cookie(&self, name: &str) -> Option<String> {
        self.request.cookies().get(name)
            .map(|c| c.value().to_string())
    }
    
    fn get_param(&self, name: &str) -> Option<String> {
        // Rocket 的查询参数需要从 URI 中提取
        if let Some(query) = self.request.uri().query() {
            return parse_query_string(query.as_str())
                .get(name)
                .cloned();
        }
        None
    }
    
    fn get_path(&self) -> String {
        self.request.uri().path().to_string()
    }
    
    fn get_method(&self) -> String {
        self.request.method().to_string()
    }
    
    fn get_client_ip(&self) -> Option<String> {
        self.request.client_ip()
            .map(|ip| ip.to_string())
    }
}

/// Rocket 响应适配器
pub struct RocketResponseAdapter<'a> {
    response: &'a mut Response<'a>,
}

impl<'a> RocketResponseAdapter<'a> {
    pub fn new(response: &'a mut Response<'a>) -> Self {
        Self { response }
    }
}

impl<'a> SaResponse for RocketResponseAdapter<'a> {
    fn set_header(&mut self, name: &str, value: &str) {
        self.response.set_header(Header::new(name.to_string(), value.to_string()));
    }
    
    fn set_cookie(&mut self, name: &str, value: &str, options: CookieOptions) {
        let mut cookie = Cookie::new(name.to_string(), value.to_string());
        
        if let Some(domain) = options.domain {
            cookie.set_domain(domain);
        }
        if let Some(path) = options.path {
            cookie.set_path(path);
        }
        if let Some(max_age) = options.max_age {
            cookie.set_max_age(rocket::time::Duration::seconds(max_age));
        }
        cookie.set_http_only(options.http_only);
        cookie.set_secure(options.secure);
        
        if let Some(same_site) = options.same_site {
            use sa_token_adapter::context::SameSite as SaSameSite;
            use rocket::http::SameSite;
            
            let ss = match same_site {
                SaSameSite::Strict => SameSite::Strict,
                SaSameSite::Lax => SameSite::Lax,
                SaSameSite::None => SameSite::None,
            };
            cookie.set_same_site(ss);
        }
        
        self.response.adjoin_header(cookie);
    }
    
    fn set_status(&mut self, status: u16) {
        if let Some(status_code) = Status::from_code(status) {
            self.response.set_status(status_code);
        }
    }
    
    fn set_json_body<T: Serialize>(&mut self, body: T) -> Result<(), serde_json::Error> {
        let json = serde_json::to_string(&body)?;
        self.response.set_header(ContentType::JSON);
        self.response.set_sized_body(Some(json.len()), std::io::Cursor::new(json));
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
