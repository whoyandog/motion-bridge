import { useUIStore } from "../../store/uiStore";
import "./HUD.css";

export default function HUD() {
  const isConnected = useUIStore((state) => state.isConnected);

  return (
    <div className="hud-container">
      <header className="hud-header">
        <h1 className="hud-title">Motion bridge</h1>
        <span style={{ color: isConnected ? "#00FF00" : "#FF0000" }}>
          {isConnected ? "Online" : "Offline"}
        </span>
      </header>
    </div>
  );
}
