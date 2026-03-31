declare global {
  namespace App {}
}

interface ImportMetaEnv {
  readonly VITE_AGENT_ORIGIN?: string;
  readonly VITE_AGENT_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

export {};
