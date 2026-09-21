import { useEffect, useRef } from "react";

function App() {
  const videoRef = useRef<HTMLVideoElement>(null);

  useEffect(() => {
    navigator.mediaDevices
      .getUserMedia({ video: true })
      .then((stream) => {
        if (videoRef.current) {
          videoRef.current.srcObject = stream;
        }
      })
      .catch((err) => console.error("Ошибка доступа к камере:", err));
  }, []);

  return (
    <div>
      <h1>Motion Capture</h1>
      <video ref={videoRef} autoPlay playsInline style={{ width: "100%" }} />
    </div>
  );
}

export default App;
