# PhotoKit Permissions API 文档

## 概述

PhotoKit Permissions 插件为 Tauri 应用提供了完整的 macOS PhotoKit 权限管理功能。该插件支持权限状态检查、权限请求和实时权限状态变化监听。

## 安装和配置

### 依赖添加

在 `Cargo.toml` 中添加插件依赖：

```toml
[dependencies]
tauri-plugin-macos-permissions = "2.3.0"
```

### 插件注册

在 Tauri 应用的 `main.rs` 中注册插件：

```rust
use tauri_plugin_macos_permissions;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_macos_permissions::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## 数据类型

### PhotoKitAccessLevel

PhotoKit 访问权限级别枚举：

```typescript
type PhotoKitAccessLevel = "read" | "readWrite" | "addOnly";
```

- `"read"`: 只读权限，可以读取照片库内容
- `"readWrite"`: 读写权限，可以读取和修改照片库内容
- `"addOnly"`: 仅添加权限，只能向照片库添加新内容

### PhotoKitAuthorizationStatus

PhotoKit 授权状态枚举：

```typescript
type PhotoKitAuthorizationStatus =
    | "notDetermined"  // 未确定
    | "restricted"     // 受限制
    | "denied"         // 已拒绝
    | "authorized"     // 已授权
    | "limited";       // 有限授权
```

### ListenerInfo

监听器信息结构：

```typescript
interface ListenerInfo {
    id: string;              // 监听器唯一标识符
    access_level: PhotoKitAccessLevel; // 监听的权限级别
    created_at: number;      // 创建时间戳（Unix 时间）
    active: boolean;         // 是否活跃
}
```

### PermissionStatusChangeEvent

权限状态变化事件：

```typescript
interface PermissionStatusChangeEvent {
    new_status: PhotoKitAuthorizationStatus; // 新的权限状态
    access_level: PhotoKitAccessLevel;       // 权限级别
    timestamp: number;                       // 变化时间戳
}
```

## API 命令

### check_photokit_permission

检查指定权限级别的当前授权状态。

**语法**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

const status = await invoke('check_photokit_permission', {
    accessLevel: PhotoKitAccessLevel
});
```

**参数**:
- `accessLevel`: PhotoKitAccessLevel - 要检查的权限级别

**返回值**: PhotoKitAuthorizationStatus - 当前的权限授权状态

**示例**:
```javascript
// 检查读取权限
const readStatus = await invoke('check_photokit_permission', {
    accessLevel: 'read'
});

console.log('读取权限状态:', readStatus);

if (readStatus === 'authorized') {
    console.log('已获得读取权限');
} else if (readStatus === 'notDetermined') {
    console.log('权限未确定，需要请求权限');
} else {
    console.log('权限被拒绝或受限');
}
```

### request_photokit_permission

请求指定权限级别的授权，会显示系统权限对话框。

**语法**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

const result = await invoke('request_photokit_permission', {
    accessLevel: PhotoKitAccessLevel
});
```

**参数**:
- `accessLevel`: PhotoKitAccessLevel - 要请求的权限级别

**返回值**: Result<PhotoKitAuthorizationStatus, string>
- 成功: PhotoKitAuthorizationStatus - 用户响应后的权限状态
- 失败: string - 错误信息

**示例**:
```javascript
try {
    const status = await invoke('request_photokit_permission', {
        accessLevel: 'readWrite'
    });

    if (status === 'authorized') {
        console.log('用户授予了读写权限');
        // 执行需要权限的操作
    } else {
        console.log('用户拒绝了权限请求，状态:', status);
        // 显示权限说明或引导用户到设置
    }
} catch (error) {
    console.error('请求权限时发生错误:', error);
}
```

### register_photokit_permission_listener

注册 PhotoKit 权限状态监听器，用于监听权限状态变化。

**语法**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

const listenerId = await invoke('register_photokit_permission_listener', {
    accessLevel: PhotoKitAccessLevel
});
```

**参数**:
- `accessLevel`: PhotoKitAccessLevel - 要监听的权限级别

**返回值**: Result<string, string>
- 成功: string - 监听器唯一标识符
- 失败: string - 错误信息

**示例**:
```javascript
try {
    const listenerId = await invoke('register_photokit_permission_listener', {
        accessLevel: 'read'
    });

    console.log('监听器注册成功，ID:', listenerId);

    // 保存监听器 ID 以便后续注销
    localStorage.setItem('photokit_listener_id', listenerId);
} catch (error) {
    console.error('注册监听器失败:', error);
}
```

### unregister_photokit_permission_listener

注销指定的 PhotoKit 权限状态监听器。

**语法**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

await invoke('unregister_photokit_permission_listener', {
    listenerId: string
});
```

**参数**:
- `listenerId`: string - 要注销的监听器标识符

**返回值**: Result<void, string>
- 成功: void
- 失败: string - 错误信息

**示例**:
```javascript
const listenerId = localStorage.getItem('photokit_listener_id');

