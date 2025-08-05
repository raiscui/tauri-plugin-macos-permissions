import { invoke } from "@tauri-apps/api/core";

export const COMMAND = {
    CHECK_ACCESSIBILITY_PERMISSION:
        "plugin:macos-permissions|check_accessibility_permission",
    REQUEST_ACCESSIBILITY_PERMISSION:
        "plugin:macos-permissions|request_accessibility_permission",
    CHECK_FULL_DISK_ACCESS_PERMISSION:
        "plugin:macos-permissions|check_full_disk_access_permission",
    REQUEST_FULL_DISK_ACCESS_PERMISSION:
        "plugin:macos-permissions|request_full_disk_access_permission",
    CHECK_SCREEN_RECORDING_PERMISSION:
        "plugin:macos-permissions|check_screen_recording_permission",
    REQUEST_SCREEN_RECORDING_PERMISSION:
        "plugin:macos-permissions|request_screen_recording_permission",
    CHECK_MICROPHONE_PERMISSION:
        "plugin:macos-permissions|check_microphone_permission",
    REQUEST_MICROPHONE_PERMISSION:
        "plugin:macos-permissions|request_microphone_permission",
    CHECK_CAMERA_PERMISSION: "plugin:macos-permissions|check_camera_permission",
    REQUEST_CAMERA_PERMISSION:
        "plugin:macos-permissions|request_camera_permission",
    CHECK_INPUT_MONITORING_PERMISSION:
        "plugin:macos-permissions|check_input_monitoring_permission",
    REQUEST_INPUT_MONITORING_PERMISSION:
        "plugin:macos-permissions|request_input_monitoring_permission",
    CHECK_PHOTOKIT_PERMISSION:
        "plugin:macos-permissions|check_photokit_permission",
    REQUEST_PHOTOKIT_PERMISSION:
        "plugin:macos-permissions|request_photokit_permission",
    REGISTER_PHOTOKIT_PERMISSION_LISTENER:
        "plugin:macos-permissions|register_photokit_permission_listener",
    UNREGISTER_PHOTOKIT_PERMISSION_LISTENER:
        "plugin:macos-permissions|unregister_photokit_permission_listener",
    GET_PHOTOKIT_PERMISSION_LISTENERS:
        "plugin:macos-permissions|get_photokit_permission_listeners",
};

/**
 * Check accessibility permission.
 *
 * @returns `true` if accessibility permission are granted, `false` otherwise.
 *
 * @example
 * import { checkAccessibilityPermission } from "tauri-plugin-macos-permissions-api";
 *
 * const authorized = await checkAccessibilityPermission();
 * console.log(authorized); // false
 */
export const checkAccessibilityPermission = () => {
    return invoke<boolean>(COMMAND.CHECK_ACCESSIBILITY_PERMISSION);
};

/**
 * Request accessibility permission.
 *
 * @example
 * import { requestAccessibilityPermission } from "tauri-plugin-macos-permissions-api";
 *
 * await requestAccessibilityPermission();
 */
export const requestAccessibilityPermission = () => {
    return invoke(COMMAND.REQUEST_ACCESSIBILITY_PERMISSION);
};

/**
 * Check full disk access permission.
 *
 * @returns `true` if full disk access permission are granted, `false` otherwise.
 *
 * @example
 * import { checkFullDiskAccessPermission } from "tauri-plugin-macos-permissions-api";
 *
 * const authorized = await checkFullDiskAccessPermission();
 * console.log(authorized); // false
 */
export const checkFullDiskAccessPermission = () => {
    return invoke<boolean>(COMMAND.CHECK_FULL_DISK_ACCESS_PERMISSION);
};

/**
 * Request full disk access permission.
 *
 * @example
 * import { requestFullDiskAccessPermission } from "tauri-plugin-macos-permission-api";
 *
 * await requestFullDiskAccessPermission();
 */
export const requestFullDiskAccessPermission = () => {
    return invoke(COMMAND.REQUEST_FULL_DISK_ACCESS_PERMISSION);
};

