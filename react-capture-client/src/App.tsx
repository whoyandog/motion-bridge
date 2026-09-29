import { useCallback } from "react";
import HUD from "./components/HUD";
import PoseCapture from "./components/PoseCapture";
import type { MocapFrame } from "./types";
import { useMocapSocket } from "./hooks/useMocapSocket.ts";

function App() {
  const { sendFrame, isConnected } = useMocapSocket(
    "wss://192.168.88.24:8443/ws",
  );

  const handleFrame = useCallback((frame: MocapFrame) => {
    if (Math.random() < 0.02) {
      console.log("Скелет: ", frame);
      sendFrame(frame);
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
