import { useUIStore } from "../../store/uiStore";
import "./HUD.css";

export default function HUD() {
  const isConnected = useUIStore((state) => state.isConnected);
  const fps = useUIStore((state) => state.fps);

  return (
    <header className="hud-overlay">
      <div className="left-panel">
        <h1>Motion bridge</h1>
        <div className={`status ${isConnected ? "online" : "offline"}`}>
          {isConnected ? "Online" : "Offline"}
        </div>
      </div>

      <div className="right-panel">FPS: {fps}</div>
    </header>
  );
}
