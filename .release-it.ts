import type { Config } from "release-it";

export default {
    git: {
        commitMessage: "tauri-plugin-macos-permissions-with-photokit-api v${version}",
        tagName: "v${version}",
    },
    npm: {
        publish: true,
    },
} satisfies Config;

