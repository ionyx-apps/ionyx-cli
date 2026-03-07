declare global {
  interface Window {
    ionyx: {
      invoke: (command: string, payload?: any) => Promise<any>;
      resolveResponse: (responseId: string, response: any) => void;
      fs: {
        readFile: (path: string) => Promise<{ content: string }>;
        writeFile: (path: string, content: string) => Promise<{ success: boolean }>;
        exists: (path: string) => Promise<{ exists: boolean }>;
        readDir: (path: string) => Promise<{ entries: any[] }>;
      };
      os: {
        info: () => Promise<{ platform: string, arch: string, version: string, hostname: string }>;
      };
      dialog: {
        openFile: () => Promise<{ filePath: string | null }>;
        saveFile: () => Promise<{ filePath: string | null }>;
      };
      app: {
        getVersion: () => Promise<{ name: string, version: string }>;
        getConfig: () => Promise<any>;
      };
      network: {
        request: (url: string, method?: string, body?: any) => Promise<{ status: number, headers: any, body: string }>;
      };
    };
  }

  interface Navigator {
    gpu: {
      requestAdapter(): Promise<any>;
    };
  }
}

declare module "*.svelte" {
  import type { ComponentType } from "svelte";
  const component: ComponentType;
  export default component;
}

export {};