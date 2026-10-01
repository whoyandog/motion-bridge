import { useUIStore } from "../../store/uiStore";
import { MocapFrame } from "../../types/schema";

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

  static sendFrame(frameData: any) {
    if (this.ws && this.ws.readyState === WebSocket.OPEN) {
      const bytes = MocapFrame.encode(frameData).finish();
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
