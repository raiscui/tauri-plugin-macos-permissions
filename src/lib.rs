use serde::{Deserialize, Serialize};
use tauri::{
    generate_handler,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;

#[cfg(target_os = "macos")]
mod photokit_bridge;

mod photokit_manager;

#[cfg(target_os = "macos")]
mod photokit_listener;

pub use commands::*;
pub use photokit_manager::*;

#[cfg(target_os = "macos")]
pub use photokit_listener::*;

/// PhotoKit 访问权限级别
///
/// 定义了应用可以请求的不同级别的照片库访问权限。
/// 这些级别对应于 macOS PhotoKit 框架中的 PHAccessLevel 枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhotoKitAccessLevel {
    /// 只读权限 - 可以读取照片和视频，但不能修改或删除
    /// 对应 PHAccessLevelRead
    #[serde(rename = "read")]
    Read,

    /// 读写权限 - 可以读取、修改和删除照片和视频
    /// 对应 PHAccessLevelReadWrite
    #[serde(rename = "readWrite")]
    ReadWrite,

    /// 添加权限 - 只能添加新的照片和视频，不能访问现有内容
    /// 对应 PHAccessLevelAddOnly
    #[serde(rename = "addOnly")]
    AddOnly,
}

/// PhotoKit 权限授权状态
///
/// 表示应用当前的照片库访问权限状态。
/// 这些状态对应于 macOS PhotoKit 框架中的 PHAuthorizationStatus 枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhotoKitAuthorizationStatus {
    /// 未确定 - 用户尚未被询问权限，或者应用首次请求权限
    /// 对应 PHAuthorizationStatusNotDetermined
    #[serde(rename = "notDetermined")]
    NotDetermined,

    /// 受限 - 由于家长控制等原因，应用被限制访问照片库
    /// 对应 PHAuthorizationStatusRestricted
    #[serde(rename = "restricted")]
    Restricted,

    /// 已拒绝 - 用户明确拒绝了应用的照片库访问权限
    /// 对应 PHAuthorizationStatusDenied
    #[serde(rename = "denied")]
    Denied,

    /// 已授权 - 用户已授予应用完整的照片库访问权限
    /// 对应 PHAuthorizationStatusAuthorized
    #[serde(rename = "authorized")]
    Authorized,

    /// 有限访问 - 用户选择了部分照片访问（iOS 14+ 功能）
    /// 对应 PHAuthorizationStatusLimited
    #[serde(rename = "limited")]
    Limited,
}

/// 权限状态变化事件
///
/// 当照片库权限状态发生变化时，通过此结构体传递事件信息。
/// 用于实时监听权限状态的变化。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionStatusChangeEvent {
    /// 新的权限状态
    pub new_status: PhotoKitAuthorizationStatus,

    /// 相关的权限级别
    pub access_level: PhotoKitAccessLevel,

    /// 变化发生的时间戳（Unix 时间戳，毫秒）
    pub timestamp: u64,
}

impl PhotoKitAccessLevel {
    /// 将 PhotoKitAccessLevel 转换为对应的 PhotoKit 原生常量值
    ///
    /// # Returns
    /// 返回对应的 PHAccessLevel 整数值：
    /// - Read: 1 (PHAccessLevelRead)
    /// - ReadWrite: 2 (PHAccessLevelReadWrite)
    /// - AddOnly: 3 (PHAccessLevelAddOnly)
    pub fn to_native_value(self) -> i32 {
        match self {
            PhotoKitAccessLevel::Read => 1,
            PhotoKitAccessLevel::ReadWrite => 2,
            PhotoKitAccessLevel::AddOnly => 3,
        }
    }

