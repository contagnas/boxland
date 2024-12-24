import { useEffect } from 'react';
import init, { send_event, run } from "@/game/game"

let engine: any | null = null;
let engineIniting = false;

const initEngine = async () => {
  if (!engine && !engineIniting) {
    engine = await init()
  }
  return engine
}

async function runGame(username: string) {
  try {
    console.log('passing username to run', username)
    run(username)
  } catch (e) {
    const error = e as Error;
    if (error.message.includes("Using exceptions for control flow")) {
      return;
    }
    console.error("Failed to start game", error);
  }
}

export type GameProps = {
  websocket: WebSocket | null
  visible?: Boolean
  username: string
}

export const Game = ({ websocket, visible, username }: GameProps) => {
  useEffect(() => {
    initEngine()
  }, [])

  useEffect(() => {
    if (engine && visible) {
      console.log(engine)
      runGame(username)
    }
  }, [engine, visible])

  useEffect(() => {

    if (websocket) {
      websocket.onmessage = (event) => {
        send_event(event.data)
      };

      // Register the function on the window object
      // @ts-ignore
      window.publish_event = (message: string) => {
        if (websocket.readyState === WebSocket.OPEN) {
          websocket.send(message)
        }
      };
    }
  }, [websocket])

  return (<canvas id="game" className={`h-full w-full ${visible ? "" : "hidden"}`} />);
}
