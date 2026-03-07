<script lang="ts">
  import { onMount } from "svelte";

  declare global {
    interface Window {
      ionyx: {
        invoke: (command: string, payload?: any) => Promise<any>;
      };
    }
  }

  let message = "Loading Ionyx Framework...";
  let appInfo: any = null;
  let webGpuSupported: boolean | null = null;

  onMount(async () => {
    // Check WebGPU support
    const checkWebGPU = async () => {
      if (navigator.gpu) {
        try {
          const adapter = await navigator.gpu.requestAdapter();
          webGpuSupported = !!adapter;
        } catch (e) {
          webGpuSupported = false;
        }
      } else {
        webGpuSupported = false;
      }
    };

    // Test IPC communication
    try {
      const info = await window.ionyx.invoke("app.getVersion");
      appInfo = info;
      message = "Hello from Ionyx Framework! 🚀";
    } catch (error) {
      message = "Error connecting to backend";
      console.error("IPC Error:", error);
    }

    checkWebGPU();
  });
</script>

<main>
  <h1>{message}</h1>
  
  {#if appInfo}
    <div class="app-info">
      <p><strong>App:</strong> {appInfo.name}</p>
      <p><strong>Version:</strong> {appInfo.version}</p>
      <p><strong>Platform:</strong> {appInfo.platform}</p>
    </div>
  {/if}
  
  <div class="features">
    <h2>🚀 Ionyx Framework Features</h2>
    <ul>
      <li>✅ File System Access</li>
      <li>✅ Network Requests</li>
      <li>✅ OS Information</li>
      <li>✅ Cross-platform Desktop Apps</li>
      <li>✅ Rust Backend Performance</li>
      <li>✅ Svelte Reactive Frontend</li>
      <li>{webGpuSupported === true ? "✅ WebGPU Supported" : webGpuSupported === false ? "❌ WebGPU Not Supported" : "⏳ Checking WebGPU..."}</li>
    </ul>
  </div>
  
  <p>
    Edit <code>src/App.svelte</code> and save to reload.
  </p>
</main>

<style>
  main {
    text-align: center;
    min-height: 100vh;
    background: linear-gradient(135deg, #ff3e00 0%, #ff6b00 100%);
    color: white;
    padding: 2rem;
    max-width: 800px;
    margin: 0 auto;
  }

  h1 {
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
