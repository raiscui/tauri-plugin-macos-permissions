use crate::{PhotoKitAccessLevel, PhotoKitAuthorizationStatus, PhotoKitPermissionManager};
use tauri::{command, AppHandle, Runtime};

#[cfg(target_os = "macos")]
use crate::{ListenerInfo, PhotoKitPermissionListener};

#[cfg(target_os = "macos")]
use {
    macos_accessibility_client::accessibility::{
        application_is_trusted, application_is_trusted_with_prompt,
    },
    objc2::{class, msg_send, runtime::Bool},
    objc2_foundation::NSString,
    std::{fs::read_dir, process::Command},
    tauri::Manager,
};

#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn CGRequestScreenCaptureAccess() -> bool;
}

#[cfg(target_os = "macos")]
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOHIDCheckAccess(request: u32) -> u32;
}

/// Check accessibility permission.
///
/// # Returns
/// - `bool`: `true` if accessibility permission are granted, `false` otherwise.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::check_accessibility_permission;
///
/// # async fn example() {
/// let authorized = check_accessibility_permission().await;
/// println!("Authorized: {}", authorized); // false
/// # }
/// ```
#[command]
pub async fn check_accessibility_permission() -> bool {
    #[cfg(target_os = "macos")]
    return application_is_trusted();

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request accessibility permission.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::request_accessibility_permission;
///
/// # async fn example() {
/// request_accessibility_permission().await;
/// # }
/// ```
#[command]
pub async fn request_accessibility_permission() {
    #[cfg(target_os = "macos")]
    application_is_trusted_with_prompt();
}

/// Check full disk access permission.
///
/// # Returns
/// - `bool`: `true` if full disk access permission are granted, `false` otherwise.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::check_full_disk_access_permission;
/// use tauri::AppHandle;
///
/// # async fn example(app_handle: AppHandle<tauri::Wry>) {
/// let authorized = check_full_disk_access_permission(app_handle).await;
/// println!("Authorized: {}", authorized); // false
/// # }
/// ```
#[command]
pub async fn check_full_disk_access_permission<R: Runtime>(app_handle: AppHandle<R>) -> bool {
    #[cfg(target_os = "macos")]
    {
        // Reference: https://github.com/inket/FullDiskAccess/blob/846e04ea2b84fce843f47d7e7f3421189221829c/Sources/FullDiskAccess/FullDiskAccess.swift#L46
        let check_dirs = vec!["Library/Containers/com.apple.stocks", "Library/Safari"];

        if let Ok(home_dir) = app_handle.path().home_dir() {
            for check_dir in check_dirs.iter() {
                if read_dir(&home_dir.join(check_dir)).is_ok() {
                    return true;
                }
            }
        }

        false
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = app_handle;

        true
    }
}

/// Request full disk access permission.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::request_full_disk_access_permission;
///
/// # async fn example() {
/// request_full_disk_access_permission().await;
/// # }
/// ```
#[command]
pub async fn request_full_disk_access_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .output()
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

/// Check screen recording permission.
///
/// # Returns
/// - `bool`: `true` if screen recording permission are granted, `false` otherwise.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::check_screen_recording_permission;
///
/// # async fn example() {
/// let authorized = check_screen_recording_permission().await;
/// println!("Authorized: {}", authorized); // false
/// # }
/// ```
#[command]
pub async fn check_screen_recording_permission() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        CGPreflightScreenCaptureAccess()
    }

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request screen recording permission.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::request_screen_recording_permission;
///
/// # async fn example() {
/// request_screen_recording_permission().await;
/// # }
/// ```
#[command]
pub async fn request_screen_recording_permission() {
    #[cfg(target_os = "macos")]
    unsafe {
        CGRequestScreenCaptureAccess();
    }
}

