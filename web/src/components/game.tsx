import { useEffect, useState } from 'react';
import init, { InitOutput, send_event } from "@/game/game"

let engine: any | null = null;
let engineIniting = false;

const initEngine = async () => {
  if (!engine && !engineIniting) {
    engine = await init()
  }
  return engine
}

async function runGame(engine: InitOutput) {
  try {
    engine.run()
  } catch (e) {
    const error = e as Error;
    if (error.message.includes("Using exceptions for control flow")) {
      return;
    }
    console.error("Failed to start game", error);
  }
}

export type GameProps = {
  websocket: WebSocket | null,
  visible?: Boolean
}

export const Game = ({ websocket, visible }: GameProps) => {
  useEffect(() => {
    initEngine()
  }, [])

  useEffect(() => {
    if (engine && visible) {
      console.log(engine)
      runGame(engine)
    }
  }, [engine, visible])

  useEffect(() => {

    if (websocket) {
      websocket.onmessage = (event) => {
        console.log('Message from server: ', event.data);
        send_event(event.data)
      };

      // Register the function on the window object
      window.publish_event = (message: string) => {
        if (websocket.readyState === WebSocket.OPEN) {
          websocket.send(message)
        }
      };
    }
  }, [websocket])

  return (<canvas id="game" className={`h-full w-full ${visible ? "" : "hidden"}`} />);
}
