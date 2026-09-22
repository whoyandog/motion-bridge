import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import PoseCapture from "./components/PoseCapture";
import HUD from "./components/HUD";
import "./index.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <div>
      <HUD />
      <PoseCapture />
    </div>
  </StrictMode>,
);