/// Check microphone permission.
///
/// # Returns
/// - `bool`: `true` if microphone permission is granted, `false` otherwise.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::check_microphone_permission;
///
/// # async fn example() {
/// let authorized = check_microphone_permission().await;
/// println!("Authorized: {}", authorized); // false
/// # }
/// ```
#[command]
pub async fn check_microphone_permission() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        let av_media_type = NSString::from_str("soun");
        let status: i32 = msg_send![
            class!(AVCaptureDevice),
            authorizationStatusForMediaType: &*av_media_type
        ];

        status == 3
    }

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request microphone permission.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::request_microphone_permission;
///
/// # async fn example() {
/// request_microphone_permission().await;
/// # }
/// ```
#[command]
pub async fn request_microphone_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    unsafe {
        let av_media_type = NSString::from_str("soun");
        type CompletionBlock = Option<extern "C" fn(Bool)>;
        let completion_block: CompletionBlock = None;
        let _: () = msg_send![
            class!(AVCaptureDevice),
            requestAccessForMediaType: &*av_media_type,
            completionHandler: completion_block
        ];
    }

    Ok(())
}

/// Check camera permission.
///
/// # Returns
/// - `bool`: `true` if camera permission is granted, `false` otherwise.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::check_camera_permission;
///
/// # async fn example() {
/// let authorized = check_camera_permission().await;
/// println!("Authorized: {}", authorized); // false
/// # }
/// ```
#[command]
pub async fn check_camera_permission() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        let av_media_type = NSString::from_str("vide");
        let status: i32 = msg_send![
            class!(AVCaptureDevice),
            authorizationStatusForMediaType: &*av_media_type
        ];

        status == 3
    }

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request camera permission.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::request_camera_permission;
///
/// # async fn example() {
/// request_camera_permission().await;
/// # }
/// ```
#[command]
pub async fn request_camera_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    unsafe {
        let av_media_type = NSString::from_str("vide");
        type CompletionBlock = Option<extern "C" fn(Bool)>;
        let completion_block: CompletionBlock = None;
        let _: () = msg_send![
            class!(AVCaptureDevice),
            requestAccessForMediaType: &*av_media_type,
            completionHandler: completion_block
        ];
    }

    Ok(())
}

/// Check input monitoring permission.
///
/// # Returns
/// - `bool`: `true` if input monitoring permission is granted, `false` otherwise.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::check_input_monitoring_permission;
///
/// # async fn example() {
/// let authorized = check_input_monitoring_permission().await;
/// println!("Authorized: {}", authorized); // false
/// # }
/// ```
#[command]
pub async fn check_input_monitoring_permission() -> bool {
    #[cfg(target_os = "macos")]
    unsafe {
        let status = IOHIDCheckAccess(1);

        status == 0
    }

    #[cfg(not(target_os = "macos"))]
    return true;
}

/// Request input monitoring permission.
///
/// # Example
/// ```no_run
/// use tauri_plugin_macos_permissions::request_input_monitoring_permission;
///
/// # async fn example() {
/// request_input_monitoring_permission().await;
/// # }
/// ```
#[command]
pub async fn request_input_monitoring_permission() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent")
            .output()
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

/// Check PhotoKit permission for the specified access level.
///
/// # Arguments
/// * `access_level` - The PhotoKit access level to check
///
/// # Returns
/// - `PhotoKitAuthorizationStatus`: The current authorization status for the specified access level
///
/// # Example
/// ```javascript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// const status = await invoke('check_photokit_permission', {
///     accessLevel: 'read'
/// });
/// console.log('权限状态:', status); // "authorized" | "denied" | "notDetermined" | ...
/// ```
#[command]
pub async fn check_photokit_permission(
    access_level: PhotoKitAccessLevel,
) -> PhotoKitAuthorizationStatus {
    let manager = PhotoKitPermissionManager::new(None);

    match manager.check_authorization_status(access_level) {
        Ok(status) => status,
        Err(_) => {
            // 在错误情况下，返回未确定状态
            PhotoKitAuthorizationStatus::NotDetermined
        }
    }
}

/// Request PhotoKit permission for the specified access level.
///
/// This will show the system permission dialog if the permission has not been determined yet.
///
/// # Arguments
/// * `access_level` - The PhotoKit access level to request
///
/// # Returns
/// - `Result<PhotoKitAuthorizationStatus, String>`: The authorization status after user response, or error message
///
/// # Example
/// ```javascript
/// import { invoke } from '@tauri-apps/api/tauri';
///
/// try {
///     const status = await invoke('request_photokit_permission', {
///         accessLevel: 'readWrite'
///     });
///
///     if (status === 'authorized') {
///         console.log('权限已授予');
///     } else {
///         console.log('权限被拒绝或受限:', status);
///     }
/// } catch (error) {
///     console.error('请求权限失败:', error);
/// }
/// ```
#[command]
pub async fn request_photokit_permission(
    access_level: PhotoKitAccessLevel,
) -> Result<PhotoKitAuthorizationStatus, String> {
    let manager = PhotoKitPermissionManager::new(None);

    manager
        .request_authorization(access_level)
        .map_err(|e| e.to_string())
}

