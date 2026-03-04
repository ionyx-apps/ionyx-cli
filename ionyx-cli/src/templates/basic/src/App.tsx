declare global {
  interface Window {
    ionyx: {
      invoke: (args: { command: string; payload?: any }) => Promise<any>;
    };
  }
}

import { useState } from "react"
import "./App.css"

function App() {
  const [message, setMessage] = useState("Hello Ionyx Framework!")

  return (
    <div className="App">
      <h1>{message}</h1>
      <button onClick={() => setMessage("Button clicked!")}>
        Click me
      </button>
    </div>
  )
}

export default App
