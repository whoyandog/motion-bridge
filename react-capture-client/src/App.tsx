import { useCallback } from "react";
import HUD from "./components/HUD";
import PoseCapture from "./components/PoseCapture";
import type { MocapFrame } from "./types";

function App() {
  const handleFrame = useCallback((frame: MocapFrame) => {
    if (Math.random() < 0.02) {
      console.log("Скелет: ", frame);
    }
  }, []);

  return (
    <div>
      <HUD />
      <PoseCapture modelType="lite" onFrame={handleFrame} />
    </div>
  );
}

export default App;
