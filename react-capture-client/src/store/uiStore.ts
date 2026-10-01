import { create } from "zustand";

interface UIState {
  isConnected: boolean;
  setConnected: (status: boolean) => void;
}

export const useUIStore = create<UIState>((set) => ({
  isConnected: false,
  setConnected: (isConnected) => set({ isConnected }),
}));
