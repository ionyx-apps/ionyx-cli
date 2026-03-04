// Ionyx IPC Bridge JavaScript
window.ionyx = {
  invoke: (command, payload = {}) => {
    return new Promise((resolve, reject) => {
      // Generate unique ID with timestamp and random
      const id = `${Date.now()}_${Math.random().toString(36).substr(2, 9)}_${command}`
      const request = {
        id,
        command,
        payload: payload || {}
      }
      
      console.log("🚀 Sending IPC request:", request)
      
      // Store the resolve/reject functions for this request
      const timeoutId = setTimeout(() => {
        console.error("❌ IPC request timeout for:", command)
        reject(new Error("IPC request timeout"))
      }, 15000)
      
      // Store handler (in a real app, you would use a Map)
      window.ionyx.resolveResponse = (responseId, response) => {
        if (responseId === id) {
          console.log("📥 Received IPC response:", response)
          clearTimeout(timeoutId)
          if (response.success) {
            resolve(response.data)
          } else {
            reject(new Error(response.error))
          }
        }
      }
      
      // Send IPC request to Rust backend
      if (window.ipc) {
        console.log("📤 Sending via window.ipc.postMessage")
        window.ipc.postMessage(JSON.stringify(request))
      } else {
        console.error("❌ window.ipc not available")
        reject(new Error("IPC not available"))
      }
    })
  },
  resolveResponse: () => {}
}

console.log("🔧 Ionyx IPC Bridge initialized")
