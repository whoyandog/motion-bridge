import {
  FilesetResolver,
  PoseLandmarker,
  DrawingUtils,
  type NormalizedLandmark,
} from "@mediapipe/tasks-vision";
import type { IModelProvider, IRLandmark } from "../types";
import { WASM_BASE_URL, POSE_MODELS, type ModelType } from "./config";

export interface MediaPipeConfig {
  modelType: ModelType;
  numPoses?: number;
}

export class MediaPipeProvider implements IModelProvider {
  public readonly name = "mediapipe_33";

  private landmarker: PoseLandmarker | null = null;
  private lastRawLandmarks: NormalizedLandmark[] | null = null;
  private config: MediaPipeConfig;

  constructor(config: MediaPipeConfig) {
    this.config = {
      modelType: config.modelType,
      numPoses: config.numPoses ?? 1,
    };
  }

  async initialize(): Promise<void> {
    const vision = await FilesetResolver.forVisionTasks(WASM_BASE_URL);

    this.landmarker = await PoseLandmarker.createFromOptions(vision, {
      baseOptions: {
        modelAssetPath: POSE_MODELS[this.config.modelType],
        delegate: "GPU",
      },
      runningMode: "VIDEO",
      numPoses: this.config.numPoses,
    });
  }

  async predict(
    video: HTMLVideoElement,
    timestamp: number,
  ): Promise<IRLandmark[] | null> {
    if (!this.landmarker) return null;

    const results = this.landmarker.detectForVideo(video, timestamp);

    if (results.landmarks && results.landmarks.length > 0) {
      this.lastRawLandmarks = results.landmarks[0];

      return this.lastRawLandmarks.map((lm) => ({
        x: lm.x,
        y: lm.y,
        z: lm.z,
        visibility: lm.visibility ?? 1.0,
      }));
    }

    this.lastRawLandmarks = null;
    return null;
  }

  draw(ctx: CanvasRenderingContext2D): void {
    if (!this.lastRawLandmarks) return;

    const drawingUtils = new DrawingUtils(ctx);
    drawingUtils.drawConnectors(
      this.lastRawLandmarks,
      PoseLandmarker.POSE_CONNECTIONS,
      { color: "#00FF00", lineWidth: 5 },
    );
    drawingUtils.drawLandmarks(this.lastRawLandmarks, {
      color: "#FF0000",
      lineWidth: 2,
      radius: 5,
    });
  }

  async dispose(): Promise<void> {
    if (this.landmarker) {
      this.landmarker.close();
      this.landmarker = null;
    }
    this.lastRawLandmarks = null;
  }
}
