use tide::Request;
use sa_token_core::token::TokenValue;

/// 中文: Tide 必填 Token 提取器，读取扩展中的 TokenValue
/// English: Tide required token extractor reading TokenValue from request extensions
pub struct SaTokenExtractor(pub Option<TokenValue>);

impl SaTokenExtractor {
    /// 中文: 中间件将 Token 写入扩展，这里转为 Option 返回
    /// English: Middleware writes TokenValue into extensions; convert to Option
    pub fn from_request<State: Clone + Send + Sync + 'static>(req: &Request<State>) -> Self {
        let token = req.ext::<TokenValue>().cloned();
        SaTokenExtractor(token)
    }
}

/// 中文: 可选 Token 提取器，用于无需强制鉴权的接口
/// English: Optional token extractor for routes without mandatory auth
pub struct OptionalSaTokenExtractor(pub Option<TokenValue>);

impl OptionalSaTokenExtractor {
    /// 中文: 直接返回 Option<TokenValue>
    /// English: Returns Option<TokenValue> directly
    pub fn from_request<State: Clone + Send + Sync + 'static>(req: &Request<State>) -> Self {
        let token = req.ext::<TokenValue>().cloned();
        OptionalSaTokenExtractor(token)
    }
}

/// 中文: 登录 ID 提取器，从扩展中获取 login_id
/// English: Login ID extractor retrieving login_id from extensions
pub struct LoginIdExtractor(pub Option<String>);

impl LoginIdExtractor {
    /// 中文: 若登录成功，中间件会注入 login_id
    /// English: Middleware injects login_id when user authenticated
    pub fn from_request<State: Clone + Send + Sync + 'static>(req: &Request<State>) -> Self {
        let login_id = req.ext::<String>().cloned();
        LoginIdExtractor(login_id)
    }
}

