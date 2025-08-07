# README 更新建议 - PhotoKit Permissions 功能

## 概述

本文档提供了为项目 README 添加 PhotoKit Permissions 功能说明的建议内容。

## 建议的 README 更新内容

### 在 Features 部分添加

```markdown
### PhotoKit Permissions (macOS)

- ✅ **权限状态检查**: 检查照片库访问权限状态
- ✅ **权限请求**: 触发系统权限对话框请求用户授权
- ✅ **实时监听**: 监听权限状态变化并实时通知前端
- ✅ **多权限级别**: 支持 Read、ReadWrite、AddOnly 三种权限级别
- ✅ **跨平台兼容**: macOS 完整功能，其他平台兼容性实现
```

### 在 API 部分添加

```markdown
## PhotoKit Permissions API

### 基础权限管理

```javascript
import { invoke } from '@tauri-apps/api/tauri';

// 检查权限状态
const status = await invoke('check_photokit_permission', {
    accessLevel: 'read' // 'read' | 'readWrite' | 'addOnly'
});

// 请求权限授权
const newStatus = await invoke('request_photokit_permission', {
    accessLevel: 'readWrite'
});
```

### 权限状态监听

```javascript
import { invoke, listen } from '@tauri-apps/api';

// 注册监听器
const listenerId = await invoke('register_photokit_permission_listener', {
    accessLevel: 'read'
});

// 监听权限状态变化
const unlisten = await listen('photokit-permission-changed', (event) => {
    const { new_status, access_level, timestamp } = event.payload;
    console.log(`权限 ${access_level} 变更为: ${new_status}`);
});

// 注销监听器
await invoke('unregister_photokit_permission_listener', {
    listenerId
});
```

### 权限状态说明

- `"notDetermined"`: 未确定，需要请求权限
- `"restricted"`: 受限制，通常由家长控制等限制
- `"denied"`: 已拒绝，用户明确拒绝了权限
- `"authorized"`: 已授权，可以完全访问照片库
- `"limited"`: 有限授权，只能访问用户选择的照片
```

### 在 Installation 部分添加

```markdown
### PhotoKit Permissions 配置

PhotoKit Permissions 功能需要在应用的 `Info.plist` 中添加相应的权限说明：

```xml
<key>NSPhotoLibraryUsageDescription</key>
<string>此应用需要访问您的照片库以便...</string>
<key>NSPhotoLibraryAddUsageDescription</key>
<string>此应用需要向您的照片库添加照片以便...</string>
```

确保权限说明清楚地解释了应用为什么需要这些权限。
```

### 在 Examples 部分添加

```markdown
### PhotoKit Permissions 示例

完整的 PhotoKit 权限管理示例：

```javascript
class PhotoKitManager {
    constructor() {
        this.setupPermissionListener();
    }

    async setupPermissionListener() {
        // 监听权限状态变化
        await listen('photokit-permission-changed', (event) => {
            this.handlePermissionChange(event.payload);
        });
    }

    async requestPhotoAccess() {
        try {
            // 检查当前权限状态
            let status = await invoke('check_photokit_permission', {
                accessLevel: 'readWrite'
            });

            if (status === 'notDetermined') {
                // 请求权限
                status = await invoke('request_photokit_permission', {
                    accessLevel: 'readWrite'
                });
            }

            return this.handlePermissionStatus(status);
        } catch (error) {
            console.error('权限请求失败:', error);
            return false;
        }
    }

    handlePermissionStatus(status) {
        switch (status) {
            case 'authorized':
                console.log('已获得完整照片库访问权限');
                return true;
            case 'limited':
                console.log('已获得有限照片库访问权限');
                return true;
            case 'denied':
                console.log('照片库访问权限被拒绝');
                this.showPermissionDeniedDialog();
                return false;
            case 'restricted':
                console.log('照片库访问权限受限');
                return false;
            default:
                console.log('未知权限状态:', status);
                return false;
        }
    }

    handlePermissionChange({ new_status, access_level, timestamp }) {
        console.log(`权限变化: ${access_level} -> ${new_status}`);
        // 更新 UI 状态
        this.updateUIForPermissionChange(new_status);
    }

