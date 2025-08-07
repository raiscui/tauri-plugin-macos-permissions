# TASK-006 完成报告: 监听器管理命令

## 任务概述

**任务ID**: TASK-006  
**任务名称**: 监听器管理命令  
**完成日期**: 2025-08-05  
**开发者**: Claude (Augment Agent)  
**状态**: ✅ 已完成  

## 实施内容

### 1. Tauri 命令实现

在 `src/commands.rs` 中添加了监听器管理相关的 Tauri 命令函数。

### 2. 核心命令

#### register_photokit_permission_listener
```rust
#[command]
pub async fn register_photokit_permission_listener<R: Runtime>(
    app_handle: AppHandle<R>,
    access_level: PhotoKitAccessLevel,
) -> Result<String, String>
```

**功能**: 注册 PhotoKit 权限状态监听器  
**参数**: 
- `app_handle` - Tauri 应用句柄
- `access_level` - 要监听的权限级别  
**返回**: 监听器 ID 或错误信息  

#### unregister_photokit_permission_listener
```rust
#[command]
pub async fn unregister_photokit_permission_listener<R: Runtime>(
    app_handle: AppHandle<R>,
    listener_id: String,
) -> Result<(), String>
```

**功能**: 注销 PhotoKit 权限状态监听器  
**参数**:
- `app_handle` - Tauri 应用句柄
- `listener_id` - 要注销的监听器 ID  
**返回**: 成功或错误信息  

#### get_photokit_permission_listeners
```rust
#[command]
pub async fn get_photokit_permission_listeners<R: Runtime>(
    app_handle: AppHandle<R>,
) -> Result<Vec<ListenerInfo>, String>
```

**功能**: 获取所有活跃的权限监听器  
**参数**: `app_handle` - Tauri 应用句柄  
**返回**: 活跃监听器列表或错误信息  

### 3. JavaScript 集成

#### 注册监听器示例
```javascript
import { invoke, listen } from '@tauri-apps/api';

// 注册监听器
const listenerId = await invoke('register_photokit_permission_listener', {
    accessLevel: 'read'
});

// 监听权限状态变化事件
const unlisten = await listen('photokit-permission-changed', (event) => {
    console.log('权限状态变化:', event.payload);
});

// 稍后注销监听器
await invoke('unregister_photokit_permission_listener', {
    listenerId: listenerId
});
```

#### 监听器管理示例
```javascript
import { invoke } from '@tauri-apps/api';

// 获取所有活跃监听器
const listeners = await invoke('get_photokit_permission_listeners');
console.log('活跃的监听器:', listeners);

// 注销特定监听器
for (const listener of listeners) {
    if (listener.access_level === 'read') {
        await invoke('unregister_photokit_permission_listener', {
            listenerId: listener.id
        });
    }
}
```

### 4. 插件注册

在 `src/lib.rs` 的插件初始化函数中注册了新命令：

```rust
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("macos-permissions")
        .invoke_handler(generate_handler![
            // ... 其他命令
            commands::register_photokit_permission_listener,
            commands::unregister_photokit_permission_listener,
            commands::get_photokit_permission_listeners,
        ])
        .build()
}
```

## 验收标准完成情况

- [x] **register_photokit_permission_listener 命令**: 实现监听器注册命令
- [x] **unregister_photokit_permission_listener 命令**: 实现监听器注销命令
- [x] **监听器 ID 生成和验证**: 使用 UUID 生成唯一 ID，验证 ID 存在性
- [x] **错误处理和状态检查**: 完整的错误处理和状态验证
- [x] **资源清理机制**: 自动清理注销的监听器资源

## 技术特性

### ID 管理
- 使用 UUID v4 生成唯一监听器标识符
- 自动验证监听器 ID 的有效性
- 防止重复注册和无效注销

### 错误处理
- 统一的错误处理模式
- 详细的错误信息反馈
- 用户友好的错误提示

### 跨平台支持
- **macOS**: 完整的监听器功能
- **其他平台**: 兼容性实现，提供相同的 API 接口

### 异步支持
- 所有命令都是异步的
- 支持并发的监听器操作
- 非阻塞的用户界面集成

