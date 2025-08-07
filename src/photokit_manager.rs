//! PhotoKit 权限管理器
//!
//! 此模块提供 PhotoKit 权限管理的核心逻辑，包括权限状态检查、权限请求协调、
//! 跨平台兼容性处理、错误处理和线程安全的状态管理。

use crate::{PhotoKitAccessLevel, PhotoKitAuthorizationStatus};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "macos")]
use crate::photokit_bridge::{PhotoKitBridge, PhotoKitBridgeError};

/// PhotoKit 权限管理器错误类型
#[derive(Debug, thiserror::Error)]
pub enum PhotoKitManagerError {
    #[error("平台不支持 PhotoKit")]
    PlatformNotSupported,
    #[error("权限检查失败: {0}")]
    CheckFailed(String),
    #[error("权限请求失败: {0}")]
    RequestFailed(String),
    #[error("状态缓存错误: {0}")]
    CacheError(String),
    #[cfg(target_os = "macos")]
    #[error("桥接层错误: {0}")]
    BridgeError(#[from] PhotoKitBridgeError),
}

/// 权限状态缓存项
#[derive(Debug, Clone)]
struct CacheEntry {
    status: PhotoKitAuthorizationStatus,
    timestamp: std::time::SystemTime,
}

/// PhotoKit 权限管理器
///
/// 提供线程安全的 PhotoKit 权限管理功能，包括状态检查、权限请求和状态缓存。
pub struct PhotoKitPermissionManager {
    #[cfg(target_os = "macos")]
    bridge: PhotoKitBridge,

    /// 权限状态缓存，按权限级别存储
    cache: Arc<Mutex<HashMap<PhotoKitAccessLevel, CacheEntry>>>,

    /// 缓存过期时间（秒）
    cache_ttl: u64,
}

impl PhotoKitPermissionManager {
    /// 创建新的 PhotoKit 权限管理器实例
    ///
    /// # Arguments
    /// * `cache_ttl` - 缓存过期时间（秒），默认为 30 秒
    pub fn new(cache_ttl: Option<u64>) -> Self {
        Self {
            #[cfg(target_os = "macos")]
            bridge: PhotoKitBridge::new(),
            cache: Arc::new(Mutex::new(HashMap::new())),
            cache_ttl: cache_ttl.unwrap_or(30),
        }
    }

    /// 检查指定权限级别的当前授权状态
    ///
    /// 此方法首先检查缓存，如果缓存有效则直接返回，否则调用底层 API 获取最新状态。
    ///
    /// # Arguments
    /// * `access_level` - 要检查的权限级别
    ///
    /// # Returns
    /// 返回当前的权限授权状态
    ///
    /// # Errors
    /// 如果权限检查失败，返回相应的错误
    pub fn check_authorization_status(
        &self,
        access_level: PhotoKitAccessLevel,
    ) -> Result<PhotoKitAuthorizationStatus, PhotoKitManagerError> {
        // 首先检查缓存
        if let Some(cached_status) = self.get_cached_status(access_level)? {
            return Ok(cached_status);
        }

        // 缓存未命中或已过期，调用底层 API
        let status = self.check_status_from_system(access_level)?;

        // 更新缓存
        self.update_cache(access_level, status)?;

        Ok(status)
    }

    /// 请求指定权限级别的授权
    ///
    /// 此方法会触发系统权限请求对话框，并在用户响应后更新缓存。
    ///
    /// # Arguments
    /// * `access_level` - 要请求的权限级别
    ///
    /// # Returns
    /// 返回用户响应后的权限授权状态
    ///
    /// # Errors
    /// 如果权限请求失败，返回相应的错误
    pub fn request_authorization(
        &self,
        access_level: PhotoKitAccessLevel,
    ) -> Result<PhotoKitAuthorizationStatus, PhotoKitManagerError> {
        let status = self.request_authorization_from_system(access_level)?;

        // 更新缓存
        self.update_cache(access_level, status)?;

        Ok(status)
    }

    /// 清除指定权限级别的缓存
    ///
    /// # Arguments
    /// * `access_level` - 要清除缓存的权限级别，如果为 None 则清除所有缓存
    pub fn clear_cache(
        &self,
        access_level: Option<PhotoKitAccessLevel>,
    ) -> Result<(), PhotoKitManagerError> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PhotoKitManagerError::CacheError(format!("获取缓存锁失败: {}", e)))?;

        match access_level {
            Some(level) => {
                cache.remove(&level);
            }
            None => {
                cache.clear();
            }
        }

