import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// struct MidiDevice
type MidiDevice = {
  id: number;
  d_name: string;
};

function App() {
  const [devices, setDevices] = useState<MidiDevice[]>([]); // state to hold device list

  async function getMidiDevices() {
    const result = await invoke<MidiDevice[]>("get_midi_devices");
    setDevices(result); // store it in state
  }

  return (
    <main>
      <h1>keyboard^2</h1>

      <form
        onSubmit={(e) => {
          e.preventDefault(); // prevent page reload
          getMidiDevices();
        }}
      >
        <button type="submit">List MIDI devices</button>
      </form>

      <ul>
        {devices.map((device) => (
          <li key={device.id}>
            <pre style={{ margin: 0 }}>
              {device.d_name.padEnd(20)}     ID: {device.id.toString().padStart(2, " ")}
            </pre>
          </li>
        ))}
      </ul>
    </main>
  );
}

export default App;
