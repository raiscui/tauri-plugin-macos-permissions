// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        })
        .plugin(tauri_plugin_macos_permissions::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
