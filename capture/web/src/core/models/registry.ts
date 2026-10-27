export const MODEL_ENGINES = {
  mediapipe_33: {
    id: "mediapipe_33",
    label: "MediaPipe Pose 33",
    variants: [
      { id: "lite", label: "Lite" },
      { id: "full", label: "Full" },
      { id: "heavy", label: "Heavy" },
    ],
    defaultVariant: "lite",
  },
} as const;

export type EngineId = keyof typeof MODEL_ENGINES;
