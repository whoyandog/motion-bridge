import { useEffect, useRef, useState } from "react";
import { MocapFrame } from "../types";

export function useMocapSocket(url: string) {
  const wsRef = useRef<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    console.log(`Подключение к ${url}...`);
    const ws = new WebSocket(url);
    ws.binaryType = "arraybuffer";

    ws.onopen = () => {
      console.log("WS: connected");
      setIsConnected(true);
    };

    ws.onclose = () => {
      console.log("WS: disconnected");
      setIsConnected(true);
    };

    ws.onerror = (error) => {
      console.log("WS: error");
      setIsConnected(false);
    };

    wsRef.current = ws;

    return () => {
      if (
        ws.readyState === WebSocket.OPEN ||
        ws.readyState === WebSocket.CONNECTING
      ) {
        ws.close();
      }
    };
  }, [url]);

  const sendFrame = (frameData: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      const bytes = MocapFrame.encode(frameData).finish();
      wsRef.current.send(bytes);
    }
  };

  return { sendFrame, isConnected };
}
