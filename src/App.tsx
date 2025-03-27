import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("get_midi_devices", { name }));
  }

  async function test_lib() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("test_lib", { name }));
  }

  return (
    <main>
      <h1>Welcome to Tauri + React</h1>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <form
        onSubmit={(e) => {
          e.preventDefault();
          test_lib();
        }}
      >
        <button type="submit">Test Lib</button>
      </form>
      <p>{greetMsg}</p>

    </main>
  );
}

export default App;
