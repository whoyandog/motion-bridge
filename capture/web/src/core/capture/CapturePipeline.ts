import type { IModelProvider } from "../models/types";
import { SocketEngine } from "../network/SocketEngine";
import { PayloadPacker } from "../network/PayloadPacker";

export class CapturePipeline {
  private active = false;
  private animationFrameId?: number;
  private lastVideoTime = -1;

  private framesThisSecond = 0;
  private lastFpsUpdate = 0;

  private video: HTMLVideoElement;
  private canvas: HTMLCanvasElement;
  private model: IModelProvider;
  private onFpsUpdate?: (fps: number) => void;

  constructor(
    video: HTMLVideoElement,
    canvas: HTMLCanvasElement,
    model: IModelProvider,
    onFpsUpdate?: (fps: number) => void,
  ) {
    this.video = video;
    this.canvas = canvas;
    this.model = model;
    this.onFpsUpdate = onFpsUpdate;
  }

  public start() {
    if (this.active) return;
    this.active = true;
    this.lastFpsUpdate = Date.now();

    const controlPacket = PayloadPacker.createControlPacket(this.model.name);
    SocketEngine.sendControlPacket(controlPacket);

    this.loop();
  }

  public stop() {
    this.active = false;
    if (this.animationFrameId !== undefined) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = undefined;
    }
  }

  private loop = async () => {
    if (!this.active) return;

    if (this.video.currentTime !== this.lastVideoTime) {
      this.lastVideoTime = this.video.currentTime;

      const frameTimestamp = Date.now();

      const landmarks = await this.model.predict(this.video, performance.now());
      if (!this.active) return;

      if (landmarks) {
        const dataPacket = PayloadPacker.createDataPacket(
          landmarks,
          frameTimestamp,
        );
        SocketEngine.sendPacket(dataPacket);
      }

      const ctx = this.canvas.getContext("2d");
      if (ctx) {
        ctx.save();
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        ctx.drawImage(this.video, 0, 0, this.canvas.width, this.canvas.height);

        this.model.draw(ctx);
        ctx.restore();
      }

      this.framesThisSecond++;
      const now = Date.now();
      const elapsedMs = now - this.lastFpsUpdate;
      if (elapsedMs >= 200) {
        const currentFps = Math.round(
          (this.framesThisSecond / elapsedMs) * 1000,
        );
        this.onFpsUpdate?.(currentFps);

        this.framesThisSecond = 0;
        this.lastFpsUpdate = now;
      }
    }

    if (this.active) {
      this.animationFrameId = requestAnimationFrame(this.loop);
    }
  };
}
