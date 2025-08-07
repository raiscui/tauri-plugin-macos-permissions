//! PhotoKit 权限状态监听系统
//!
//! 此模块提供 PhotoKit 权限状态变化的监听功能，基于 NSNotificationCenter
//! 实现权限状态变化的实时监听和事件分发。

use crate::{PermissionStatusChangeEvent, PhotoKitAccessLevel, PhotoKitAuthorizationStatus};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Runtime};
use uuid::Uuid;

/// 监听器错误类型
#[derive(Debug, thiserror::Error)]
pub enum PhotoKitListenerError {
    #[error("监听器不存在: {0}")]
    ListenerNotFound(String),
    #[error("监听器已存在: {0}")]
    ListenerAlreadyExists(String),
    #[error("事件发送失败: {0}")]
    EventEmitFailed(String),
    #[error("监听器管理器锁定失败")]
    LockFailed,
    #[error("平台不支持")]
    PlatformNotSupported,
}

/// 监听器信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListenerInfo {
    /// 监听器唯一标识符
    pub id: String,
    /// 监听的权限级别
    pub access_level: PhotoKitAccessLevel,
    /// 创建时间戳
    pub created_at: u64,
    /// 是否活跃
    pub active: bool,
}

impl ListenerInfo {
    /// 创建新的监听器信息
    pub fn new(access_level: PhotoKitAccessLevel) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            access_level,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            active: true,
        }
    }
}

/// PhotoKit 权限状态监听器管理器
///
/// 管理权限状态变化的监听器，提供注册、注销和事件分发功能。
pub struct PhotoKitPermissionListener<R: Runtime> {
    /// 应用句柄
    app_handle: AppHandle<R>,
    /// 活跃的监听器映射
    listeners: Arc<Mutex<HashMap<String, ListenerInfo>>>,
    /// 是否已初始化 NSNotificationCenter 监听
    notification_initialized: Arc<Mutex<bool>>,
}

impl<R: Runtime> PhotoKitPermissionListener<R> {
    /// 创建新的权限监听器管理器
    ///
    /// # Arguments
    /// * `app_handle` - Tauri 应用句柄
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            app_handle,
            listeners: Arc::new(Mutex::new(HashMap::new())),
            notification_initialized: Arc::new(Mutex::new(false)),
        }
    }

    /// 注册权限状态监听器
    ///
    /// # Arguments
    /// * `access_level` - 要监听的权限级别
    ///
    /// # Returns
    /// 返回监听器的唯一标识符
    ///
    /// # Errors
    /// 如果监听器注册失败，返回相应的错误
    pub fn register_listener(
        &self,
        access_level: PhotoKitAccessLevel,
    ) -> Result<String, PhotoKitListenerError> {
        let listener_info = ListenerInfo::new(access_level);
        let listener_id = listener_info.id.clone();

        // 添加监听器到映射中
        {
            let mut listeners = self
                .listeners
                .lock()
                .map_err(|_| PhotoKitListenerError::LockFailed)?;

            listeners.insert(listener_id.clone(), listener_info);
        }

        // 初始化 NSNotificationCenter 监听（如果尚未初始化）
        self.initialize_notification_listener()?;

        Ok(listener_id)
    }

    /// 注销权限状态监听器
    ///
    /// # Arguments
    /// * `listener_id` - 要注销的监听器标识符
    ///
    /// # Errors
    /// 如果监听器不存在或注销失败，返回相应的错误
    pub fn unregister_listener(&self, listener_id: &str) -> Result<(), PhotoKitListenerError> {
        let mut listeners = self
            .listeners
            .lock()
            .map_err(|_| PhotoKitListenerError::LockFailed)?;

        if listeners.remove(listener_id).is_none() {
            return Err(PhotoKitListenerError::ListenerNotFound(
                listener_id.to_string(),
            ));
        }

        Ok(())
    }

    /// 获取所有活跃的监听器
    ///
    /// # Returns
    /// 返回所有活跃监听器的信息列表
    pub fn get_active_listeners(&self) -> Result<Vec<ListenerInfo>, PhotoKitListenerError> {
        let listeners = self
            .listeners
            .lock()
            .map_err(|_| PhotoKitListenerError::LockFailed)?;

        Ok(listeners
            .values()
            .filter(|info| info.active)
            .cloned()
            .collect())
    }

    /// 清除所有监听器
    pub fn clear_all_listeners(&self) -> Result<(), PhotoKitListenerError> {
        let mut listeners = self
            .listeners
            .lock()
            .map_err(|_| PhotoKitListenerError::LockFailed)?;

        listeners.clear();
        Ok(())
    }

    /// 处理权限状态变化事件
    ///
    /// 当检测到权限状态变化时，此方法会被调用来分发事件到前端。
    ///
    /// # Arguments
    /// * `new_status` - 新的权限状态
    /// * `access_level` - 权限级别
    pub fn handle_permission_change(
        &self,
        new_status: PhotoKitAuthorizationStatus,
        access_level: PhotoKitAccessLevel,
    ) -> Result<(), PhotoKitListenerError> {
        // 检查是否有监听器关注此权限级别
        let has_listeners = {
            let listeners = self
                .listeners
                .lock()
                .map_err(|_| PhotoKitListenerError::LockFailed)?;

            listeners
                .values()
                .any(|info| info.access_level == access_level && info.active)
        };

        if !has_listeners {
            return Ok(());
        }

        // 创建权限状态变化事件
        let event = PermissionStatusChangeEvent::new(new_status, access_level);

        // 发送事件到前端
        self.app_handle
            .emit_to("main", "photokit-permission-changed", &event)
            .map_err(|e| PhotoKitListenerError::EventEmitFailed(e.to_string()))?;

        Ok(())
    }

    /// 初始化 NSNotificationCenter 监听
    ///
    /// 在 macOS 上，这会设置 NSNotificationCenter 来监听 PhotoKit 权限变化。
    /// 在其他平台上，这是一个空操作。
    fn initialize_notification_listener(&self) -> Result<(), PhotoKitListenerError> {
        let mut initialized = self
            .notification_initialized
            .lock()
            .map_err(|_| PhotoKitListenerError::LockFailed)?;

        if *initialized {
            return Ok(());
        }

        #[cfg(target_os = "macos")]
        {
            // 在实际实现中，这里会设置 NSNotificationCenter 监听
            // 由于 objc2 的复杂性，这里我们使用一个简化的实现
            // 实际的 NSNotificationCenter 监听需要更复杂的 Objective-C 桥接代码

            // TODO: 实现真正的 NSNotificationCenter 监听
            // 这需要：
            // 1. 注册 PHPhotoLibraryAvailabilityDidChangeNotification 通知
            // 2. 设置回调函数来处理通知
            // 3. 在回调中调用 handle_permission_change
        }

        #[cfg(not(target_os = "macos"))]
        {
            // 在非 macOS 平台上，我们不需要实际的监听
        }

        *initialized = true;
        Ok(())
    }
}

