# PhotoKit 权限功能 API 规格

## API 概述
本文档定义了 PhotoKit 权限功能的完整 API 规格，包括所有公共函数、数据类型和错误处理机制。

## 数据类型定义

### PhotoKitAccessLevel 枚举
```rust
/// PhotoKit 访问权限级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhotoKitAccessLevel {
    /// 只读权限 - 可以读取照片和视频
    #[serde(rename = "read")]
    Read,
    /// 读写权限 - 可以读取、修改和删除照片和视频
    #[serde(rename = "readWrite")]
    ReadWrite,
    /// 添加权限 - 只能添加新的照片和视频
    #[serde(rename = "addOnly")]
    AddOnly,
}
```

### PhotoKitAuthorizationStatus 枚举
```rust
/// PhotoKit 授权状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhotoKitAuthorizationStatus {
    /// 未确定 - 用户尚未做出选择
    #[serde(rename = "notDetermined")]
    NotDetermined,
    /// 受限 - 由于家长控制等原因被限制
    #[serde(rename = "restricted")]
    Restricted,
    /// 已拒绝 - 用户明确拒绝了权限
    #[serde(rename = "denied")]
    Denied,
    /// 已授权 - 用户已授予完整权限
    #[serde(rename = "authorized")]
    Authorized,
    /// 有限访问 - 用户选择了部分照片访问 (iOS 14+)
    #[serde(rename = "limited")]
    Limited,
}
```

### PermissionStatusChangeEvent 结构体
```rust
/// 权限状态变化事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionStatusChangeEvent {
    /// 新的权限状态
    pub new_status: PhotoKitAuthorizationStatus,
    /// 权限级别
    pub access_level: PhotoKitAccessLevel,
    /// 变化时间戳 (Unix 时间戳，毫秒)
    pub timestamp: u64,
    /// 事件 ID
    pub event_id: String,
}
```

## 核心 API 函数

### 1. 检查权限状态

#### `check_photokit_permission`
检查指定权限级别的当前授权状态。

```rust
#[command]
pub async fn check_photokit_permission(
    access_level: PhotoKitAccessLevel
) -> PhotoKitAuthorizationStatus
```

**参数**:
- `access_level`: 要检查的权限级别

**返回值**:
- `PhotoKitAuthorizationStatus`: 当前权限状态

**示例**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

// 检查读取权限
const status = await invoke('check_photokit_permission', {
    accessLevel: 'read'
});
console.log('权限状态:', status); // "authorized" | "denied" | "notDetermined" | ...
```

**平台行为**:
- **macOS**: 返回实际的 PhotoKit 权限状态
- **其他平台**: 始终返回 `Authorized`

---

### 2. 请求权限

#### `request_photokit_permission`
请求指定权限级别的授权，会显示系统权限对话框。

```rust
#[command]
pub async fn request_photokit_permission(
    access_level: PhotoKitAccessLevel
) -> Result<PhotoKitAuthorizationStatus, String>
```

**参数**:
- `access_level`: 要请求的权限级别

**返回值**:
- `Ok(PhotoKitAuthorizationStatus)`: 用户响应后的权限状态
- `Err(String)`: 错误信息

**示例**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

try {
    const status = await invoke('request_photokit_permission', {
        accessLevel: 'readWrite'
    });
    
    if (status === 'authorized') {
        console.log('权限已授予');
    } else {
        console.log('权限被拒绝或受限:', status);
    }
} catch (error) {
    console.error('请求权限失败:', error);
}
```

**错误情况**:
- 系统不支持 PhotoKit (macOS < 10.15)
- 权限对话框显示失败
- 内部系统错误

---

### 3. 注册权限状态监听器

#### `register_photokit_permission_listener`
注册权限状态变化监听器，当用户在系统设置中修改权限时会收到通知。

```rust
#[command]
pub async fn register_photokit_permission_listener(
    access_level: PhotoKitAccessLevel
) -> Result<String, String>
```

**参数**:
- `access_level`: 要监听的权限级别

**返回值**:
- `Ok(String)`: 监听器 ID，用于后续注销
- `Err(String)`: 错误信息

**示例**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