    /// 从 PhotoKit 原生常量值创建 PhotoKitAccessLevel
    ///
    /// # Arguments
    /// * `value` - PHAccessLevel 的整数值
    ///
    /// # Returns
    /// 对应的 PhotoKitAccessLevel，如果值无效则返回 None
    pub fn from_native_value(value: i32) -> Option<Self> {
        match value {
            1 => Some(PhotoKitAccessLevel::Read),
            2 => Some(PhotoKitAccessLevel::ReadWrite),
            3 => Some(PhotoKitAccessLevel::AddOnly),
            _ => None,
        }
    }
}

impl PhotoKitAuthorizationStatus {
    /// 将 PhotoKitAuthorizationStatus 转换为对应的 PhotoKit 原生常量值
    ///
    /// # Returns
    /// 返回对应的 PHAuthorizationStatus 整数值：
    /// - NotDetermined: 0 (PHAuthorizationStatusNotDetermined)
    /// - Restricted: 1 (PHAuthorizationStatusRestricted)
    /// - Denied: 2 (PHAuthorizationStatusDenied)
    /// - Authorized: 3 (PHAuthorizationStatusAuthorized)
    /// - Limited: 4 (PHAuthorizationStatusLimited)
    pub fn to_native_value(self) -> i32 {
        match self {
            PhotoKitAuthorizationStatus::NotDetermined => 0,
            PhotoKitAuthorizationStatus::Restricted => 1,
            PhotoKitAuthorizationStatus::Denied => 2,
            PhotoKitAuthorizationStatus::Authorized => 3,
            PhotoKitAuthorizationStatus::Limited => 4,
        }
    }

    /// 从 PhotoKit 原生常量值创建 PhotoKitAuthorizationStatus
    ///
    /// # Arguments
    /// * `value` - PHAuthorizationStatus 的整数值
    ///
    /// # Returns
    /// 对应的 PhotoKitAuthorizationStatus，如果值无效则返回 None
    pub fn from_native_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(PhotoKitAuthorizationStatus::NotDetermined),
            1 => Some(PhotoKitAuthorizationStatus::Restricted),
            2 => Some(PhotoKitAuthorizationStatus::Denied),
            3 => Some(PhotoKitAuthorizationStatus::Authorized),
            4 => Some(PhotoKitAuthorizationStatus::Limited),
            _ => None,
        }
    }

    /// 检查权限状态是否表示已授权（包括完全授权和有限授权）
    ///
    /// # Returns
    /// 如果状态为 Authorized 或 Limited 则返回 true，否则返回 false
    pub fn is_authorized(self) -> bool {
        matches!(
            self,
            PhotoKitAuthorizationStatus::Authorized | PhotoKitAuthorizationStatus::Limited
        )
    }
}

