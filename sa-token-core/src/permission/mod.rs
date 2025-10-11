//! 权限验证模块

use async_trait::async_trait;
use crate::error::SaTokenResult;

/// 权限检查器
#[async_trait]
pub trait PermissionChecker: Send + Sync {
    /// 检查用户是否拥有指定权限
    async fn has_permission(&self, login_id: &str, permission: &str) -> SaTokenResult<bool>;
    
    /// 检查用户是否拥有所有指定权限
    async fn has_all_permissions(&self, login_id: &str, permissions: &[&str]) -> SaTokenResult<bool> {
        for permission in permissions {
            if !self.has_permission(login_id, permission).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    
    /// 检查用户是否拥有任一指定权限
    async fn has_any_permission(&self, login_id: &str, permissions: &[&str]) -> SaTokenResult<bool> {
        for permission in permissions {
            if self.has_permission(login_id, permission).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    /// 获取用户的所有权限列表
    async fn get_permissions(&self, login_id: &str) -> SaTokenResult<Vec<String>>;
}

/// 角色检查器
#[async_trait]
pub trait RoleChecker: Send + Sync {
    /// 检查用户是否拥有指定角色
    async fn has_role(&self, login_id: &str, role: &str) -> SaTokenResult<bool>;
    
    /// 检查用户是否拥有所有指定角色
    async fn has_all_roles(&self, login_id: &str, roles: &[&str]) -> SaTokenResult<bool> {
        for role in roles {
            if !self.has_role(login_id, role).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    
    /// 检查用户是否拥有任一指定角色
    async fn has_any_role(&self, login_id: &str, roles: &[&str]) -> SaTokenResult<bool> {
        for role in roles {
            if self.has_role(login_id, role).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    /// 获取用户的所有角色列表
    async fn get_roles(&self, login_id: &str) -> SaTokenResult<Vec<String>>;
}

