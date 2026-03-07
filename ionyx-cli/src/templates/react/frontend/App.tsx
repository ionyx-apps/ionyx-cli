declare global {
  interface Window {
    ionyx: {
      invoke: (command: string, payload?: any) => Promise<any>;
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
    fusion: any;
  }

  interface Navigator {
    gpu: {
      requestAdapter(): Promise<any>;
    };
  }
}

import { useState, useEffect } from "react"
import "./App.css"

function App() {
  const [message, setMessage] = useState("Loading Ionyx Framework...")
  const [appInfo, setAppInfo] = useState<any>(null)
  const [webGpuSupported, setWebGpuSupported] = useState<boolean | null>(null)

  useEffect(() => {
    // Check WebGPU support
    const checkWebGPU = async () => {
      if (navigator.gpu) {
        try {
          const adapter = await navigator.gpu.requestAdapter()
          setWebGpuSupported(!!adapter)
        } catch (e) {
          setWebGpuSupported(false)
        }
      } else {
        setWebGpuSupported(false)
      }
    }

    // Test IPC communication
    const testIPC = async () => {
      try {
        const info = await window.ionyx.invoke("app.getVersion")
        setAppInfo(info)
        setMessage("Hello from Ionyx Framework! 🚀")
      } catch (error) {
        setMessage("Error connecting to backend")
        console.error("IPC Error:", error)
      }
    }

    checkWebGPU()
    testIPC()
  }, [])

  return (
    <div className="App">
      <header className="App-header">
        <h1>{message}</h1>
        {appInfo && (
          <div className="app-info">
            <p><strong>App:</strong> {appInfo.name}</p>
            <p><strong>Version:</strong> {appInfo.version}</p>
            <p><strong>Platform:</strong> {appInfo.platform}</p>
          </div>
        )}
        <div className="features">
          <h2>🚀 Ionyx Framework Features</h2>
          <ul>
            <li>✅ File System Access</li>
            <li>✅ Network Requests</li>
            <li>✅ OS Information</li>
            <li>✅ Cross-platform Desktop Apps</li>
            <li>✅ Rust Backend Performance</li>
            <li>✅ Modern Frontend Frameworks</li>
            <li>✅ Native Fusion (Seamless State Sync)</li>
            <li>{webGpuSupported === true ? "✅ WebGPU Supported" : webGpuSupported === false ? "❌ WebGPU Not Supported" : "⏳ Checking WebGPU..."}</li>
          </ul>
        </div>
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
      </header>
    </div>
  )
}

export default App
