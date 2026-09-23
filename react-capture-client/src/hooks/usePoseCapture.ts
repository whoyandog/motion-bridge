import { useEffect, RefObject } from "react";
import {
  FilesetResolver,
  PoseLandmarker,
  DrawingUtils,
} from "@mediapipe/tasks-vision";
import { formatMediaPipeData } from "../core/mediapipe/adapter.ts";

export function usePoseCapture(
  videoRef: RefObject<HTMLVideoElement>,
  canvasRef: RefObject<HTMLCanvasElement>,
) {
  useEffect(() => {
    let animationFrameId: number;
    let active = true;
    let lastVideoTime = -1;
    let landmarker: PoseLandmarker | null = null;

    const initializeMediaPipe = async () => {
      const vision = await FilesetResolver.forVisionTasks(
        "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@1.0.1/wasm",
      );

      landmarker = await PoseLandmarker.createFromOptions(vision, {
        baseOptions: {
          modelAssetPath:
            "https://storage.googleapis.com/mediapipe-models/pose_landmarker/pose_landmarker_lite/float16/1/pose_landmarker_lite.task",
          delegate: "GPU",
        },
        runningMode: "VIDEO",
        numPoses: 1,
      });

      if (active) startCamera();
    };

    const startCamera = async () => {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({
          video: { facingMode: "user" },
        });
        if (videoRef.current) {
          videoRef.current.srcObject = stream;
          videoRef.current.onloadedmetadata = () => {
            if (canvasRef.current && videoRef.current) {
              canvasRef.current.width = videoRef.current.videoWidth;
              canvasRef.current.height = videoRef.current.videoHeight;
            }
            videoRef.current?.play();
            predictWebcam();
          };
        }
      } catch (error) {
        console.error("Ошибка доступа к камере:", error);
      }
    };

    const predictWebcam = () => {
      const video = videoRef.current;
      const canvas = canvasRef.current;

      if (!video || !canvas || !landmarker) return;

      const ctx = canvas.getContext("2d");
      if (!ctx) return;

      if (video.currentTime !== lastVideoTime) {
        lastVideoTime = video.currentTime;

        const drawingUtils = new DrawingUtils(ctx);
        const startTimeMs = performance.now();

        const results = landmarker.detectForVideo(video, startTimeMs);

        ctx.save();
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(video, 0, 0, canvas.width, canvas.height);

        if (results.landmarks && results.landmarks.length > 0) {
          const frameData = formatMediaPipeData(results.landmarks[0]);
          if (Math.random() < 0.02) {
            console.log("Готово к отправке: ", frameData);
          }
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
      }

      if (active) {
        animationFrameId = requestAnimationFrame(predictWebcam);
      }
    };

    initializeMediaPipe();

    return () => {
      active = false;
      if (animationFrameId) cancelAnimationFrame(animationFrameId);

      if (videoRef.current?.srcObject) {
        const stream = videoRef.current.srcObject as MediaStream;
        stream.getTracks().forEach((track) => track.stop());
      }

      if (landmarker) {
        landmarker.close();
      }
    };
  }, []);
}
