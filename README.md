# tauri-plugin-macos-permissions

> This plugin only works on tauri v2, if you need the v1 plugin, feel free to submit a PR!

Check and request macOS permissions to accessibility and full disk access.

https://github.com/user-attachments/assets/547a920c-29ef-4cd4-bba7-3e58c3f3bcd0

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

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```ts
import { checkAccessibilityPermissions } from "tauri-plugin-macos-permissions-api";

const authorized = await checkAccessibilityPermissions();
console.log(authorized); // true
```

## Methods

| Method                             | Description                           |
| ---------------------------------- | ------------------------------------- |
| `checkAccessibilityPermissions`    | Check Accessibility Permissions.      |
| `requestAccessibilityPermissions`  | Request Accessibility Permissions.    |
| `checkFullDiskAccessPermissions`   | Check Full Disk Access Permissions.   |
| `requestFullDiskAccessPermissions` | Request Full Disk Access Permissions. |

## Example

```shell
git clone https://github.com/ayangweb/tauri-plugin-macos-permissions.git
```

```shell
pnpm install

pnpm build

cd examples/tauri-app

pnpm install

pnpm tauri dev
```