/**
 * Check screen recording permission.
 *
 * @returns `true` if screen recording permission are granted, `false` otherwise.
 *
 * @example
 * import { checkScreenRecordingPermission } from "tauri-plugin-macos-permissions-api";
 *
 * const authorized = await checkScreenRecordingPermission();
 * console.log(authorized); // false
 */
export const checkScreenRecordingPermission = () => {
    return invoke<boolean>(COMMAND.CHECK_SCREEN_RECORDING_PERMISSION);
};

/**
 * Request screen recording permission.
 *
 * @example
 * import { requestScreenRecordingPermission } from "tauri-plugin-macos-permissions-api";
 *
 * await requestScreenRecordingPermission();
 */
export const requestScreenRecordingPermission = () => {
    return invoke(COMMAND.REQUEST_SCREEN_RECORDING_PERMISSION);
};

/**
 * Check microphone permission.
 *
 * @returns `true` if microphone permission are granted, `false` otherwise.
 *
 * @example
 * import { checkMicrophonePermission } from "tauri-plugin-macos-permissions-api";
 *
 * const authorized = await checkMicrophonePermission();
 * console.log(authorized); // false
 */
export const checkMicrophonePermission = () => {
    return invoke<boolean>(COMMAND.CHECK_MICROPHONE_PERMISSION);
};

/**
 * Request microphone permission.
 *
 * @example
 * import { requestMicrophonePermission } from "tauri-plugin-macos-permissions-api";
 *
 * await requestMicrophonePermission();
 */
export const requestMicrophonePermission = () => {
    return invoke(COMMAND.REQUEST_MICROPHONE_PERMISSION);
};

/**
 * Check camera permission.
 *
 * @returns `true` if camera permission are granted, `false` otherwise.
 *
 * @example
 * import { checkCameraPermission } from "tauri-plugin-macos-permissions-api";
 *
 * const authorized = await checkCameraPermission();
 * console.log(authorized); // false
 */
export const checkCameraPermission = () => {
    return invoke<boolean>(COMMAND.CHECK_CAMERA_PERMISSION);
};

/**
 * Request camera permission.
 *
 * @example
 * import { requestCameraPermission } from "tauri-plugin-macos-permissions-api";
 *
 * await requestCameraPermission();
 */
export const requestCameraPermission = () => {
    return invoke(COMMAND.REQUEST_CAMERA_PERMISSION);
};

/**
 * Check input monitoring permission.
 *
 * @returns `true` if input monitoring permission are granted, `false` otherwise.
 *
 * @example
 * import { checkInputMonitoringPermission } from "tauri-plugin-macos-permissions-api";
 *
 * const authorized = await checkInputMonitoringPermission();
 * console.log(authorized); // false
 */
export const checkInputMonitoringPermission = () => {
    return invoke<boolean>(COMMAND.CHECK_INPUT_MONITORING_PERMISSION);
};

/**
 * Request input monitoring permission.
 *
 * @example
 * import { requestInputMonitoringPermission } from "tauri-plugin-macos-permissions-api";
 *
 * await requestInputMonitoringPermission();
 */
export const requestInputMonitoringPermission = () => {
    return invoke(COMMAND.REQUEST_INPUT_MONITORING_PERMISSION);
};

/**
 * PhotoKit access levels for photo library permissions.
 */
export type PhotoKitAccessLevel = "read" | "readWrite" | "addOnly";

/**
 * PhotoKit authorization status values.
 */
export type PhotoKitAuthorizationStatus =
    | "notDetermined"
    | "restricted"
    | "denied"
    | "authorized"
    | "limited";

/**
 * PhotoKit permission listener information.
 */
export interface ListenerInfo {
    id: string;
    access_level: PhotoKitAccessLevel;
    created_at: number;
    active: boolean;
}

/**
 * Check PhotoKit permission for the specified access level.
 *
 * @param accessLevel - The PhotoKit access level to check ('read' | 'readWrite' | 'addOnly')
 * @returns The current authorization status for the specified access level
 *
 * @example
 * import { checkPhotoKitPermission } from "tauri-plugin-macos-permissions-api";
 *
 * const status = await checkPhotoKitPermission('read');
 * console.log(status); // "authorized" | "denied" | "notDetermined" | ...
 */