try {
    // 注册监听器
    const listenerId = await invoke('register_photokit_permission_listener', {
        accessLevel: 'read'
    });
    
    // 监听权限状态变化事件
    const unlisten = await listen('photokit-permission-changed', (event) => {
        const changeEvent = event.payload;
        console.log('权限状态变化:', changeEvent);
        
        if (changeEvent.new_status === 'denied') {
            // 处理权限被拒绝的情况
            showPermissionDeniedDialog();
        }
    });
    
    // 保存监听器 ID 和取消监听函数
    window.photokitListenerId = listenerId;
    window.photokitUnlisten = unlisten;
    
} catch (error) {
    console.error('注册监听器失败:', error);
}
```

---

### 4. 注销权限状态监听器

#### `unregister_photokit_permission_listener`
注销之前注册的权限状态监听器。

```rust
#[command]
pub async fn unregister_photokit_permission_listener(
    listener_id: String
) -> Result<(), String>
```

**参数**:
- `listener_id`: 注册时返回的监听器 ID

**返回值**:
- `Ok(())`: 成功注销
- `Err(String)`: 错误信息

**示例**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

try {
    // 注销监听器
    await invoke('unregister_photokit_permission_listener', {
        listenerId: window.photokitListenerId
    });
    
    // 取消事件监听
    if (window.photokitUnlisten) {
        window.photokitUnlisten();
    }
    
    console.log('监听器已注销');
} catch (error) {
    console.error('注销监听器失败:', error);
}
```

## 事件系统

### 权限状态变化事件
当权限状态发生变化时，会触发 `photokit-permission-changed` 事件。

**事件名称**: `photokit-permission-changed`

**事件数据**: `PermissionStatusChangeEvent`

**触发时机**:
- 用户在系统设置中修改应用权限
- 权限从未确定变为其他状态
- 权限从授权变为拒绝或受限

## 错误处理

### 错误类型
```rust
#[derive(Debug, thiserror::Error)]
pub enum PhotoKitError {
    #[error("PhotoKit 不可用 (需要 macOS 10.15+)")]
    PhotoKitUnavailable,
    
    #[error("权限请求失败: {0}")]
    PermissionRequestFailed(String),
    
    #[error("监听器注册失败: {0}")]
    ListenerRegistrationFailed(String),
    
    #[error("监听器未找到: {0}")]
    ListenerNotFound(String),
    
    #[error("内部错误: {0}")]
    InternalError(String),
}
```

### 错误码映射
| 错误类型 | 错误码 | 描述 | 建议处理 |
|----------|--------|------|----------|
| PhotoKitUnavailable | PK001 | 系统不支持 PhotoKit | 提示用户升级系统 |
| PermissionRequestFailed | PK002 | 权限请求失败 | 重试或引导用户手动设置 |
| ListenerRegistrationFailed | PK003 | 监听器注册失败 | 降级为轮询检查 |
| ListenerNotFound | PK004 | 监听器不存在 | 检查监听器 ID 有效性 |
| InternalError | PK005 | 内部系统错误 | 记录日志并报告问题 |

## 使用模式和最佳实践

### 1. 基本权限检查流程
```javascript
async function checkPhotoPermission() {
    const status = await invoke('check_photokit_permission', {
        accessLevel: 'read'
    });
    
    switch (status) {
        case 'authorized':
            // 可以访问照片
            return true;
        case 'notDetermined':
            // 需要请求权限
            return await requestPhotoPermission();
        case 'denied':
        case 'restricted':
            // 权限被拒绝，引导用户到设置
            showPermissionDeniedDialog();
            return false;
        case 'limited':
            // 有限访问，可以使用但功能受限
            showLimitedAccessDialog();
            return true;
    }
}
```

### 2. 权限请求最佳实践
```javascript
async function requestPhotoPermission() {
    try {
        const status = await invoke('request_photokit_permission', {
            accessLevel: 'readWrite'
        });
        
        if (status === 'authorized') {
            return true;
        } else {
            // 用户拒绝了权限，提供替代方案
            showAlternativeOptions();
            return false;
        }
    } catch (error) {
        console.error('权限请求失败:', error);
        return false;
    }
}
```

### 3. 权限状态监听
```javascript
class PhotoPermissionManager {
    constructor() {
        this.listenerId = null;
        this.unlisten = null;
    }
    
    async startListening() {
        try {
            this.listenerId = await invoke('register_photokit_permission_listener', {
                accessLevel: 'read'
            });
            
            this.unlisten = await listen('photokit-permission-changed', (event) => {
                this.handlePermissionChange(event.payload);
            });
        } catch (error) {
            console.error('启动权限监听失败:', error);
        }
    }
    
    async stopListening() {
        if (this.listenerId) {
            try {
                await invoke('unregister_photokit_permission_listener', {
                    listenerId: this.listenerId
                });
            } catch (error) {
                console.error('停止权限监听失败:', error);
            }
        }
        
        if (this.unlisten) {
            this.unlisten();
        }
    }
    
    handlePermissionChange(event) {
        console.log('权限状态变化:', event);
        // 根据新状态更新 UI
        this.updateUI(event.new_status);
    }
}
```

## 版本兼容性

### API 版本
- **当前版本**: 1.0.0
- **最低支持**: macOS 10.15 (Catalina)
- **推荐版本**: macOS 11.0+ (Big Sur)

### 向后兼容性
- API 设计遵循语义化版本控制
- 主版本号变更时可能包含破坏性变更
- 次版本号变更保证向后兼容
- 补丁版本仅包含错误修复
