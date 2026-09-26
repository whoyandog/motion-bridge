import { useRef } from "react";
import { usePoseCapture } from "../../hooks/usePoseCapture";
import type { ModelType } from "../../core/mediapipe/config.ts";
import "./PoseCapture.css";

export default function PoseCapture() {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const model: ModelType = "lite";

  usePoseCapture(videoRef, canvasRef, model);

  return (
    <div className="capture-container">
      <video
        ref={videoRef}
        autoPlay
        playsInline
        muted
        className="capture-video"
      />
      <canvas ref={canvasRef} className="capture-canvas" />
    </div>
  );
}
