import { useUIStore } from "../../store/uiStore";
import { CaptureEventPacket } from "../../types/schema";

export class SocketEngine {
  private static ws: WebSocket | null = null;
  private static lastControlPacket: CaptureEventPacket | null = null;

  static connect(url: string) {
    if (
      this.ws?.readyState === WebSocket.OPEN ||
      this.ws?.readyState === WebSocket.CONNECTING
    ) {
      return;
    }

    console.log(`Подключение к ${url}...`);
    this.ws = new WebSocket(url);
    this.ws.binaryType = "arraybuffer";

    this.ws.onopen = () => {
      console.log("WS: connected");
      useUIStore.getState().setConnected(true);

      if (this.lastControlPacket) {
        this.sendPacket(this.lastControlPacket);
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

  static sendControlPacket(controlPacket: CaptureEventPacket) {
    this.lastControlPacket = controlPacket;
    this.sendPacket(controlPacket);
  }

  static sendPacket(packet: CaptureEventPacket) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      const bytes = CaptureEventPacket.encode(packet).finish();
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
