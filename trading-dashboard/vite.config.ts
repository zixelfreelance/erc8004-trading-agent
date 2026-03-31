import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const target = env.VITE_AGENT_URL ?? "http://127.0.0.1:3030";

  return {
    plugins: [sveltekit()],
    server: {
      proxy: {
        "/logs": { target, changeOrigin: true },
        "/decision-schema": { target, changeOrigin: true },
      },
    },
  };
});