        Ok(())
    }

    /// 检查 PhotoKit 框架是否可用
    ///
    /// # Returns
    /// 如果 PhotoKit 框架可用返回 true，否则返回 false
    pub fn is_framework_available(&self) -> bool {
        #[cfg(target_os = "macos")]
        {
            self.bridge.is_framework_available()
        }

        #[cfg(not(target_os = "macos"))]
        {
            false
        }
    }

    /// 获取照片库中的总照片数量
    ///
    /// 此方法会查询照片库中所有图片类型的资源数量。
    /// 需要用户已授予读取权限才能成功查询。
    ///
    /// # Returns
    /// 返回照片库中的总照片数量
    ///
    /// # Errors
    /// 如果没有权限或查询失败，返回相应的错误
    pub fn get_photos_count(&self) -> Result<u64, PhotoKitManagerError> {
        #[cfg(target_os = "macos")]
        {
            self.bridge
                .get_photos_count()
                .map_err(PhotoKitManagerError::from)
        }

        #[cfg(not(target_os = "macos"))]
        {
            // 在非 macOS 平台，返回 0 以保持兼容性
            Ok(0)
        }
    }

    /// 从缓存获取权限状态
    fn get_cached_status(
        &self,
        access_level: PhotoKitAccessLevel,
    ) -> Result<Option<PhotoKitAuthorizationStatus>, PhotoKitManagerError> {
        let cache = self
            .cache
            .lock()
            .map_err(|e| PhotoKitManagerError::CacheError(format!("获取缓存锁失败: {}", e)))?;

        if let Some(entry) = cache.get(&access_level) {
            // 检查缓存是否过期
            if let Ok(elapsed) = entry.timestamp.elapsed() {
                if elapsed.as_secs() < self.cache_ttl {
                    return Ok(Some(entry.status));
                }
            }
        }

        Ok(None)
    }

    /// 更新缓存
    fn update_cache(
        &self,
        access_level: PhotoKitAccessLevel,
        status: PhotoKitAuthorizationStatus,
    ) -> Result<(), PhotoKitManagerError> {
        let mut cache = self
            .cache
            .lock()
            .map_err(|e| PhotoKitManagerError::CacheError(format!("获取缓存锁失败: {}", e)))?;

        cache.insert(
            access_level,
            CacheEntry {
                status,
                timestamp: std::time::SystemTime::now(),
            },
        );

        Ok(())
    }

    /// 从系统检查权限状态
    fn check_status_from_system(
        &self,
        access_level: PhotoKitAccessLevel,
    ) -> Result<PhotoKitAuthorizationStatus, PhotoKitManagerError> {
        #[cfg(target_os = "macos")]
        {
            self.bridge
                .check_authorization_status(access_level)
                .map_err(PhotoKitManagerError::from)
        }

        #[cfg(not(target_os = "macos"))]
        {
            // 在非 macOS 平台，返回已授权状态以保持兼容性
            Ok(PhotoKitAuthorizationStatus::Authorized)
        }
    }

    /// 从系统请求权限授权
    fn request_authorization_from_system(
        &self,
        access_level: PhotoKitAccessLevel,
    ) -> Result<PhotoKitAuthorizationStatus, PhotoKitManagerError> {
        #[cfg(target_os = "macos")]
        {
            self.bridge
                .request_authorization(access_level)
                .map_err(PhotoKitManagerError::from)
        }

        #[cfg(not(target_os = "macos"))]
        {
            // 在非 macOS 平台，直接返回已授权状态
            Ok(PhotoKitAuthorizationStatus::Authorized)
        }
    }
}

impl Default for PhotoKitPermissionManager {
    fn default() -> Self {
        Self::new(None)
    }
}

// 线程安全标记
unsafe impl Send for PhotoKitPermissionManager {}
unsafe impl Sync for PhotoKitPermissionManager {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = PhotoKitPermissionManager::new(Some(60));
        assert!(manager.cache_ttl == 60);
    }

    #[test]
    fn test_manager_default() {
        let manager = PhotoKitPermissionManager::default();
        assert!(manager.cache_ttl == 30);
    }

    #[test]
    fn test_cache_operations() {
        let manager = PhotoKitPermissionManager::new(Some(60));

        // 清除缓存应该成功
        assert!(manager.clear_cache(None).is_ok());
        assert!(manager.clear_cache(Some(PhotoKitAccessLevel::Read)).is_ok());
    }

    #[test]
    fn test_framework_availability() {
        let manager = PhotoKitPermissionManager::new(None);
        // 在测试环境中，框架可用性检查应该不会崩溃
        let _ = manager.is_framework_available();
    }
}