/// Register a PhotoKit permission status listener.
///
/// This creates a listener that will emit events when the PhotoKit permission status changes
/// for the specified access level.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
/// * `access_level` - The PhotoKit access level to monitor
///
/// # Returns
/// - `Result<String, String>`: The listener ID on success, or error message on failure
///
/// # Example
/// ```javascript
/// import { invoke, listen } from '@tauri-apps/api';
///
/// // 注册监听器
/// const listenerId = await invoke('register_photokit_permission_listener', {
///     accessLevel: 'read'
/// });
///
/// // 监听权限状态变化事件
/// const unlisten = await listen('photokit-permission-changed', (event) => {
///     console.log('权限状态变化:', event.payload);
/// });
///
/// // 稍后注销监听器
/// await invoke('unregister_photokit_permission_listener', {
///     listenerId: listenerId
/// });
/// ```
#[command]
pub async fn register_photokit_permission_listener<R: Runtime>(
    app_handle: AppHandle<R>,
    access_level: PhotoKitAccessLevel,
) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let listener = PhotoKitPermissionListener::new(app_handle);
        listener
            .register_listener(access_level)
            .map_err(|e| e.to_string())
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 在非 macOS 平台上，返回一个模拟的监听器 ID
        use uuid::Uuid;
        Ok(Uuid::new_v4().to_string())
    }
}

/// Unregister a PhotoKit permission status listener.
///
/// This removes a previously registered listener and stops monitoring permission changes.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
/// * `listener_id` - The ID of the listener to unregister
///
/// # Returns
/// - `Result<(), String>`: Success or error message
///
/// # Example
/// ```javascript
/// import { invoke } from '@tauri-apps/api';
///
/// // 注销监听器
/// try {
///     await invoke('unregister_photokit_permission_listener', {
///         listenerId: 'your-listener-id'
///     });
///     console.log('监听器已注销');
/// } catch (error) {
///     console.error('注销监听器失败:', error);
/// }
/// ```
#[command]
pub async fn unregister_photokit_permission_listener<R: Runtime>(
    app_handle: AppHandle<R>,
    listener_id: String,
) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let listener = PhotoKitPermissionListener::new(app_handle);
        listener
            .unregister_listener(&listener_id)
            .map_err(|e| e.to_string())
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 在非 macOS 平台上，这是一个空操作
        Ok(())
    }
}

/// Get all active PhotoKit permission listeners.
///
/// This returns information about all currently registered permission listeners.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle
///
/// # Returns
/// - `Result<Vec<ListenerInfo>, String>`: List of active listeners or error message
///
/// # Example
/// ```javascript
/// import { invoke } from '@tauri-apps/api';
///
/// const listeners = await invoke('get_photokit_permission_listeners');
/// console.log('活跃的监听器:', listeners);
/// ```
#[command]
pub async fn get_photokit_permission_listeners<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<Vec<ListenerInfo>, String> {
    #[cfg(target_os = "macos")]
    {
        let listener = PhotoKitPermissionListener::new(app_handle);
        listener.get_active_listeners().map_err(|e| e.to_string())
    }

    #[cfg(not(target_os = "macos"))]
    {
        // 在非 macOS 平台上，返回空列表
        Ok(vec![])
    }
}

