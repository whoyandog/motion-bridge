import type { BridgeCaptureEvent } from "../../types/schema";
import type { NormalizedLandmark } from "@mediapipe/tasks-vision";

export function formatMediaPipeData(
  rawLandmarks: NormalizedLandmark[],
  timestamp: number,
): CaptureEventPacket {
  const flatArray: number[] = [];
  for (const lm of rawLandmarks) {
    flatArray.push(lm.x, lm.y, lm.z, lm.visibility ?? 1.0);
  }

  return {
    timestamp,
    actorId: 1,
    body: {
      landmarksData: flatArray,
    },
  };
}
