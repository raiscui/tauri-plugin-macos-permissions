import { invoke } from "@tauri-apps/api/core";

export const MACOS_PERMISSIONS_PLUGIN = {
  CHECK_ACCESSIBILITY_PERMISSIONS:
    "plugin:macos-permissions|check_accessibility_permissions",
  REQUEST_ACCESSIBILITY_PERMISSIONS:
    "plugin:macos-permissions|request_accessibility_permissions",
  CHECK_FULL_DISK_ACCESS_PERMISSIONS:
    "plugin:macos-permissions|check_full_disk_access_permissions",
  REQUEST_FULL_DISK_ACCESS_PERMISSIONS:
    "plugin:macos-permissions|request_full_disk_access_permissions",
};

/**
 * Check Accessibility Permissions.
 * @example
 * import { checkAccessibilityPermissions } from "tauri-plugin-macos-permissions-api";
 * const authorized = await checkAccessibilityPermissions();
 * console.log(authorized); // false
 */
export const checkAccessibilityPermissions = () => {
  return invoke<boolean>(
    MACOS_PERMISSIONS_PLUGIN.CHECK_ACCESSIBILITY_PERMISSIONS
  );
};

/**
 * Request Accessibility Permissions.
 * @example
 * import { requestAccessibilityPermissions } from "tauri-plugin-macos-permissions-api";
 * const authorized = await requestAccessibilityPermissions();
 * console.log(authorized); // false
 */
export const requestAccessibilityPermissions = () => {
  return invoke<boolean>(
    MACOS_PERMISSIONS_PLUGIN.REQUEST_ACCESSIBILITY_PERMISSIONS
  );
};

/**
 * Check Full Disk Access Permissions.
 * @example
 * import { checkFullDiskAccessPermissions } from "tauri-plugin-macos-permissions-api";
 * const authorized = await checkFullDiskAccessPermissions();
 * console.log(authorized); // false
 */
export const checkFullDiskAccessPermissions = () => {
  return invoke<boolean>(
    MACOS_PERMISSIONS_PLUGIN.CHECK_FULL_DISK_ACCESS_PERMISSIONS
  );
};

/**
 * Request Full Disk Access Permissions.
 * @example
 * import { requestFullDiskAccessPermissions } from "tauri-plugin-macos-permissions-api";
 * await requestFullDiskAccessPermissions();
 */
export const requestFullDiskAccessPermissions = () => {
  return invoke(MACOS_PERMISSIONS_PLUGIN.REQUEST_FULL_DISK_ACCESS_PERMISSIONS);
};