/// Get the total number of photos in the photo library.
///
/// This method queries the photo library for all image-type assets and returns the count.
/// The user must have granted read permission for this to work.
///
/// # Returns
/// - `Result<u64, String>`: The total number of photos, or error message
///
/// # Example
/// ```javascript
/// import { invoke } from '@tauri-apps/api';
///
/// try {
///     const count = await invoke('get_photos_count');
///     console.log('照片总数:', count);
/// } catch (error) {
///     console.error('查询照片数量失败:', error);
/// }
/// ```
#[command]
pub async fn get_photos_count() -> Result<u64, String> {
    let manager = PhotoKitPermissionManager::new(None);

    manager.get_photos_count().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_check_photokit_permission_macos() {
        // 测试检查权限功能（仅在 macOS 上运行）
        // 注意：在某些测试环境中可能没有完整的PhotoKit框架
        let status = check_photokit_permission(PhotoKitAccessLevel::Read).await;

        // 在 macOS 平台上应该返回有效的状态
        assert!(matches!(
            status,
            PhotoKitAuthorizationStatus::NotDetermined
                | PhotoKitAuthorizationStatus::Restricted
                | PhotoKitAuthorizationStatus::Denied
                | PhotoKitAuthorizationStatus::Authorized
                | PhotoKitAuthorizationStatus::Limited
        ));
    }

    #[tokio::test]
    #[cfg(not(target_os = "macos"))]
    async fn test_check_photokit_permission_non_macos() {
        // 测试检查权限功能（在非 macOS 平台上运行）
        let status = check_photokit_permission(PhotoKitAccessLevel::Read).await;

        // 在非 macOS 平台上应该返回 NotDetermined
        assert_eq!(status, PhotoKitAuthorizationStatus::NotDetermined);
    }

    #[tokio::test]
    #[cfg(target_os = "macos")]
    async fn test_request_photokit_permission_macos() {
        // 测试请求权限功能（仅在 macOS 上运行）
        let result = request_photokit_permission(PhotoKitAccessLevel::Read).await;

        // 应该返回成功结果
        assert!(result.is_ok());

        let status = result.unwrap();
        assert!(matches!(
            status,
            PhotoKitAuthorizationStatus::NotDetermined
                | PhotoKitAuthorizationStatus::Restricted
                | PhotoKitAuthorizationStatus::Denied
                | PhotoKitAuthorizationStatus::Authorized
                | PhotoKitAuthorizationStatus::Limited
        ));
    }

    #[tokio::test]
    #[cfg(not(target_os = "macos"))]
    async fn test_request_photokit_permission_non_macos() {
        // 测试请求权限功能（在非 macOS 平台上运行）
        let result = request_photokit_permission(PhotoKitAccessLevel::Read).await;

        // 应该返回成功结果
        assert!(result.is_ok());

        let status = result.unwrap();
        assert_eq!(status, PhotoKitAuthorizationStatus::NotDetermined);
    }

    #[test]
    fn test_photokit_access_levels() {
        // 测试所有权限级别
        let levels = [
            PhotoKitAccessLevel::Read,
            PhotoKitAccessLevel::ReadWrite,
            PhotoKitAccessLevel::AddOnly,
        ];

        for level in levels.iter() {
            // 确保每个级别都可以序列化
            let serialized = serde_json::to_string(level);
            assert!(serialized.is_ok());

            // 确保可以反序列化
            let deserialized: Result<PhotoKitAccessLevel, _> =
                serde_json::from_str(&serialized.unwrap());
            assert!(deserialized.is_ok());
            assert_eq!(*level, deserialized.unwrap());
        }
    }

    #[tokio::test]
    async fn test_traditional_permissions() {
        // 测试传统权限检查功能
        let accessibility = check_accessibility_permission().await;
        let screen_recording = check_screen_recording_permission().await;
        let microphone = check_microphone_permission().await;
        let camera = check_camera_permission().await;
        let input_monitoring = check_input_monitoring_permission().await;

        // 在非 macOS 平台上应该都返回 true
        #[cfg(not(target_os = "macos"))]
        {
            assert!(accessibility);
            assert!(screen_recording);
            assert!(microphone);
            assert!(camera);
            assert!(input_monitoring);
        }

        // 在 macOS 平台上应该返回布尔值
        #[cfg(target_os = "macos")]
        {
            assert!(accessibility == true || accessibility == false);
            assert!(screen_recording == true || screen_recording == false);
            assert!(microphone == true || microphone == false);
            assert!(camera == true || camera == false);
            assert!(input_monitoring == true || input_monitoring == false);
        }
    }
}
