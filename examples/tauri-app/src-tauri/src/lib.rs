use log::{info, LevelFilter};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_macos_permissions::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                // 设置日志级别为 Debug，这样可以记录更多信息
                .level(LevelFilter::Debug)
                // 配置日志目标
                .targets([
                    // 输出到终端（默认启用）
                    Target::new(TargetKind::Stdout),
                    // 持久化到日志目录，使用自定义文件名
                    Target::new(TargetKind::LogDir {
                        file_name: Some("tauri-app".to_string()),
                    }),
                    // 输出到 webview 控制台
                    Target::new(TargetKind::Webview),
                ])
                // 设置最大文件大小为 10MB
                .max_file_size(10_000_000)
                // 配置日志轮转策略：保留所有旧日志文件
                .rotation_strategy(RotationStrategy::KeepAll)
                // 使用本地时区
                .timezone_strategy(TimezoneStrategy::UseLocal)
                // 自定义日志格式
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        record.target(),
                        record.level(),
                        message
                    ))
                })
                .build(),
        )
        .setup(|_app| {
            info!("🚀 Tauri 应用程序启动成功！");
            info!("📁 日志文件位置: ~/Library/Logs/com.tauri-app.app/");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
