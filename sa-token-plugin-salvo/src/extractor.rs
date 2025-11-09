use salvo::prelude::*;
use sa_token_core::token::TokenValue;

/// 中文: 必填 Token 提取器，从 Salvo Request 扩展里读取 Token
/// English: Required token extractor reading TokenValue from Salvo request extensions
pub struct SaTokenExtractor(pub Option<TokenValue>);

impl SaTokenExtractor {
    /// 中文: 中间件注入 Token 后，从 extensions 中取出
    /// English: Fetches TokenValue injected by middleware in extensions
    pub fn from_request(req: &Request) -> Self {
        let token = req.extensions()
            .get::<TokenValue>()
            .cloned();
        SaTokenExtractor(token)
    }
}

/// 中文: 可选 Token 提取器，适用于无需强制登录的场景
/// English: Optional token extractor for routes without mandatory login
pub struct OptionalSaTokenExtractor(pub Option<TokenValue>);

impl OptionalSaTokenExtractor {
    /// 中文: 返回 Option<TokenValue>，不存在则为 None
    /// English: Returns Option<TokenValue>, None when token absent
    pub fn from_request(req: &Request) -> Self {
        let token = req.extensions().get::<TokenValue>().cloned();
        OptionalSaTokenExtractor(token)
    }
}

/// 中文: 登录 ID 提取器，从请求扩展中获取 login_id
/// English: Login ID extractor fetching login_id from request extensions
pub struct LoginIdExtractor(pub Option<String>);

impl LoginIdExtractor {
    /// 中文: 若登录状态已建立，中间件会写入 login_id
    /// English: Middleware stores login_id when session is authenticated
    pub fn from_request(req: &Request) -> Self {
        let login_id = req.extensions()
            .get::<String>()
            .cloned();
        LoginIdExtractor(login_id)
    }
}

