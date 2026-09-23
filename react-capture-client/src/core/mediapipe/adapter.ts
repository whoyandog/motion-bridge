import type { MocapFrame } from "../types";
import type { NormalizedLandmark } from "@mediapipe/tasks-vision";

export function formatMediaPipeData(
  rawLandmarks: NormalizedLandmark[],
): MocapFrame {
  return {
    timestamp: Date.now(),
    skeleton_type: "mediapipe_33",
    landmarks: rawLandmarks.map((lm, index) => ({
      id: index,
      x: lm.x,
      y: lm.y,
      z: lm.z,
      visibility: lm.visibility ?? 1.0,
    })),
  };
}
