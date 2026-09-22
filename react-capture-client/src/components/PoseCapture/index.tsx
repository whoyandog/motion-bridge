import { useRef } from "react";
import { usePoseCapture } from "../../hooks/usePoseCapture";
import "./PoseCapture.css";

export default function PoseCapture() {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  usePoseCapture(videoRef, canvasRef);

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
