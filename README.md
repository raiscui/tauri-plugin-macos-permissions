# tauri-plugin-macos-permissions

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

Support for checking and requesting macOS system permissions, including traditional permissions and PhotoKit photo library access permissions.

https://github.com/user-attachments/assets/acb63744-9773-420a-8a96-6a485c94f5d6

## Install

```shell
cargo add tauri-plugin-macos-permissions
```

You can install the JavaScript Guest bindings using your preferred JavaScript package manager:

```shell
pnpm add tauri-plugin-macos-permissions-api
```

## Usage

`src-tauri/src/lib.rs`

```diff
pub fn run() {
    tauri::Builder::default()
+       .plugin(tauri_plugin_macos_permissions::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

`src-tauri/capabilities/default.json`

```diff
{
    ...
    "permissions": [
        ...
+       "macos-permissions:default"
    ]
}
```

If you need to access the microphone, camera, or PhotoKit permissions, please update `src-tauri/Info.plist`：

```diff
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
     ...
+    <key>NSMicrophoneUsageDescription</key>
+    <string>Describe why your app needs to use microphone permission</string>
+    <key>NSCameraUsageDescription</key>
+    <string>Describe why your app needs to use camera permissions</string>
+    <key>NSPhotoLibraryUsageDescription</key>
+    <string>Describe why your app needs to access the photo library</string>
</dict>
</plist>
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```ts
import {
  checkAccessibilityPermission,
  checkPhotoKitPermission,
  requestPhotoKitPermission
} from "tauri-plugin-macos-permissions-api";

// Check traditional permissions
const authorized = await checkAccessibilityPermission();
console.log(authorized); // true

// Check PhotoKit permissions
const photoStatus = await checkPhotoKitPermission('read');
console.log(photoStatus); // "authorized" | "denied" | "notDetermined" | ...

// Request PhotoKit permissions
const newStatus = await requestPhotoKitPermission('readWrite');
console.log(newStatus); // "authorized" | "denied" | ...
```

## Methods

### Traditional Permissions

| Method                             | Description                          |
| ---------------------------------- | ------------------------------------ |
| `checkAccessibilityPermission`     | Check accessibility permission.      |
| `requestAccessibilityPermission`   | Request accessibility permission.    |
| `checkFullDiskAccessPermission`    | Check full disk access permission.   |
| `requestFullDiskAccessPermission`  | Request full disk access permission. |
| `checkScreenRecordingPermission`   | Check screen recording permission.   |
| `requestScreenRecordingPermission` | Request screen recording permission. |
| `checkMicrophonePermission`        | Check microphone permission.         |
| `requestMicrophonePermission`      | Request microphone permission.       |
| `checkCameraPermission`            | Check camera permission.             |
| `requestCameraPermission`          | Request camera permission.           |
| `checkInputMonitoringPermission`   | Check input monitoring permission.   |
| `requestInputMonitoringPermission` | Request input monitoring permission. |

### PhotoKit Permissions

| Method                                             | Description                                             |
| -------------------------------------------------- | ------------------------------------------------------- |
| `checkPhotoKitPermission(accessLevel)`             | Check PhotoKit permission for specified access level.   |
| `requestPhotoKitPermission(accessLevel)`           | Request PhotoKit permission for specified access level. |
| `registerPhotoKitPermissionListener(accessLevel)`  | Register a listener for PhotoKit permission changes.    |
| `unregisterPhotoKitPermissionListener(listenerId)` | Unregister a PhotoKit permission listener.              |
| `getPhotoKitPermissionListeners()`                 | Get all active PhotoKit permission listeners.           |

#### PhotoKit Access Levels

- `'read'` - Read-only access to the photo library
- `'readWrite'` - Read and write access to the photo library
- `'addOnly'` - Add-only access to the photo library

#### PhotoKit Authorization Status

- `'notDetermined'` - Permission has not been requested yet
- `'restricted'` - Permission is restricted (e.g., by parental controls)
- `'denied'` - Permission has been denied by the user
- `'authorized'` - Permission has been granted
- `'limited'` - Limited access has been granted (iOS 14+ feature)

## Example

```shell
git clone https://github.com/ayangweb/tauri-plugin-macos-permissions.git
```

```shell
pnpm install

pnpm build

cd examples/tauri-app

pnpm install

# 注意：权限功能需要使用构建后的应用测试，开发模式无法正确处理系统权限
pnpm tauri build
open src-tauri/target/release/bundle/macos/tauri-app.app
```

## 故障排除

### Command not allowed by ACL 错误

如果遇到类似错误：
```
Command plugin:macos-permissions|[命令名] not allowed by ACL
```

请参考：
- [详细解决方案](docs/troubleshooting/acl-command-not-allowed.md)
- [快速修复指南](docs/quick-fixes/command-acl-error.md)

**关键要点**：
- 所有插件命令都必须在权限配置中声明
- 权限测试必须使用构建后的应用，不能使用 `pnpm tauri dev`

## Thanks

- Use [macos-accessibility-client](https://github.com/next-slide-please/macos-accessibility-client) to check and request accessibility permission.

- Use [FullDiskAccess](https://github.com/inket/FullDiskAccess/blob/846e04ea2b84fce843f47d7e7f3421189221829c/Sources/FullDiskAccess/FullDiskAccess.swift#L46) to check full disk access permission.

- Use [objc2](https://github.com/madsmtm/objc2) to check and request microphone, camera, and PhotoKit permissions.

## Who's Use It

- [EcoPaste](https://github.com/EcoPasteHub/EcoPaste) - Open source cross-platform clipboard management tool.

- [BongoCat](https://github.com/ayangweb/BongoCat) - Open source cross-platform desktop pets.

- [Coco AI](https://github.com/infinilabs/coco-app) - Search, Connect, Collaborate, Your Personal AI Search and Assistant, all in one space.