    showPermissionDeniedDialog() {
        // 显示权限被拒绝的对话框，引导用户到设置
        alert('需要照片库访问权限才能使用此功能。请到系统设置中授权。');
    }

    updateUIForPermissionChange(newStatus) {
        // 根据权限状态更新用户界面
        const isAuthorized = newStatus === 'authorized' || newStatus === 'limited';
        document.getElementById('photo-features').style.display = 
            isAuthorized ? 'block' : 'none';
    }
}

// 使用示例
const photoManager = new PhotoKitManager();

document.getElementById('request-photo-permission').addEventListener('click', async () => {
    const hasPermission = await photoManager.requestPhotoAccess();
    if (hasPermission) {
        // 执行需要照片库权限的操作
        console.log('可以访问照片库了');
    }
});
```
```

### 在 Platform Support 部分添加

```markdown
### PhotoKit Permissions 平台支持

| 平台 | 权限检查 | 权限请求 | 状态监听 | 系统对话框 |
|------|----------|----------|----------|------------|
| macOS 10.15+ | ✅ | ✅ | ✅ | ✅ |
| Windows | ✅* | ✅* | ✅* | ❌ |
| Linux | ✅* | ✅* | ✅* | ❌ |

*兼容性实现：在非 macOS 平台上，API 调用不会出错，但会返回模拟的权限状态。
```

### 在 Troubleshooting 部分添加

```markdown
### PhotoKit Permissions 故障排除

#### 权限请求没有显示对话框

1. 确保在 `Info.plist` 中添加了正确的权限说明
2. 检查应用是否已经被用户永久拒绝权限
3. 在系统设置中检查应用的权限状态

#### 权限状态监听不工作

1. 确保正确注册了监听器
2. 检查事件监听器是否正确设置
3. 在应用退出时记得注销监听器

#### 在非 macOS 平台上的行为

在 Windows 和 Linux 上，PhotoKit API 会返回兼容性结果：
- 权限检查总是返回 `"authorized"`
- 权限请求立即返回 `"authorized"`
- 监听器可以正常注册但不会收到真实的权限变化事件

#### 常见错误信息

- `"平台不支持"`: 在不支持的平台上调用了特定功能
- `"框架不可用"`: PhotoKit 框架不可用，可能是系统版本过低
- `"监听器不存在"`: 尝试注销不存在的监听器
```

## 完整的功能列表更新

建议在 README 的功能列表中更新为：

```markdown
## Features

### 系统权限管理

- ✅ **辅助功能权限** (Accessibility)
- ✅ **完全磁盘访问权限** (Full Disk Access)  
- ✅ **屏幕录制权限** (Screen Recording)
- ✅ **麦克风权限** (Microphone)
- ✅ **摄像头权限** (Camera)
- ✅ **输入监控权限** (Input Monitoring)
- ✅ **PhotoKit 权限** (Photo Library) - 新增

### PhotoKit 权限特性

- 🔍 **多权限级别**: Read、ReadWrite、AddOnly
- 📱 **系统对话框**: 原生权限请求界面
- 🔄 **实时监听**: 权限状态变化通知
- 🌐 **跨平台兼容**: 统一的 API 接口
- 💾 **状态缓存**: 优化性能的缓存机制
- 🛡️ **类型安全**: TypeScript 类型定义
```

## 版本更新说明

建议在 CHANGELOG 中添加：

```markdown
## [2.3.0] - 2025-08-05

### Added
- PhotoKit 权限管理功能
  - 支持照片库权限状态检查
  - 支持照片库权限请求
  - 支持权限状态变化监听
  - 支持 Read、ReadWrite、AddOnly 三种权限级别
  - 跨平台兼容性实现

### New Commands
- `check_photokit_permission` - 检查 PhotoKit 权限状态
- `request_photokit_permission` - 请求 PhotoKit 权限授权
- `register_photokit_permission_listener` - 注册权限状态监听器
- `unregister_photokit_permission_listener` - 注销权限状态监听器
- `get_photokit_permission_listeners` - 获取活跃监听器列表

### New Events
- `photokit-permission-changed` - PhotoKit 权限状态变化事件
```

这些更新将帮助用户了解和使用新的 PhotoKit Permissions 功能。
