import { create } from "zustand";

interface UIState {
  isConnected: boolean;
  fps: number;
  setConnected: (status: boolean) => void;
  setFps: (fps: number) => void;
}

export const useUIStore = create<UIState>((set) => ({
  isConnected: false,
  fps: 0,
  setConnected: (isConnected) => set({ isConnected }),
  setFps: (fps) => set({ fps }),
}));
