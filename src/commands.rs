use tauri::{command, AppHandle, Runtime};

#[cfg(target_os = "macos")]
use tauri::Manager;

// Check Accessibility Permissions.
#[command]
pub async fn check_accessibility_permissions() -> bool {
    #[cfg(target_os = "macos")]
    return macos_accessibility_client::accessibility::application_is_trusted();

    #[cfg(not(target_os = "macos"))]
    return true;
}

// Request Accessibility Permissions.
#[command]
pub async fn request_accessibility_permissions() {
    #[cfg(target_os = "macos")]
    macos_accessibility_client::accessibility::application_is_trusted_with_prompt();
}

// Check Full Disk Access Permissions.
#[command]
pub async fn check_full_disk_access_permissions<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        // Reference: https://github.com/inket/FullDiskAccess/blob/846e04ea2b84fce843f47d7e7f3421189221829c/Sources/FullDiskAccess/FullDiskAccess.swift#L46
        let check_dirs = vec!["Library/Containers/com.apple.stocks", "Library/Safari"];

        if let Ok(home_dir) = app_handle.path().home_dir() {
            for dir in check_dirs.iter() {
                if std::fs::read_dir(&home_dir.join(dir)).is_ok() {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = app_handle;

        Ok(true)
    }
}

// Request Full Disk Access Permissions.
#[command]
pub async fn request_full_disk_access_permissions() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .output()
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}
