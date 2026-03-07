declare global {
  interface Window {
    ionyx: {
      invoke: (command: string, payload?: any) => Promise<any>;
    };
  }
}

import { useState, useEffect } from "react"
import "./App.css"

function App() {
  const [message, setMessage] = useState("Loading Ionyx Framework...")
  const [appInfo, setAppInfo] = useState<any>(null)

  useEffect(() => {
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
