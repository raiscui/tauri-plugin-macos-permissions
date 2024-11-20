use tauri::{command, AppHandle, Manager, Runtime};

// Check Accessibility Permissions
#[command]
pub async fn check_accessibility_permissions() -> bool {
    #[cfg(target_os = "macos")]
    return macos_accessibility_client::accessibility::application_is_trusted();

    #[cfg(not(target_os = "macos"))]
    return true;
}

// Request Accessibility Permissions
#[command]
pub async fn request_accessibility_permissions() -> bool {
    #[cfg(target_os = "macos")]
    return macos_accessibility_client::accessibility::application_is_trusted_with_prompt();

    #[cfg(not(target_os = "macos"))]
    return true;
}

// Check Full Disk Access Permissions
#[command]
pub async fn check_full_disk_access_permissions<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        // Reference: https://github.com/inket/FullDiskAccess/blob/846e04ea2b84fce843f47d7e7f3421189221829c/Sources/FullDiskAccess/FullDiskAccess.swift#L46
        let check_dirs = vec!["Library/Containers/com.apple.stocks", "Library/Safari"];

        let home_dir = app_handle.path().home_dir().map_err(|err| {
            let error = format!("Failed to get home directory: {}", err);

            log::error!("{error}");

            error
        })?;

        for dir in check_dirs.iter() {
            let full_path = home_dir.join(dir);

            if std::fs::read_dir(&full_path).is_ok() {
                return Ok(true);
            }
        }

        Ok(false)
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = app_handle;

        return true;
    }
}

// Request Full Disk Access Permissions
#[command]
pub async fn request_full_disk_access_permissions() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .output()
            .map_err(|err| {
                let error = format!("Failed to open Security & Privacy settings: {}", err);

                log::error!("{error}");

                error
            })?;

        Ok(())
    }
}
