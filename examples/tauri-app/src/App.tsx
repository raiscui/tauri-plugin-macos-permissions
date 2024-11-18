import { useCreation, useMount, useReactive } from "ahooks";
import { Button, Flex, List, Typography } from "antd";
import {
  checkAccessibilityPermissions,
  checkFullDiskAccessPermissions,
  requestAccessibilityPermissions,
  requestFullDiskAccessPermissions,
} from "tauri-plugin-macos-permissions-api";

const App = () => {
  const state = useReactive({
    accessibilityPermissions: false,
    fullDiskAccessPermissions: false,
  });

  useMount(async () => {
    state.accessibilityPermissions = await checkAccessibilityPermissions();
    state.fullDiskAccessPermissions = await checkFullDiskAccessPermissions();
  });

  const data = useCreation(() => {
    return [
      {
        label: "Accessibility Permissions",
        value: state.accessibilityPermissions,
        check: async () => {
          await requestAccessibilityPermissions();

          const check = async () => {
            const opened = await checkAccessibilityPermissions();

            state.accessibilityPermissions = opened;

            if (opened) return;

            setTimeout(check, 1000);
          };

          check();
        },
      },
      {
        label: "Full Disk Access Permissions",
        value: state.fullDiskAccessPermissions,
        check: requestFullDiskAccessPermissions,
      },
    ];
  }, [state.accessibilityPermissions, state.fullDiskAccessPermissions]);

  return (
    <Flex vertical gap="middle">
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
    </Flex>
  );
};

export default App;
