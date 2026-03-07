<template>
  <div class="app">
    <header class="app-header">
      <h1>{{ message }}</h1>

      <div v-if="appInfo" class="app-info">
        <p><strong>App:</strong> {{ appInfo.name }}</p>
        <p><strong>Version:</strong> {{ appInfo.version }}</p>
        <p><strong>Platform:</strong> {{ appInfo.platform }}</p>
      </div>

      <div class="features">
        <h2>🚀 Ionyx Framework Features</h2>
        <ul>
          <li>✅ File System Access</li>
          <li>✅ Network Requests</li>
          <li>✅ OS Information</li>
          <li>✅ Cross-platform Desktop Apps</li>
          <li>✅ Rust Backend Performance</li>
          <li>✅ Vue 3 Reactive Frontend</li>
          <li>
            {{
              webGpuSupported === true
                ? "✅ WebGPU Supported"
                : webGpuSupported === false
                  ? "❌ WebGPU Not Supported"
                  : "⏳ Checking WebGPU..."
            }}
          </li>
        </ul>
      </div>

      <p>Edit <code>src/App.vue</code> and save to reload.</p>
    </header>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";

declare global {
  interface Window {
    ionyx: {
      invoke: (command: string, payload?: any) => Promise<any>;
      fs: {
        readFile: (path: string) => Promise<{ content: string }>;
        writeFile: (
          path: string,
          content: string,
        ) => Promise<{ success: boolean }>;
        exists: (path: string) => Promise<{ exists: boolean }>;
        readDir: (path: string) => Promise<{ entries: any[] }>;
      };
      os: {
        info: () => Promise<{
          platform: string;
          arch: string;
          version: string;
          hostname: string;
        }>;
      };
      dialog: {
        openFile: () => Promise<{ filePath: string | null }>;
        saveFile: () => Promise<{ filePath: string | null }>;
      };
      app: {
        getVersion: () => Promise<{ name: string; version: string }>;
        getConfig: () => Promise<any>;
      };
      network: {
        request: (
          url: string,
          method?: string,
          body?: any,
        ) => Promise<{ status: number; headers: any; body: string }>;
      };
    };
  }

  interface Navigator {
    gpu: {
      requestAdapter(): Promise<any>;
    };
  }
}

const message = ref("Loading Ionyx Framework...");
const appInfo = ref<any>(null);
const webGpuSupported = ref<boolean | null>(null);

onMounted(async () => {
  // Check WebGPU support
  const checkWebGPU = async () => {
    if (navigator.gpu) {
      try {
        const adapter = await navigator.gpu.requestAdapter();
        webGpuSupported.value = !!adapter;
      } catch (e) {
        webGpuSupported.value = false;
      }
    } else {
      webGpuSupported.value = false;
    }
  };

  // Test IPC communication
  try {
    const info = await window.ionyx.invoke("app.getVersion");
    appInfo.value = info;
    message.value = "Hello from Ionyx Framework! 🚀";
  } catch (error) {
    message.value = "Error connecting to backend";
    console.error("IPC Error:", error);
  }

  checkWebGPU();
});
</script>

<style scoped>
.app {
  text-align: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #42b883 0%, #35495e 100%);
  color: white;
}

.app-header {
  padding: 2rem;
  max-width: 800px;
  margin: 0 auto;
}

.app-header h1 {
  font-size: 3rem;
  margin-bottom: 1rem;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
}

.app-info {
  background: rgba(255, 255, 255, 0.1);
  padding: 1.5rem;
  border-radius: 12px;
  margin: 1.5rem 0;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.app-info p {
  margin: 0.5rem 0;
  font-size: 1.1rem;
}

.features {
  background: rgba(255, 255, 255, 0.1);
  padding: 2rem;
  border-radius: 12px;
  margin: 1.5rem 0;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.features h2 {
  margin-bottom: 1rem;
  color: #fff;
}

.features ul {
  list-style: none;
  padding: 0;
  text-align: left;
  max-width: 400px;
  margin: 0 auto;
}

.features li {
  padding: 0.5rem 0;
  font-size: 1.1rem;
}

p {
  margin-top: 2rem;
  opacity: 0.8;
}
</style>
