import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // IPC smoke test: webview -> Rust core (smith-app greet command).
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Smith</h1>
      <p>UML modeling, humans and agents together.</p>
      <div className="row">
        <input
          value={name}
          onChange={(event) => setName(event.currentTarget.value)}
          placeholder="Enter a name"
          aria-label="Name"
        />
        <button type="button" onClick={() => void greet()}>
          Greet
        </button>
      </div>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
