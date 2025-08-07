import ReactDOM from "react-dom/client";
import { attachConsole } from '@tauri-apps/plugin-log';
import App from "./App";

// 附加控制台日志，这样前端日志也会被记录到 Rust 日志系统
attachConsole().then(() => {
    console.log("🔗 前端控制台已连接到 Tauri 日志系统");
}).catch((err) => {
    console.error("❌ 连接前端控制台到 Tauri 日志系统失败:", err);
});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <App />
);
