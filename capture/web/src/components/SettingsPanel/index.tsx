import { useState } from "react";
import { useUIStore } from "../../store/uiStore";
import { MODEL_ENGINES, type EngineId } from "../../core/models/registry";
import "./SettingsPanel.css";

export default function SettingsPanel() {
  const isSettingsOpen = useUIStore((state) => state.isSettingsOpen);

  if (!isSettingsOpen) return null;

  return <SettingsModal />;
}

function SettingsModal() {
  const {
    setSettingsOpen,
    cameras,
    selectedCameraId,
    setSelectedCameraId,
    selectedEngine,
    selectedVariant,
    setModel,
  } = useUIStore();

  const [isClosing, setIsClosing] = useState(false);

  const [localCameraId, setLocalCameraId] = useState(selectedCameraId);
  const [localEngine, setLocalEngine] = useState<EngineId>(selectedEngine);
  const [localVariant, setLocalVariant] = useState(selectedVariant);

  const currentEngineConfig = MODEL_ENGINES[localEngine];

  const closeWithAnimation = (onClosed?: () => void) => {
    setIsClosing(true);
    setTimeout(() => {
      onClosed?.();
      setSettingsOpen(false);
    }, 200);
  };

  const handleCancel = () => {
    closeWithAnimation();
  };

  const handleApply = () => {
    closeWithAnimation(() => {
      if (localCameraId !== selectedCameraId) {
        setSelectedCameraId(localCameraId);
      }
      if (localEngine !== selectedEngine || localVariant !== selectedVariant) {
        setModel(localEngine, localVariant);
      }
    });
  };

  return (
    <div
      className={`settings-overlay ${isClosing ? "closing" : ""}`}
      onClick={handleCancel}
    >
      <div
        className={`settings-modal ${isClosing ? "closing" : ""}`}
        onClick={(e) => e.stopPropagation()}
      >
        <div className="settings-header">
          <h2>Настройки</h2>
          <button
            type="button"
            className="close-btn"
            onClick={handleCancel}
            aria-label="Закрыть"
          >
            ×
          </button>
        </div>

        <div className="settings-content">
          <div className="settings-section">
            <label>Камера</label>
            {cameras.length === 0 ? (
              <div className="empty-cameras">Камеры не найдены</div>
            ) : (
              <select
                value={localCameraId || ""}
                onChange={(e) => setLocalCameraId(e.target.value)}
              >
                {cameras.map((cam) => (
                  <option key={cam.deviceId} value={cam.deviceId}>
                    {cam.label || `Камера: ${cam.deviceId.slice(0, 5)}...`}
                  </option>
                ))}
              </select>
            )}
          </div>

          <div className="settings-section">
            <label>Движок трекинга</label>
            <select
              value={localEngine}
              onChange={(e) => {
                const newEngine = e.target.value as EngineId;
                const defaultVariant = MODEL_ENGINES[newEngine].defaultVariant;
                setLocalEngine(newEngine);
                setLocalVariant(defaultVariant);
              }}
            >
              {Object.values(MODEL_ENGINES).map((engine) => (
                <option key={engine.id} value={engine.id}>
                  {engine.label}
                </option>
              ))}
            </select>
          </div>

          <div className="settings-section">
            <label>Точность модели</label>
            <div className="variants-group">
              {currentEngineConfig.variants.map((variant) => (
                <button
                  type="button"
                  key={variant.id}
                  className={`variant-btn ${localVariant === variant.id ? "active" : ""}`}
                  onClick={() => setLocalVariant(variant.id)}
                >
                  {variant.label}
                </button>
              ))}
            </div>
          </div>
        </div>

        <div className="settings-footer">
          <button
            type="button"
            className="footer-btn btn-apply"
            onClick={handleApply}
          >
            Применить
          </button>
          <button
            type="button"
            className="footer-btn btn-cancel"
            onClick={handleCancel}
          >
            Отмена
          </button>
        </div>
      </div>
    </div>
  );
}
