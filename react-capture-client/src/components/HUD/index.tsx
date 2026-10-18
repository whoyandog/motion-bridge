import { useUIStore } from "../../store/uiStore";
import "./HUD.css";

export default function HUD() {
  const isConnected = useUIStore((state) => state.isConnected);
  const fps = useUIStore((state) => state.fps);
  const setSettingsOpen = useUIStore((state) => state.setSettingsOpen);

  return (
    <div className="hud">
      <header>
        <h1>Motion bridge</h1>

        <div className="status-bar">
          <div className="status">
            <span>Server is</span>
            <span className={isConnected ? "online" : "offline"}>
              {isConnected ? "online" : "offline"}
            </span>
          </div>

          <span className="fps-counter">FPS: {fps}</span>
        </div>
      </header>

      <div className="controls">
        <button
          className="control-tile settings-btn"
          onClick={() => setSettingsOpen(true)}
          title="Настройки"
          aria-label="Настройки"
        >
          <span className="icon">⚙️</span>
        </button>
      </div>
    </div>
  );
}
