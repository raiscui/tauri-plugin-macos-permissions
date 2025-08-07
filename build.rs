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
    "get_photos_count",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();

    // 测试：PhotoKit可能不需要显式链接，就像AVFoundation一样
    // 原项目没有链接任何框架，AVFoundation和CoreGraphics都是自动链接的
    // 让我们测试PhotoKit是否也是自动链接的
    #[cfg(target_os = "macos")]
    {
        // 暂时注释掉显式链接，测试是否需要
        // println!("cargo:rustc-link-lib=framework=Photos");
        // println!("cargo:rustc-link-lib=framework=PhotosUI");
        println!("cargo:warning=测试PhotoKit自动链接");
    }
}
