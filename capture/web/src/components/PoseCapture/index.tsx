import { useEffect, useRef } from "react";
import { CameraManager } from "../../core/capture/CameraManager";
import { CapturePipeline } from "../../core/capture/CapturePipeline";
import { ModelFactory } from "../../core/models/ModelFactory";
import type { IModelProvider } from "../../core/models/types";
import { SocketEngine } from "../../core/network/SocketEngine";
import { useUIStore } from "../../store/uiStore";
import "./PoseCapture.css";

export default function PoseCapture() {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const setFps = useUIStore((state) => state.setFps);
  const setCameras = useUIStore((state) => state.setCameras);
  const selectedCameraId = useUIStore((state) => state.selectedCameraId);
  const setSelectedCameraId = useUIStore((state) => state.setSelectedCameraId);
  const selectedEngine = useUIStore((state) => state.selectedEngine);
  const selectedVariant = useUIStore((state) => state.selectedVariant);

  useEffect(() => {
    SocketEngine.connect("wss://192.168.88.24:8443/ws");

    CameraManager.getAvailableCameras().then((devices) => {
      setCameras(devices);
      if (devices.length > 0 && !useUIStore.getState().selectedCameraId) {
        setSelectedCameraId(devices[0].deviceId);
      }
    });

    return () => {
      SocketEngine.disconnect();
    };
  }, [setCameras, setSelectedCameraId]);

  useEffect(() => {
    const videoEl = videoRef.current;
    const canvasEl = canvasRef.current;
    if (!videoEl || !canvasEl || !selectedCameraId) return;

    let pipeline: CapturePipeline | null = null;
    let provider: IModelProvider | null = null;
    let isCancelled = false;

    const initCapture = async () => {
      try {
        await CameraManager.start(videoEl, selectedCameraId);
        if (isCancelled) {
          CameraManager.stop(videoEl);
          return;
        }

        canvasEl.width = videoEl.videoWidth;
        canvasEl.height = videoEl.videoHeight;

        provider = ModelFactory.create(selectedEngine, selectedVariant);
        await provider.initialize();

        if (isCancelled) {
          await provider.dispose();
          CameraManager.stop(videoEl);
          return;
        }

        pipeline = new CapturePipeline(videoEl, canvasEl, provider, (fps) =>
          setFps(fps),
        );
        pipeline.start();
      } catch (error) {
        console.error("Ошибка инициализации: ", error);
      }
    };
    initCapture();

    return () => {
      isCancelled = true;
      if (pipeline) pipeline.stop();
      if (provider) void provider.dispose();
      CameraManager.stop(videoEl);
      setFps(0);
    };
  }, [selectedEngine, selectedVariant, selectedCameraId, setFps]);

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
