import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const App = () => {
  const [count, setCount] = useState("");

  useEffect(() => {
    const getMessage = async () => {
      const scriptCount: string = await invoke('count');
      setCount(scriptCount);
    }

    getMessage();
  }, [])

  return (
    <div className="container">
      <h1>Welcome to Hildegard!</h1>

      <p>{`There are ${count} scripts in this Hildegard instance.`}</p>
    </div>
  );
}

export default App;
