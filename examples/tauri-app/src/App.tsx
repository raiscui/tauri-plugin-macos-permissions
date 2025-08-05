import { useCreation, useMount, useReactive } from "ahooks";
import { Button, Flex, List, Typography, Card, Space, Tag, message } from "antd";
import { listen } from '@tauri-apps/api/event';
import {
    checkAccessibilityPermission,
    checkFullDiskAccessPermission,
    requestAccessibilityPermission,
    requestFullDiskAccessPermission,
    checkScreenRecordingPermission,
    requestScreenRecordingPermission,
    checkMicrophonePermission,
    requestMicrophonePermission,
    checkCameraPermission,
    requestCameraPermission,
    checkInputMonitoringPermission,
    requestInputMonitoringPermission,
    checkPhotoKitPermission,
    requestPhotoKitPermission,
    registerPhotoKitPermissionListener,
    unregisterPhotoKitPermissionListener,
    getPhotoKitPermissionListeners,
    type PhotoKitAccessLevel,
    type PhotoKitAuthorizationStatus,
    type ListenerInfo,
} from "tauri-plugin-macos-permissions-api";

const App = () => {
    const state = useReactive({
        accessibilityPermission: false,
        fullDiskAccessPermission: false,
        screenRecordingPermission: false,
        microphonePermission: false,
        cameraPermission: false,
        inputMonitoringPermission: false,
        // PhotoKit 权限状态
        photoKitReadPermission: "notDetermined" as PhotoKitAuthorizationStatus,
        photoKitReadWritePermission: "notDetermined" as PhotoKitAuthorizationStatus,
        photoKitAddOnlyPermission: "notDetermined" as PhotoKitAuthorizationStatus,
        // PhotoKit 监听器状态
        activeListeners: [] as ListenerInfo[],
        listenerIds: {
            read: null as string | null,
            readWrite: null as string | null,
            addOnly: null as string | null,
        } as Record<PhotoKitAccessLevel, string | null>,
    });

    useMount(async () => {
        // 检查传统权限
        state.accessibilityPermission = await checkAccessibilityPermission();
        state.fullDiskAccessPermission = await checkFullDiskAccessPermission();
        state.screenRecordingPermission = await checkScreenRecordingPermission();
        state.microphonePermission = await checkMicrophonePermission();
        state.cameraPermission = await checkCameraPermission();
        state.inputMonitoringPermission = await checkInputMonitoringPermission();

        // 检查 PhotoKit 权限
        try {
            state.photoKitReadPermission = await checkPhotoKitPermission("read");
            state.photoKitReadWritePermission = await checkPhotoKitPermission("readWrite");
            state.photoKitAddOnlyPermission = await checkPhotoKitPermission("addOnly");

            // 获取活跃的监听器
            state.activeListeners = await getPhotoKitPermissionListeners();
        } catch (error) {
            console.error("检查 PhotoKit 权限失败:", error);
            message.error("检查 PhotoKit 权限失败");
        }

        // 设置 PhotoKit 权限变化监听
        try {
            await listen('photokit-permission-changed', (event) => {
                console.log('PhotoKit 权限状态变化:', event.payload);
                message.info(`PhotoKit 权限状态已更新: ${JSON.stringify(event.payload)}`);

                // 重新检查权限状态
                refreshPhotoKitPermissions();
            });
        } catch (error) {
            console.error("设置 PhotoKit 权限监听失败:", error);
        }
    });

    // PhotoKit 权限相关函数
    const refreshPhotoKitPermissions = async () => {
        try {
            state.photoKitReadPermission = await checkPhotoKitPermission("read");
            state.photoKitReadWritePermission = await checkPhotoKitPermission("readWrite");
            state.photoKitAddOnlyPermission = await checkPhotoKitPermission("addOnly");
            state.activeListeners = await getPhotoKitPermissionListeners();
        } catch (error) {
            console.error("刷新 PhotoKit 权限失败:", error);
        }
    };

    const requestPhotoKitPermissionWithLevel = async (accessLevel: PhotoKitAccessLevel) => {
        try {
            const status = await requestPhotoKitPermission(accessLevel);
            message.success(`PhotoKit ${accessLevel} 权限请求完成: ${status}`);

            // 更新对应的权限状态
            if (accessLevel === "read") {
                state.photoKitReadPermission = status;
            } else if (accessLevel === "readWrite") {
                state.photoKitReadWritePermission = status;
            } else if (accessLevel === "addOnly") {
                state.photoKitAddOnlyPermission = status;
            }
        } catch (error) {
            console.error(`请求 PhotoKit ${accessLevel} 权限失败:`, error);
            message.error(`请求 PhotoKit ${accessLevel} 权限失败`);
        }
    };

    const togglePhotoKitListener = async (accessLevel: PhotoKitAccessLevel) => {
        try {
            const currentListenerId = state.listenerIds[accessLevel];

            if (currentListenerId) {
                // 注销监听器
                await unregisterPhotoKitPermissionListener(currentListenerId);
                state.listenerIds[accessLevel] = null;
                message.success(`PhotoKit ${accessLevel} 监听器已注销`);
            } else {
                // 注册监听器
                const listenerId = await registerPhotoKitPermissionListener(accessLevel);
                state.listenerIds[accessLevel] = listenerId;
                message.success(`PhotoKit ${accessLevel} 监听器已注册: ${listenerId}`);
            }

            // 刷新监听器列表
            state.activeListeners = await getPhotoKitPermissionListeners();
        } catch (error) {
            console.error(`切换 PhotoKit ${accessLevel} 监听器失败:`, error);
            message.error(`切换 PhotoKit ${accessLevel} 监听器失败`);
        }
    };

    const getStatusColor = (status: PhotoKitAuthorizationStatus) => {
        switch (status) {
            case "authorized":
                return "green";
            case "denied":
                return "red";
            case "restricted":
                return "orange";
            case "limited":
                return "blue";
            case "notDetermined":
            default:
                return "default";
        }
    };

    const getStatusText = (status: PhotoKitAuthorizationStatus) => {
        switch (status) {
            case "authorized":
                return "已授权";
            case "denied":
                return "已拒绝";
            case "restricted":
                return "受限制";
            case "limited":
                return "有限访问";
            case "notDetermined":
            default:
                return "未确定";
        }
    };

    const data = useCreation(() => {
        return [
            {
                label: "Accessibility Permission",
                value: state.accessibilityPermission,
                check: async () => {
                    await requestAccessibilityPermission();

                    const check = async () => {
                        state.accessibilityPermission =
                            await checkAccessibilityPermission();

                        if (state.accessibilityPermission) return;

                        setTimeout(check, 1000);
                    };

                    check();
                },
            },
            {
                label: "Full Disk Access Permission",
                value: state.fullDiskAccessPermission,
                check: requestFullDiskAccessPermission,
            },
            {
                label: "Screen Recording Permission",
                value: state.screenRecordingPermission,
                check: requestScreenRecordingPermission,
            },
            {
                label: "Microphone Permission",
                value: state.microphonePermission,
                check: async () => {
                    await requestMicrophonePermission();

                    const check = async () => {
                        state.microphonePermission = await checkMicrophonePermission();

                        if (state.microphonePermission) return;

                        setTimeout(check, 1000);
                    };

                    check();
                },
            },
            {
                label: "Camera Permission",
                value: state.cameraPermission,
                check: async () => {
                    await requestCameraPermission();

                    const check = async () => {
                        state.cameraPermission = await checkCameraPermission();

                        if (state.cameraPermission) return;

                        setTimeout(check, 1000);
                    };

                    check();
                },
            },
            {
                label: "Input Monitoring Permission",
                value: state.inputMonitoringPermission,
                check: requestInputMonitoringPermission,
            },
        ];
    }, [{ ...state }]);

    return (
        <Flex vertical gap="large" style={{ padding: "20px" }}>
            {/* 传统权限管理 */}
            <Card title="传统权限管理" size="small">
                <List
                    bordered
                    dataSource={data}
                    renderItem={(item) => {
                        const { label, value, check } = item;

                        return (
                            <List.Item key={label} title={label}>
                                <List.Item.Meta title={label} />

                                {value ? (
                                    <Typography.Text type="success">Authorized</Typography.Text>
                                ) : (
                                    <Button onClick={check}>Authorize Now</Button>
                                )}
                            </List.Item>
                        );
                    }}
                />
            </Card>

            {/* PhotoKit 权限管理 */}
            <Card title="PhotoKit 权限管理" size="small">
                <Space direction="vertical" style={{ width: "100%" }}>
                    {/* 权限状态显示 */}
                    <Card title="权限状态" size="small" type="inner">
                        <Space direction="vertical" style={{ width: "100%" }}>
                            <Flex justify="space-between" align="center">
                                <Typography.Text strong>读取权限 (Read):</Typography.Text>
                                <Space>
                                    <Tag color={getStatusColor(state.photoKitReadPermission)}>
                                        {getStatusText(state.photoKitReadPermission)}
                                    </Tag>
                                    <Button
                                        size="small"
                                        onClick={() => requestPhotoKitPermissionWithLevel("read")}
                                        disabled={state.photoKitReadPermission === "authorized"}
                                    >
                                        请求权限
                                    </Button>
                                </Space>
                            </Flex>

                            <Flex justify="space-between" align="center">
                                <Typography.Text strong>读写权限 (ReadWrite):</Typography.Text>
                                <Space>
                                    <Tag color={getStatusColor(state.photoKitReadWritePermission)}>
                                        {getStatusText(state.photoKitReadWritePermission)}
                                    </Tag>
                                    <Button
                                        size="small"
                                        onClick={() => requestPhotoKitPermissionWithLevel("readWrite")}
                                        disabled={state.photoKitReadWritePermission === "authorized"}
                                    >
                                        请求权限
                                    </Button>
                                </Space>
                            </Flex>

                            <Flex justify="space-between" align="center">
                                <Typography.Text strong>仅添加权限 (AddOnly):</Typography.Text>
                                <Space>
                                    <Tag color={getStatusColor(state.photoKitAddOnlyPermission)}>
                                        {getStatusText(state.photoKitAddOnlyPermission)}
                                    </Tag>
                                    <Button
                                        size="small"
                                        onClick={() => requestPhotoKitPermissionWithLevel("addOnly")}
                                        disabled={state.photoKitAddOnlyPermission === "authorized"}
                                    >
                                        请求权限
                                    </Button>
                                </Space>
                            </Flex>
                        </Space>
                    </Card>

                    {/* 监听器管理 */}
                    <Card title="权限监听器管理" size="small" type="inner">
                        <Space direction="vertical" style={{ width: "100%" }}>
                            <Flex justify="space-between" align="center">
                                <Typography.Text strong>读取权限监听器:</Typography.Text>
                                <Button
                                    size="small"
                                    type={state.listenerIds.read ? "primary" : "default"}
                                    onClick={() => togglePhotoKitListener("read")}
                                >
                                    {state.listenerIds.read ? "注销监听器" : "注册监听器"}
                                </Button>
                            </Flex>

                            <Flex justify="space-between" align="center">
                                <Typography.Text strong>读写权限监听器:</Typography.Text>
                                <Button
                                    size="small"
                                    type={state.listenerIds.readWrite ? "primary" : "default"}
                                    onClick={() => togglePhotoKitListener("readWrite")}
                                >
                                    {state.listenerIds.readWrite ? "注销监听器" : "注册监听器"}
                                </Button>
                            </Flex>

                            <Flex justify="space-between" align="center">
                                <Typography.Text strong>仅添加权限监听器:</Typography.Text>
                                <Button
                                    size="small"
                                    type={state.listenerIds.addOnly ? "primary" : "default"}
                                    onClick={() => togglePhotoKitListener("addOnly")}
                                >
                                    {state.listenerIds.addOnly ? "注销监听器" : "注册监听器"}
                                </Button>
                            </Flex>
                        </Space>
                    </Card>

                    {/* 活跃监听器列表 */}
                    <Card title={`活跃监听器 (${state.activeListeners.length})`} size="small" type="inner">
                        {state.activeListeners.length > 0 ? (
                            <List
                                size="small"
                                dataSource={state.activeListeners}
                                renderItem={(listener) => (
                                    <List.Item key={listener.id}>
                                        <List.Item.Meta
                                            title={
                                                <Space>
                                                    <Typography.Text code>{listener.access_level}</Typography.Text>
                                                    <Tag color={listener.active ? "green" : "red"}>
                                                        {listener.active ? "活跃" : "非活跃"}
                                                    </Tag>
                                                </Space>
                                            }
                                            description={
                                                <Space>
                                                    <Typography.Text type="secondary">
                                                        ID: {listener.id.substring(0, 8)}...
                                                    </Typography.Text>
                                                    <Typography.Text type="secondary">
                                                        创建时间: {new Date(listener.created_at * 1000).toLocaleString()}
                                                    </Typography.Text>
                                                </Space>
                                            }
                                        />
                                    </List.Item>
                                )}
                            />
                        ) : (
                            <Typography.Text type="secondary">暂无活跃监听器</Typography.Text>
                        )}
                    </Card>

                    {/* 操作按钮 */}
                    <Card title="操作" size="small" type="inner">
                        <Space>
                            <Button onClick={refreshPhotoKitPermissions}>
                                刷新权限状态
                            </Button>
                            <Button
                                onClick={async () => {
                                    try {
                                        const listeners = await getPhotoKitPermissionListeners();
                                        state.activeListeners = listeners;
                                        message.success("监听器列表已刷新");
                                    } catch (error) {
                                        message.error("刷新监听器列表失败");
                                    }
                                }}
                            >
                                刷新监听器列表
                            </Button>
                        </Space>
                    </Card>
                </Space>
            </Card>
        </Flex>
    );
};

export default App;
