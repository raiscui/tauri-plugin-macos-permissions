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
 * Check Accessibility Permissions
 */
export const checkAccessibilityPermissions = () => {
  return invoke<boolean>(
    MACOS_PERMISSIONS_PLUGIN.CHECK_ACCESSIBILITY_PERMISSIONS
  );
};

/**
 * Request Accessibility Permissions
 */
export const requestAccessibilityPermissions = () => {
  return invoke<boolean>(
    MACOS_PERMISSIONS_PLUGIN.REQUEST_ACCESSIBILITY_PERMISSIONS
  );
};

/**
 * Check Full Disk Access Permissions
 */
export const checkFullDiskAccessPermissions = () => {
  return invoke<boolean>(
    MACOS_PERMISSIONS_PLUGIN.CHECK_FULL_DISK_ACCESS_PERMISSIONS
  );
};

/**
 * Request Full Disk Access Permissions
 */
export const requestFullDiskAccessPermissions = () => {
  return invoke(MACOS_PERMISSIONS_PLUGIN.REQUEST_FULL_DISK_ACCESS_PERMISSIONS);
};