impl PermissionStatusChangeEvent {
    /// 创建新的权限状态变化事件
    ///
    /// # Arguments
    /// * `new_status` - 新的权限状态
    /// * `access_level` - 相关的权限级别
    ///
    /// # Returns
    /// 新的 PermissionStatusChangeEvent 实例，时间戳为当前时间
    pub fn new(new_status: PhotoKitAuthorizationStatus, access_level: PhotoKitAccessLevel) -> Self {
        Self {
            new_status,
            access_level,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("macos-permissions")
        .invoke_handler(generate_handler![
            commands::check_accessibility_permission,
            commands::request_accessibility_permission,
            commands::check_full_disk_access_permission,
            commands::request_full_disk_access_permission,
            commands::check_screen_recording_permission,
            commands::request_screen_recording_permission,
            commands::check_microphone_permission,
            commands::request_microphone_permission,
            commands::check_camera_permission,
            commands::request_camera_permission,
            commands::check_input_monitoring_permission,
            commands::request_input_monitoring_permission,
            commands::check_photokit_permission,
            commands::request_photokit_permission,
            commands::register_photokit_permission_listener,
            commands::unregister_photokit_permission_listener,
            commands::get_photokit_permission_listeners
        ])
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photokit_access_level_native_conversion() {
        // 测试 PhotoKitAccessLevel 与原生值的转换
        assert_eq!(PhotoKitAccessLevel::Read.to_native_value(), 1);
        assert_eq!(PhotoKitAccessLevel::ReadWrite.to_native_value(), 2);
        assert_eq!(PhotoKitAccessLevel::AddOnly.to_native_value(), 3);

        assert_eq!(
            PhotoKitAccessLevel::from_native_value(1),
            Some(PhotoKitAccessLevel::Read)
        );
        assert_eq!(
            PhotoKitAccessLevel::from_native_value(2),
            Some(PhotoKitAccessLevel::ReadWrite)
        );
        assert_eq!(
            PhotoKitAccessLevel::from_native_value(3),
            Some(PhotoKitAccessLevel::AddOnly)
        );
        assert_eq!(PhotoKitAccessLevel::from_native_value(999), None);
    }

    #[test]
    fn test_photokit_authorization_status_native_conversion() {
        // 测试 PhotoKitAuthorizationStatus 与原生值的转换
        assert_eq!(
            PhotoKitAuthorizationStatus::NotDetermined.to_native_value(),
            0
        );
        assert_eq!(PhotoKitAuthorizationStatus::Restricted.to_native_value(), 1);
        assert_eq!(PhotoKitAuthorizationStatus::Denied.to_native_value(), 2);
        assert_eq!(PhotoKitAuthorizationStatus::Authorized.to_native_value(), 3);
        assert_eq!(PhotoKitAuthorizationStatus::Limited.to_native_value(), 4);

        assert_eq!(
            PhotoKitAuthorizationStatus::from_native_value(0),
            Some(PhotoKitAuthorizationStatus::NotDetermined)
        );
        assert_eq!(
            PhotoKitAuthorizationStatus::from_native_value(1),
            Some(PhotoKitAuthorizationStatus::Restricted)
        );
        assert_eq!(
            PhotoKitAuthorizationStatus::from_native_value(2),
            Some(PhotoKitAuthorizationStatus::Denied)
        );
        assert_eq!(
            PhotoKitAuthorizationStatus::from_native_value(3),
            Some(PhotoKitAuthorizationStatus::Authorized)
        );
        assert_eq!(
            PhotoKitAuthorizationStatus::from_native_value(4),
            Some(PhotoKitAuthorizationStatus::Limited)
        );
        assert_eq!(PhotoKitAuthorizationStatus::from_native_value(999), None);
    }

    #[test]
    fn test_photokit_authorization_status_is_authorized() {
        // 测试权限状态的授权检查
        assert!(PhotoKitAuthorizationStatus::Authorized.is_authorized());
        assert!(PhotoKitAuthorizationStatus::Limited.is_authorized());
        assert!(!PhotoKitAuthorizationStatus::NotDetermined.is_authorized());
        assert!(!PhotoKitAuthorizationStatus::Restricted.is_authorized());
        assert!(!PhotoKitAuthorizationStatus::Denied.is_authorized());
    }

    #[test]
    fn test_permission_status_change_event_creation() {
        // 测试权限状态变化事件的创建
        let event = PermissionStatusChangeEvent::new(
            PhotoKitAuthorizationStatus::Authorized,
            PhotoKitAccessLevel::Read,
        );

        assert_eq!(event.new_status, PhotoKitAuthorizationStatus::Authorized);
        assert_eq!(event.access_level, PhotoKitAccessLevel::Read);
        assert!(event.timestamp > 0);
    }

    #[test]
    fn test_serde_serialization() {
        // 测试序列化和反序列化
        let access_level = PhotoKitAccessLevel::ReadWrite;
        let json = serde_json::to_string(&access_level).unwrap();
        assert_eq!(json, "\"readWrite\"");

        let deserialized: PhotoKitAccessLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, access_level);

        let status = PhotoKitAuthorizationStatus::Limited;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"limited\"");

        let deserialized: PhotoKitAuthorizationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, status);
    }
}