if (listenerId) {
    try {
        await invoke('unregister_photokit_permission_listener', {
            listenerId: listenerId
        });

        console.log('监听器注销成功');
        localStorage.removeItem('photokit_listener_id');
    } catch (error) {
        console.error('注销监听器失败:', error);
    }
}
```

### get_photokit_permission_listeners

获取所有活跃的 PhotoKit 权限监听器信息。

**语法**:
```javascript
import { invoke } from '@tauri-apps/api/tauri';

const listeners = await invoke('get_photokit_permission_listeners');
```

**参数**: 无

**返回值**: Result<ListenerInfo[], string>
- 成功: ListenerInfo[] - 活跃监听器信息数组
- 失败: string - 错误信息

**示例**:
```javascript
try {
    const listeners = await invoke('get_photokit_permission_listeners');

    console.log(`当前有 ${listeners.length} 个活跃监听器:`);

    listeners.forEach(listener => {
        console.log(`- ID: ${listener.id}`);
        console.log(`  权限级别: ${listener.access_level}`);
        console.log(`  创建时间: ${new Date(listener.created_at * 1000)}`);
        console.log(`  状态: ${listener.active ? '活跃' : '非活跃'}`);
    });
} catch (error) {
    console.error('获取监听器列表失败:', error);
}
```

## 事件监听

### photokit-permission-changed

当 PhotoKit 权限状态发生变化时触发的事件。

**事件载荷**: PermissionStatusChangeEvent

**监听示例**:
```javascript
import { listen } from '@tauri-apps/api/event';

// 监听权限状态变化事件
const unlisten = await listen('photokit-permission-changed', (event) => {
    const { new_status, access_level, timestamp } = event.payload;

    console.log(`权限 ${access_level} 在 ${new Date(timestamp * 1000)} 变更为: ${new_status}`);

    // 根据权限变化更新应用状态
    handlePermissionChange(access_level, new_status);
});

function handlePermissionChange(accessLevel, newStatus) {
    switch (newStatus) {
        case 'authorized':
            console.log(`${accessLevel} 权限已授权，可以执行相关操作`);
            break;
        case 'denied':
            console.log(`${accessLevel} 权限被拒绝，需要引导用户到设置`);
            break;
        case 'limited':
            console.log(`${accessLevel} 权限受限，功能可能不完整`);
            break;
        default:
            console.log(`${accessLevel} 权限状态: ${newStatus}`);
    }
}

// 在组件卸载时取消监听
// unlisten();

## 完整使用示例

### 基础权限管理

```javascript
import { invoke } from '@tauri-apps/api/tauri';

class PhotoKitPermissionManager {
    async checkAndRequestPermission(accessLevel) {
        try {
            // 首先检查当前权限状态
            let status = await invoke('check_photokit_permission', {
                accessLevel
            });

            console.log(`当前 ${accessLevel} 权限状态:`, status);

            // 如果权限未确定，请求权限
            if (status === 'notDetermined') {
                console.log('权限未确定，正在请求权限...');

                status = await invoke('request_photokit_permission', {
                    accessLevel
                });

                console.log(`权限请求结果:`, status);
            }

            return status;
        } catch (error) {
            console.error('权限管理出错:', error);
            throw error;
        }
    }

    async hasPermission(accessLevel) {
        const status = await invoke('check_photokit_permission', {
            accessLevel
        });

        return status === 'authorized' || status === 'limited';
    }
}

// 使用示例
const permissionManager = new PhotoKitPermissionManager();

// 检查并请求读取权限
const readStatus = await permissionManager.checkAndRequestPermission('read');

if (await permissionManager.hasPermission('read')) {
    console.log('可以读取照片库');
} else {
    console.log('无法读取照片库');
}
```

### 权限状态监听管理

