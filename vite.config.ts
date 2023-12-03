import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path, { resolve } from "path";

const root = resolve(__dirname, "src");
const outDir = resolve(__dirname, "dist");

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    root: root,
    plugins: [vue()],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 5173,
        strictPort: true,
    },
    // 3. to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ["VITE_", "TAURI_"],
    css: {
        preprocessorOptions: {
            scss: {
                additionalData: `@import "@/assets/vars-global.scss";`,
            },
        },
    },
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "src"),
        },
    },
    build: {
        outDir: outDir,
        emptyOutDir: true,
        rollupOptions: {
            // Needed for multiple windows (main, vaults, etc.)
            input: {
                main: resolve(root, "index.html"),
                /* vite docs:
                During dev, simply navigate or link to /vaults/ - it works as 
                expected, just like for a normal static file server. */
                vaults: resolve(root, "windows/vaults/index.html"),
            },
        },
    },
}));
