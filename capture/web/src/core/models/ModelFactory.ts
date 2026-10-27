import type { IModelProvider } from "./types";
import { MediaPipeProvider } from "./mediapipe/MediaPipeProvider";
import type { ModelType } from "./mediapipe/config";
import type { EngineId } from "./registry";

export class ModelFactory {
  static create(
    engineId: EngineId,
    variantId: string,
    numPoses: number = 1,
  ): IModelProvider {
    switch (engineId) {
      case "mediapipe_33":
        return new MediaPipeProvider({
          modelType: variantId as ModelType,
          numPoses: numPoses,
        });
      default:
        throw new Error(`Неизвестный движок: ${engineId}`);
    }
  }
}
