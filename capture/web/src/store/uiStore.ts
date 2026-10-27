import { create } from "zustand";
import { MODEL_ENGINES, type EngineId } from "../core/models/registry";

interface UIState {
  isConnected: boolean;
  fps: number;
  isSettingsOpen: boolean;

  cameras: MediaDeviceInfo[];
  selectedCameraId: string;
  selectedEngine: EngineId;
  selectedVariant: string;

  setConnected: (status: boolean) => void;
  setFps: (fps: number) => void;
  setSettingsOpen: (isOpen: boolean) => void;
  setCameras: (cameras: MediaDeviceInfo[]) => void;
  setSelectedCameraId: (id: string) => void;
  setModel: (engine: EngineId, variant: string) => void;
}

export const useUIStore = create<UIState>((set) => ({
  isConnected: false,
  fps: 0,
  isSettingsOpen: false,

  cameras: [],
  selectedCameraId: "",
  selectedEngine: "mediapipe_33",
  selectedVariant: MODEL_ENGINES.mediapipe_33.defaultVariant,

  setConnected: (isConnected) => set({ isConnected }),
  setFps: (fps) => set({ fps }),
  setSettingsOpen: (isSettingsOpen) => set({ isSettingsOpen }),
  setCameras: (cameras) => set({ cameras }),
  setSelectedCameraId: (selectedCameraId) => set({ selectedCameraId }),
  setModel: (selectedEngine, selectedVariant) =>
    set({ selectedEngine, selectedVariant }),
}));