```javascript
import { invoke, listen } from '@tauri-apps/api';

class PhotoKitEventManager {
    constructor() {
        this.listeners = new Map(); // 存储监听器 ID
        this.eventUnlisten = null;  // 事件取消监听函数
        this.setupEventListening();
    }

    async setupEventListening() {
        // 设置事件监听
        this.eventUnlisten = await listen('photokit-permission-changed', (event) => {
            this.handlePermissionChange(event.payload);
        });
    }

    async startListening(accessLevel) {
        if (this.listeners.has(accessLevel)) {
            console.log(`已经在监听 ${accessLevel} 权限`);
            return;
        }

        try {
            const listenerId = await invoke('register_photokit_permission_listener', {
                accessLevel
            });

            this.listeners.set(accessLevel, listenerId);
            console.log(`开始监听 ${accessLevel} 权限，监听器 ID: ${listenerId}`);
        } catch (error) {
            console.error(`注册 ${accessLevel} 权限监听器失败:`, error);
        }
    }

    async stopListening(accessLevel) {
        const listenerId = this.listeners.get(accessLevel);

        if (!listenerId) {
            console.log(`没有在监听 ${accessLevel} 权限`);
            return;
        }

        try {
            await invoke('unregister_photokit_permission_listener', {
                listenerId
            });

            this.listeners.delete(accessLevel);
            console.log(`停止监听 ${accessLevel} 权限`);
        } catch (error) {
            console.error(`注销 ${accessLevel} 权限监听器失败:`, error);
        }
    }

    async stopAllListening() {
        for (const [accessLevel, listenerId] of this.listeners) {
            try {
                await invoke('unregister_photokit_permission_listener', {
                    listenerId
                });
                console.log(`停止监听 ${accessLevel} 权限`);
            } catch (error) {
                console.error(`注销 ${accessLevel} 权限监听器失败:`, error);
            }
        }

        this.listeners.clear();

        // 取消事件监听
        if (this.eventUnlisten) {
            this.eventUnlisten();
            this.eventUnlisten = null;
        }
    }

    handlePermissionChange(event) {
        const { new_status, access_level, timestamp } = event;

        console.log(`权限变化通知:`);
        console.log(`- 权限级别: ${access_level}`);
        console.log(`- 新状态: ${new_status}`);
        console.log(`- 时间: ${new Date(timestamp * 1000)}`);

        // 触发自定义事件或回调
        this.onPermissionChange(access_level, new_status, timestamp);
    }

    onPermissionChange(accessLevel, newStatus, timestamp) {
        // 子类可以重写此方法来处理权限变化
        console.log(`权限 ${accessLevel} 变更为 ${newStatus}`);
    }

    async getActiveListeners() {
        try {
            const listeners = await invoke('get_photokit_permission_listeners');
            console.log('活跃的监听器:', listeners);
            return listeners;
        } catch (error) {
            console.error('获取监听器列表失败:', error);
            return [];
        }
    }
}

// 使用示例
const eventManager = new PhotoKitEventManager();

// 开始监听读取权限
await eventManager.startListening('read');

// 开始监听读写权限
await eventManager.startListening('readWrite');

// 查看活跃监听器
await eventManager.getActiveListeners();

// 应用退出时清理
window.addEventListener('beforeunload', async () => {
    await eventManager.stopAllListening();
});
```

## 错误处理

### 常见错误类型

1. **平台不支持**: 在非 macOS 平台上使用
2. **权限请求失败**: 系统权限请求过程中出错
3. **监听器不存在**: 尝试注销不存在的监听器
4. **框架不可用**: PhotoKit 框架不可用

### 错误处理最佳实践

```javascript
async function safePermissionOperation(operation) {
    try {
        return await operation();
    } catch (error) {
        console.error('权限操作失败:', error);

        // 根据错误类型进行处理
        if (error.includes('平台不支持')) {
            console.log('当前平台不支持 PhotoKit 权限管理');
            return null;
        } else if (error.includes('框架不可用')) {
            console.log('PhotoKit 框架不可用，可能是系统版本过低');
            return null;
        } else {
            // 其他错误，重新抛出
            throw error;
        }
    }
}

// 使用示例
const status = await safePermissionOperation(async () => {
    return await invoke('check_photokit_permission', {
        accessLevel: 'read'
    });
});

if (status !== null) {
    console.log('权限状态:', status);
} else {
    console.log('无法获取权限状态');
}
```

## 平台兼容性

### macOS
- ✅ 完整功能支持
- ✅ 真实的 PhotoKit 权限检查和请求
- ✅ 系统权限对话框
- ⚠️ 权限状态变化监听（基础实现）

### 其他平台
- ✅ API 兼容性
- ✅ 模拟权限状态（总是返回 "authorized"）
- ✅ 模拟监听器管理
- ❌ 无实际权限检查

## 最佳实践

### 1. 权限检查时机
- 应用启动时检查必要权限
- 执行需要权限的操作前检查
- 用户主动触发权限相关功能时检查

### 2. 用户体验
- 在请求权限前向用户说明原因
- 提供权限被拒绝时的替代方案
- 引导用户到系统设置修改权限

### 3. 错误处理
- 始终使用 try-catch 包装权限操作
- 提供用户友好的错误提示
- 记录详细的错误日志用于调试

### 4. 资源管理
- 及时注销不需要的监听器
- 在应用退出时清理所有监听器
- 避免重复注册相同的监听器

## 版本兼容性

- **Tauri**: 2.x
- **macOS**: 10.15+ (Catalina 及以上)
- **Rust**: 1.77.2+

## 相关链接

- [Apple PhotoKit 文档](https://developer.apple.com/documentation/photokit)
- [Tauri 插件开发指南](https://tauri.app/v1/guides/building/plugins)
- [项目 GitHub 仓库](https://github.com/ayangweb/tauri-plugin-macos-permissions)
