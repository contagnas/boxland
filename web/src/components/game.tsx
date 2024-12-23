import { useEffect, useState } from 'react';
import init from "@/game/game"

async function runGame() {
  try {
    await init();
  } catch (e) {
    const error = e as Error;
    if (error.message.includes("Using exceptions for control flow")) {
      return;
    }
    console.error("Failed to start game", error);
  }
}

export const Game = () => {
  useEffect(() => {runGame()}, [])

  const [messages, setMessages] = useState<string[]>([]);
  const [ws, setWs] = useState<WebSocket | null>(null);

  // Register the function on the window object
  window.publish_event = (message: string) => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(message)
    }
  };


  useEffect(() => {
    const url = new URL('./ws', location.href);
    url.protocol = url.protocol.replace('http', 'ws');
    const socket = new WebSocket(url);

    socket.onopen = () => {
      console.log('WebSocket connected');
      // Optionally, send an initial message
      // socket.send(JSON.stringify({ type: 'greet', message: 'Hello!' }));
    };

    socket.onmessage = (event) => {
      console.log('Message from server: ', event.data);
      setMessages((prevMessages) => [...prevMessages, event.data]);
    };

    socket.onerror = (error) => {
      console.error('WebSocket error: ', error);
    };

    socket.onclose = () => {
      console.log('WebSocket disconnected');
    };

    // Save the WebSocket instance for sending messages later
    setWs(socket);

    setInterval(() => {
      if (socket.readyState === WebSocket.OPEN) {
        socket.send(JSON.stringify({ message: 'Ping', timestamp: Date.now() }));
      }
    }, 1000); // S

    // Cleanup on unmount
    return () => {
      socket.close();
    };
  }, []);

  const sendMessage = () => {
    if (ws) {
      ws.send('Hello, server!');
    }
  };


  return (<canvas id="game" className="h-full w-full" />);
}
