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
  const [message, setMessage] = useState("Hello Ionyx Framework!")
  const [webGpuSupported, setWebGpuSupported] = useState<boolean | null>(null)

  useEffect(() => {
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
    checkWebGPU()
  }, [])

  return (
    <div className="App">
      <h1>{message}</h1>
      <div className="status-badge" style={{ marginTop: '1rem', padding: '0.5rem', borderRadius: '4px', background: 'rgba(255,255,255,0.1)' }}>
        {webGpuSupported === true ? "✅ WebGPU Supported" : webGpuSupported === false ? "❌ WebGPU Not Supported" : "⏳ Checking WebGPU..."}
      </div>
      <button onClick={() => setMessage("Button clicked!")} style={{ marginTop: '2rem' }}>
        Click me
      </button>
    </div>
  )
}

export default App
