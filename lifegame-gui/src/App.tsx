import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Canvas from "./Canvas";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  function draw(context: CanvasRenderingContext2D, frameCount: number) {
    context.clearRect(0, 0, context.canvas.width, context.canvas.height)
    context.fillStyle = '#000000'
    context.beginPath()
    context.arc(50, 100, 20 * Math.sin(frameCount * 0.05) ** 2, 0, 2 * Math.PI)
    context.fill()
  }

  return (
    <div className="container">
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>

      <Canvas draw={draw} />
    </div>
  );
}

export default App;
