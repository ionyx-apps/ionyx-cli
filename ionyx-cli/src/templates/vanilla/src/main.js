// Ionyx IPC setup
window.ionyx = {
  invoke: (command, payload = {}) => {
    return new Promise((resolve, reject) => {
      const id = Math.random().toString(36).substr(2, 9)
      const request = { id, command, payload }
      
      console.log("🚀 Sending IPC request:", request)
      
      window.ionyx.resolveResponse = (responseId, response) => {
        if (responseId === id) {
          console.log("📥 Received IPC response:", response)
          if (response.success) {
            resolve(response.data)
          } else {
            reject(new Error(response.error))
          }
        }
      }
      
      // Send IPC request to Rust backend
      if ((window as any).ipc) {
        console.log("📤 Sending via window.ipc.postMessage")
        ;(window as any).ipc.postMessage(JSON.stringify(request))
      } else {
        console.error("❌ window.ipc not available")
        reject(new Error("IPC not available"))
      }
    })
  },
  resolveResponse: () => {}
}

// Test IPC communication
async function testConnection() {
  try {
    const info = await window.ionyx.invoke("app.getVersion")
    document.getElementById("app-name").textContent = info.name
    document.getElementById("app-version").textContent = info.version
    document.getElementById("status").textContent = "Connected to Ionyx Framework! 🚀"
  } catch (error) {
    document.getElementById("status").textContent = "Error connecting to backend"
    console.error("IPC Error:", error)
  }
}

testConnection()
