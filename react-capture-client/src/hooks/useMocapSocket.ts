import { useEffect, useRef, useState } from "react";

export function useMocapSocket(url: string) {
  const wsRef = useRef<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    console.log(`Подключение к ${url}...`);
    const ws = new WebSocket(url);

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
      wsRef.current.send(JSON.stringify(frameData));
    }
  };

  return { sendFrame, isConnected };
}
