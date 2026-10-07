import { useUIStore } from "../../store/uiStore";
import { CaptureEventPacket } from "../../types/schema";

export class SocketEngine {
  private static ws: WebSocket | null = null;

  static connect(url: string) {
    if (this.ws?.readyState === WebSocket.OPEN) return;

    console.log(`Подключение к ${url}...`);
    this.ws = new WebSocket(url);
    this.ws.binaryType = "arraybuffer";

    this.ws.onopen = () => {
      console.log("WS: connected");
      useUIStore.getState().setConnected(true);

      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        const handshakeMsg = CaptureEventPacket.create({
          handshake: { modelName: "mediapipe_33" },
        });
        const bytes = CaptureEventPacket.encode(handshakeMsg).finish();
        this.ws.send(bytes);
        console.log("WS: Handshake sent");
      }
    };

    this.ws.onclose = () => {
      console.log("WS: disconnected");
      useUIStore.getState().setConnected(false);
    };

    this.ws.onerror = (e) => {
      console.error("WS: error. ", e);
      useUIStore.getState().setConnected(false);
    };
  }

  static sendFrame(frameData: CaptureEventPacket) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      const clientMsg = CaptureEventPacket.create(frameData);
      const bytes = CaptureEventPacket.encode(clientMsg).finish();
      this.ws.send(bytes);
    }
  }

  static disconnect() {
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}
