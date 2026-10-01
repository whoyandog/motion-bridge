import HUD from "./components/HUD";
import PoseCapture from "./components/PoseCapture";

function App() {
  return (
    <div>
      <HUD />
      <PoseCapture modelType="lite" />
    </div>
  );
}

export default App;
