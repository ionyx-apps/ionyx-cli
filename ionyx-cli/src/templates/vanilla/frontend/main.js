
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

async function checkWebGPU() {
  const statusEl = document.getElementById("webgpu-status")
  if (navigator.gpu) {
    try {
      const adapter = await navigator.gpu.requestAdapter()
      if (adapter) {
        statusEl.textContent = "✅ WebGPU Supported"
      } else {
        statusEl.textContent = "❌ WebGPU Not Supported"
      }
    } catch (e) {
      statusEl.textContent = "❌ WebGPU Not Supported"
    }
  } else {
    statusEl.textContent = "❌ WebGPU Not Supported"
  }
}

testConnection()
checkWebGPU()
