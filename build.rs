const COMMANDS: &[&str] = &[
    "check_accessibility_permission",
    "request_accessibility_permission",
    "check_full_disk_access_permission",
    "request_full_disk_access_permission",
    "check_screen_recording_permission",
    "request_screen_recording_permission",
    "check_microphone_permission",
    "request_microphone_permission",
    "check_camera_permission",
    "request_camera_permission",
    "check_input_monitoring_permission",
    "request_input_monitoring_permission",
    "check_photokit_permission",
    "request_photokit_permission",
    "register_photokit_permission_listener",
    "unregister_photokit_permission_listener",
    "get_photokit_permission_listeners",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();

    // 在 macOS 上链接 PhotoKit 框架
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Photos");
        println!("cargo:rustc-link-lib=framework=PhotosUI");
    }
}
