import { useRef } from "react";
import { usePoseCapture } from "../../hooks/usePoseCapture";
import type { ModelType } from "../../core/mediapipe/config";
import type { MocapFrame } from "../../types";
import "./PoseCapture.css";

interface PoseCaptureProps {
  modelType?: ModelType;
  onFrame: (frame: MocapFrame) => void;
}

export default function PoseCapture({
  modelType = "lite",
  onFrame,
}: PoseCaptureProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  usePoseCapture(videoRef, canvasRef, modelType, onFrame);

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
