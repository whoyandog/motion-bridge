import type { IRLandmark } from "../models/types";
import { CaptureEventPacket } from "../../types/schema";

export class PayloadPacker {
  static flattenLandmarks(landmarks: IRLandmark[]): number[] {
    const flatArray: number[] = [];
    for (const lm of landmarks) {
      flatArray.push(lm.x, lm.y, lm.z, lm.visibility ?? 1.0);
    }

    return flatArray;
  }

  static createControlPacket(modelName: string): CaptureEventPacket {
    return CaptureEventPacket.create({
      timestamp: Date.now(),
      actorId: 0,
      handshake: { modelName },
    });
  }

  static createDataPacket(
    landmarks: IRLandmark[],
    timestamp: number,
    actorId: number = 1,
  ): CaptureEventPacket {
    return CaptureEventPacket.create({
      timestamp,
      actorId,
      body: {
        landmarksData: this.flattenLandmarks(landmarks),
      },
    });
  }
}
