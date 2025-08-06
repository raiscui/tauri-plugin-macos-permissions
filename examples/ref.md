在 **Tauri** 中访问 macOS 系统相册（Photos.app）时，权限处理分为两个层面：

---

### ✅ **1. Tauri 应用内权限配置（沙箱内）**

Tauri 默认运行在沙箱中，**无法直接访问 Photos.app 或 PhotoKit**，但你可以通过以下方式配置权限以支持文件交互：

#### 修改 `tauri.conf.json`（或 Tauri 2.x 的 `capabilities/*.json`）
```json
{
  "app": {
    "security": {
      "capabilities": [
        {
          "name": "photos-access",
          "permissions": [
            "dialog:allow-open",
            "fs:allow-read-file",
            "fs:allow-write-file",
            {
              "identifier": "fs:scope",
              "allow": ["$PICTURE/*", "$HOME/Desktop/*"]
            }
          ]
        }
      ]
    }
  }
}
```

> 说明：
> - `$PICTURE` 是 macOS 的「图片」目录（~/Pictures）
> - 如果你用 Swift FFI 或 AppleScript 间接操作相册，仍需确保 Tauri 能访问这些路径

---

### ✅ **2. macOS 系统级权限（Photos 访问授权）**

即使你配置了 Tauri 的文件权限，**首次访问 Photos.app 仍会被系统弹窗拦截**，需要用户授权。

#### ✅ 解决方法：在 `Info.plist` 中添加权限声明

编辑 `src-tauri/Info.plist`（或打包后自动生成的 `.app/Contents/Info.plist`）：

```xml
<key>NSPhotoLibraryUsageDescription</key>
<string>需要访问照片库以导入或管理照片</string>
```

> 这个字段是 macOS 强制要求的，**不添加会导致崩溃或无权限弹窗**。

---

### ✅ **3. 使用 Swift FFI 时的额外配置（推荐）**

如果你采用 [Swift FFI 方案](https://chat.z.ai/#/chat/9ab5f...)（Rust 调用 Swift 的 PhotoKit 接口）：

#### ✅ 必须在 `entitlements.plist` 中添加：
```xml
<key>com.apple.security.assets.pictures.read-write</key>
<true/>
```

#### ✅ 并在 `tauri.conf.json` 中指定：
```json
"bundle": {
  "macOS": {
    "entitlements": "src-tauri/entitlements.plist"
  }
}
```

---

### ✅ **4. 运行时权限弹窗处理**

- 首次调用 Swift FFI 或 AppleScript 时，macOS 会弹出：
  > “xxx 想要访问您的照片”
  用户必须点击「允许」。

- 若用户误点「不允许」，需手动在「系统设置 > 隐私与安全性 > 照片」中重新授权。

---

### ✅ **总结：完整权限链路**

| 层级           | 配置方式                                                                  | 是否必须      |
| -------------- | ------------------------------------------------------------------------- | ------------- |
| Tauri 沙箱     | `capabilities/*.json` 中配置 `fs`/`dialog` 权限                           | ✅             |
| macOS 系统权限 | `Info.plist` 添加 `NSPhotoLibraryUsageDescription`                        | ✅             |
| Swift FFI      | `entitlements.plist` 添加 `com.apple.security.assets.pictures.read-write` | ✅（仅限 FFI） |
| 用户授权       | 首次运行时弹窗授权                                                        | ✅             |

---
