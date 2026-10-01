import {
  FilesetResolver,
  PoseLandmarker,
  DrawingUtils,
} from "@mediapipe/tasks-vision";
import { formatMediaPipeData } from "./adapter";
import { POSE_MODELS, type ModelType } from "./config";
import { SocketEngine } from "../network/SocketEngine";
import { useUIStore } from "../../store/uiStore";

export class CaptureEngine {
  private static landmarker: PoseLandmarker | null = null;
  private static active = false;
  private static animationFrameId: number;
  private static lastVideoTime = -1;

  private static framesThisSecond = 0;
  private static lastFpsUpdate = 0;

  static async start(
    video: HTMLVideoElement,
    canvas: HTMLCanvasElement,
    modelType: ModelType,
  ) {
    if (this.active) return;
    this.active = true;

    const vision = await FilesetResolver.forVisionTasks(
      "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@1.0.1/wasm",
    );

    this.landmarker = await PoseLandmarker.createFromOptions(vision, {
      baseOptions: {
        modelAssetPath: POSE_MODELS[modelType],
        delegate: "GPU",
      },
      runningMode: "VIDEO",
      numPoses: 1,
    });

    if (this.active) {
      await this.startCamera(video, canvas);
    }
  }

  private static async startCamera(
    video: HTMLVideoElement,
    canvas: HTMLCanvasElement,
  ) {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        video: { facingMode: "user" },
      });
      video.srcObject = stream;
      video.onloadedmetadata = () => {
        canvas.width = video.videoWidth;
        canvas.height = video.videoHeight;
        this.framesThisSecond = 0;
        this.lastFpsUpdate = Date.now();
        video.play();
        this.predictWebcam(video, canvas);
      };
    } catch (error) {
      console.error("Ошибка доступа к камере: ", error);
    }
  }

  private static predictWebcam = (
    video: HTMLVideoElement,
    canvas: HTMLCanvasElement,
  ) => {
    if (!this.active || !this.landmarker) return;

    const ctx = canvas.getContext("2d");
    if (ctx && video.currentTime !== this.lastVideoTime) {
      this.lastVideoTime = video.currentTime;
      const frameTimestamp = Date.now();
      const startTimeMs = performance.now();
      const results = this.landmarker.detectForVideo(video, startTimeMs);

      ctx.save();
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

      if (results.landmarks && results.landmarks.length > 0) {
        const frameData = formatMediaPipeData(
          results.landmarks[0],
          frameTimestamp,
        );

        SocketEngine.sendFrame(frameData);

        const drawingUtils = new DrawingUtils(ctx);
        for (const landmark of results.landmarks) {
          drawingUtils.drawConnectors(
            landmark,
            PoseLandmarker.POSE_CONNECTIONS,
            {
              color: "#00FF00",
              lineWidth: 5,
            },
          );
          drawingUtils.drawLandmarks(landmark, {
            color: "#FF0000",
            lineWidth: 2,
            radius: 5,
          });
        }
      }
      ctx.restore();

      this.framesThisSecond++;
      const elapsedMs = frameTimestamp - this.lastFpsUpdate;
      if (elapsedMs >= 200) {
        const currentFps = Math.round(
          (this.framesThisSecond / elapsedMs) * 1000,
        );
        useUIStore.getState().setFps(currentFps);

        this.framesThisSecond = 0;
        this.lastFpsUpdate = frameTimestamp;
      }
    }

    if (this.active) {
      this.animationFrameId = requestAnimationFrame(() =>
        this.predictWebcam(video, canvas),
      );
    }
  };

  static stop(video: HTMLVideoElement) {
    this.active = false;
    if (this.animationFrameId) cancelAnimationFrame(this.animationFrameId);
    if (video.srcObject) {
      (video.srcObject as MediaStream)
        .getTracks()
        .forEach((track) => track.stop());
    }
    if (this.landmarker) {
      this.landmarker.close();
      this.landmarker = null;
    }
    useUIStore.getState().setFps(0);
  }
}
