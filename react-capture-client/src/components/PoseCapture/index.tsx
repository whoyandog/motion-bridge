import { useEffect, useRef } from "react";
import { CaptureEngine } from "../../core/mediapipe/CaptureEngine";
import { SocketEngine } from "../../core/network/SocketEngine";
import type { ModelType } from "../../core/mediapipe/config";
import "./PoseCapture.css";

interface PoseCaptureProps {
  modelType?: ModelType;
}

export default function PoseCapture({ modelType = "lite" }: PoseCaptureProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    SocketEngine.connect("wss://192.168.88.24:8443/ws");

    if (videoRef.current && canvasRef.current) {
      CaptureEngine.start(videoRef.current, canvasRef.current, modelType);
    }

    return () => {
      if (videoRef.current) CaptureEngine.stop(videoRef.current);
      SocketEngine.disconnect();
    };
  }, [modelType]);

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