## API 文档

### register_photokit_permission_listener

**描述**: 注册 PhotoKit 权限状态监听器  
**参数**:
- `access_level`: PhotoKitAccessLevel - 权限级别 ("read" | "readWrite" | "addOnly")

**返回值**: Result<String, String>
- 成功: 监听器 ID (UUID 字符串)
- 失败: 错误信息字符串

**事件**: 注册成功后，权限状态变化时会触发 `photokit-permission-changed` 事件

### unregister_photokit_permission_listener

**描述**: 注销 PhotoKit 权限状态监听器  
**参数**:
- `listener_id`: String - 监听器 ID

**返回值**: Result<(), String>
- 成功: 空值
- 失败: 错误信息字符串

### get_photokit_permission_listeners

**描述**: 获取所有活跃的权限监听器信息  
**参数**: 无

**返回值**: Result<Vec<ListenerInfo>, String>
- 成功: 监听器信息数组
- 失败: 错误信息字符串

**ListenerInfo 结构**:
```typescript
interface ListenerInfo {
    id: string;              // 监听器 ID
    access_level: string;    // 权限级别
    created_at: number;      // 创建时间戳
    active: boolean;         // 是否活跃
}
```

## 使用场景

### 基础监听器管理
```javascript
// 注册读取权限监听器
const readListenerId = await invoke('register_photokit_permission_listener', {
    accessLevel: 'read'
});

// 注册读写权限监听器
const writeListenerId = await invoke('register_photokit_permission_listener', {
    accessLevel: 'readWrite'
});

// 查看所有监听器
const listeners = await invoke('get_photokit_permission_listeners');
console.log(`当前有 ${listeners.length} 个活跃监听器`);
```

### 监听器生命周期管理
```javascript
class PhotoKitPermissionManager {
    constructor() {
        this.listeners = new Map();
    }

    async startListening(accessLevel) {
        if (this.listeners.has(accessLevel)) {
            return; // 已经在监听
        }

        const listenerId = await invoke('register_photokit_permission_listener', {
            accessLevel
        });
        
        this.listeners.set(accessLevel, listenerId);
    }

    async stopListening(accessLevel) {
        const listenerId = this.listeners.get(accessLevel);
        if (!listenerId) {
            return; // 没有在监听
        }

        await invoke('unregister_photokit_permission_listener', {
            listenerId
        });
        
        this.listeners.delete(accessLevel);
    }

    async stopAllListening() {
        for (const [accessLevel, listenerId] of this.listeners) {
            await invoke('unregister_photokit_permission_listener', {
                listenerId
            });
        }
        this.listeners.clear();
    }
}
```

### 事件处理集成
```javascript
import { listen } from '@tauri-apps/api/event';

class PhotoKitEventHandler {
    constructor() {
        this.setupEventListening();
    }

    async setupEventListening() {
        // 监听权限状态变化
        await listen('photokit-permission-changed', (event) => {
            const { new_status, access_level, timestamp } = event.payload;
            this.handlePermissionChange(access_level, new_status, timestamp);
        });
    }

    handlePermissionChange(accessLevel, newStatus, timestamp) {
        console.log(`权限 ${accessLevel} 在 ${new Date(timestamp * 1000)} 变更为: ${newStatus}`);
        
        // 根据权限变化更新 UI
        this.updateUI(accessLevel, newStatus);
    }

    updateUI(accessLevel, status) {
        // 更新用户界面逻辑
    }
}
```

## 依赖关系

- **依赖任务**: TASK-005 (权限状态监听系统) ✅ 已完成
- **被依赖任务**: TASK-007 (示例应用集成)

## 文件清单

- `src/commands.rs`: 命令实现（新增内容）
- `src/lib.rs`: 插件注册（更新内容）

## 总结

TASK-006 已成功完成，实现了完整的 PhotoKit 权限监听器管理命令接口。这些命令提供了简洁易用的 JavaScript API，支持监听器的注册、注销和查询功能。实现具有良好的错误处理、ID 管理和跨平台兼容性，为前端应用提供了强大的权限状态监听能力。结合事件系统，应用可以实时响应权限状态变化，提供更好的用户体验。
