const COMMANDS: &[&str] = &[
    "check_accessibility_permission",
    "request_accessibility_permission",
    "check_full_disk_access_permission",
    "request_full_disk_access_permission",
    "check_screen_recording_permission",
    "request_screen_recording_permission",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