export const checkPhotoKitPermission = (accessLevel: PhotoKitAccessLevel): Promise<PhotoKitAuthorizationStatus> => {
    return invoke<PhotoKitAuthorizationStatus>(COMMAND.CHECK_PHOTOKIT_PERMISSION, { accessLevel });
};

/**
 * Request PhotoKit permission for the specified access level.
 *
 * This will show the system permission dialog if the permission has not been determined yet.
 *
 * @param accessLevel - The PhotoKit access level to request ('read' | 'readWrite' | 'addOnly')
 * @returns The authorization status after user response
 *
 * @example
 * import { requestPhotoKitPermission } from "tauri-plugin-macos-permissions-api";
 *
 * try {
 *   const status = await requestPhotoKitPermission('readWrite');
 *   if (status === 'authorized') {
 *     console.log('权限已授予');
 *   } else {
 *     console.log('权限被拒绝或受限:', status);
 *   }
 * } catch (error) {
 *   console.error('请求权限失败:', error);
 * }
 */
export const requestPhotoKitPermission = (accessLevel: PhotoKitAccessLevel): Promise<PhotoKitAuthorizationStatus> => {
    return invoke<PhotoKitAuthorizationStatus>(COMMAND.REQUEST_PHOTOKIT_PERMISSION, { accessLevel });
};

/**
 * Register a PhotoKit permission status listener.
 *
 * This creates a listener that will emit events when the PhotoKit permission status changes
 * for the specified access level.
 *
 * @param accessLevel - The PhotoKit access level to monitor ('read' | 'readWrite' | 'addOnly')
 * @returns The listener ID on success
 *
 * @example
 * import { registerPhotoKitPermissionListener } from "tauri-plugin-macos-permissions-api";
 * import { listen } from '@tauri-apps/api/event';
 *
 * // 注册监听器
 * const listenerId = await registerPhotoKitPermissionListener('read');
 *
 * // 监听权限状态变化事件
 * const unlisten = await listen('photokit-permission-changed', (event) => {
 *   console.log('权限状态变化:', event.payload);
 * });
 *
 * // 稍后注销监听器
 * await unregisterPhotoKitPermissionListener(listenerId);
 */
export const registerPhotoKitPermissionListener = (accessLevel: PhotoKitAccessLevel): Promise<string> => {
    return invoke<string>(COMMAND.REGISTER_PHOTOKIT_PERMISSION_LISTENER, { accessLevel });
};

/**
 * Unregister a PhotoKit permission status listener.
 *
 * This removes a previously registered listener and stops monitoring permission changes.
 *
 * @param listenerId - The ID of the listener to unregister
 *
 * @example
 * import { unregisterPhotoKitPermissionListener } from "tauri-plugin-macos-permissions-api";
 *
 * // 注销监听器
 * try {
 *   await unregisterPhotoKitPermissionListener('your-listener-id');
 *   console.log('监听器已注销');
 * } catch (error) {
 *   console.error('注销监听器失败:', error);
 * }
 */
export const unregisterPhotoKitPermissionListener = (listenerId: string): Promise<void> => {
    return invoke<void>(COMMAND.UNREGISTER_PHOTOKIT_PERMISSION_LISTENER, { listenerId });
};

/**
 * Get all active PhotoKit permission listeners.
 *
 * This returns information about all currently registered permission listeners.
 *
 * @returns List of active listeners
 *
 * @example
 * import { getPhotoKitPermissionListeners } from "tauri-plugin-macos-permissions-api";
 *
 * const listeners = await getPhotoKitPermissionListeners();
 * console.log('活跃的监听器:', listeners);
 */
export const getPhotoKitPermissionListeners = (): Promise<ListenerInfo[]> => {
    return invoke<ListenerInfo[]>(COMMAND.GET_PHOTOKIT_PERMISSION_LISTENERS);
};