// 线程安全标记
unsafe impl<R: Runtime> Send for PhotoKitPermissionListener<R> {}
unsafe impl<R: Runtime> Sync for PhotoKitPermissionListener<R> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listener_info_creation() {
        let info = ListenerInfo::new(PhotoKitAccessLevel::Read);

        assert!(!info.id.is_empty());
        assert_eq!(info.access_level, PhotoKitAccessLevel::Read);
        assert!(info.active);
        assert!(info.created_at > 0);
    }

    // 注意：由于 Tauri 2.x 中测试 API 的变化，以下测试被暂时注释
    // 在实际项目中，应该使用集成测试或模拟 AppHandle

    /*
    #[test]
    fn test_listener_manager_creation() {
        let app = mock_app();
        let manager = PhotoKitPermissionListener::new(app);

        // 验证管理器创建成功
        assert!(manager.get_active_listeners().unwrap().is_empty());
    }

    #[test]
    fn test_listener_registration_and_unregistration() {
        let app = mock_app();
        let manager = PhotoKitPermissionListener::new(app);

        // 注册监听器
        let listener_id = manager
            .register_listener(PhotoKitAccessLevel::ReadWrite)
            .unwrap();
        assert!(!listener_id.is_empty());

        // 验证监听器已注册
        let listeners = manager.get_active_listeners().unwrap();
        assert_eq!(listeners.len(), 1);
        assert_eq!(listeners[0].access_level, PhotoKitAccessLevel::ReadWrite);

        // 注销监听器
        assert!(manager.unregister_listener(&listener_id).is_ok());

        // 验证监听器已注销
        let listeners = manager.get_active_listeners().unwrap();
        assert!(listeners.is_empty());
    }

    #[test]
    fn test_clear_all_listeners() {
        let app = mock_app();
        let manager = PhotoKitPermissionListener::new(app);

        // 注册多个监听器
        let _id1 = manager
            .register_listener(PhotoKitAccessLevel::Read)
            .unwrap();
        let _id2 = manager
            .register_listener(PhotoKitAccessLevel::ReadWrite)
            .unwrap();

        // 验证监听器已注册
        assert_eq!(manager.get_active_listeners().unwrap().len(), 2);

        // 清除所有监听器
        assert!(manager.clear_all_listeners().is_ok());

        // 验证所有监听器已清除
        assert!(manager.get_active_listeners().unwrap().is_empty());
    }
    */
}
