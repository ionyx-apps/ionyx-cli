<script lang="ts">
  import App from "./App.svelte";

  // Ionyx IPC setup
  declare global {
    interface Window {
      ionyx: {
        invoke: (command: string, payload?: any) => Promise<any>;
        resolveResponse: (responseId: string, response: any) => void;
      }
    }
  }

  window.ionyx = {
    invoke: (command: string, payload?: any) => {
      return new Promise((resolve, reject) => {
        // Generate unique ID with timestamp and random
        const id = `${Date.now()}_${Math.random().toString(36).substr(2, 9)}_${command}`;
        const request = {
          id,
          command,
          payload: payload || {}
        };
        
        console.log("🚀 Sending IPC request:", request);
        
        // Store the resolve/reject functions for this request
        const timeoutId = setTimeout(() => {
          console.error("❌ IPC request timeout for:", command);
          reject(new Error("IPC request timeout"));
        }, 15000);
        
        // Store handler (in a real app, you would use a Map)
        window.ionyx.resolveResponse = (responseId: string, response: any) => {
          if (responseId === id) {
            console.log("📥 Received IPC response:", response);
            clearTimeout(timeoutId);
            if (response.success) {
              resolve(response.data);
            } else {
              reject(new Error(response.error));
            }
          }
        };
        
        // Send IPC request to Rust backend
        if ((window as any).ipc) {
          console.log("📤 Sending via window.ipc.postMessage");
          (window as any).ipc.postMessage(JSON.stringify(request));
        } else {
          console.error("❌ window.ipc not available");
          reject(new Error("IPC not available"));
        }
      });
    },
    resolveResponse: () => {}
  };

  const app = new App({
    target: document.getElementById("app")
  });

  export default app;
</script>

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>my-ionyx-app - Ionyx Framework</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
