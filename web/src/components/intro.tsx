import { useState, useEffect } from "react";
import sphereImage from "@/assets/sphere.svg";
import { Game } from "@/components/game";

export const Intro = () => {
  const introMessages = [
    "WELCOME 2 BOX LAND",
    "I AM A BOX",
    "YOU ARE A BOX",
    "WHAT IS YOUR NAME?",
  ];

  const [messages, setMessages] = useState(introMessages);

  const [name, setName] = useState(" the Box");
  const [enterVisible, setEnterVisible] = useState(false);
  const [initWebsocket, setInitWebsocket] = useState(false);
  const [websocket, setWebsocket] = useState<null | WebSocket>(null);
  const [showGame, setShowGame] = useState(false);

  const fixedName = (name.toUpperCase().startsWith("D") ? `DR. ${name}` : name)
    .replace("snowball", "dr. snowball")
    .replace("Snowball", "Dr. Snowball")
    .replace("SNOWBALL", "DR. SNOWBALL");

  useEffect(() => {
    if (initWebsocket) {
      const url = new URL(`./ws/${fixedName}`, location.href);
      url.protocol = url.protocol.replace("http", "ws");
      const socket = new WebSocket(url);
      socket.onopen = () => {
        setWebsocket(socket);
        setShowGame(true);
      };
      socket.onerror = (event) => {
        console.error("WebSocket error:", event);
        setTimeout(() => {
          setMessages([
            `I AM ${name.toUpperCase()}`,
            `YOU ARE NOT ${name.toUpperCase()}`,
            `ENTER YOUR REAL NAME`,
            "OR AT LEAST ONE THAT IS NOT MINE",
          ]);
        }, 2000);
      };
      setInitWebsocket(false);
    }
  }, [name, fixedName, initWebsocket]);

  useEffect(() => {
    messages.forEach((_, index) => {
      const element = document.getElementById(`message-${index}`);
      if (element) {
        element.classList.remove("opacity-100");
        element.classList.add("opacity-0");
      }
      setTimeout(() => {
        const element = document.getElementById(`message-${index}`);
        if (element) {
          element.classList.remove("opacity-0");
          element.classList.add("opacity-100");
        }
      }, index * 2000);
    });

    const inputElement = document.getElementById("nameInputContainer");
    if (inputElement) {
      setTimeout(() => {
        inputElement.classList.remove("opacity-0");
        inputElement.classList.add("opacity-100");
        const input = document.getElementById("nameInput");
        if (input) {
          input.focus();
          try {
            //@ts-expect-error it's fine if this isn't supported
            input.setSelectionRange(0, 0);
          } catch {
            // ignored
          }
        }
      }, messages.length * 2000);
    }
  }, [messages]);

  const handleEnter = () => {
    setEnterVisible(false);
    setInitWebsocket(true);
    messages.forEach((_, index) => {
      const element = document.getElementById(`message-${index}`);
      if (element) {
        element.classList.remove("opacity-100");
        element.classList.add("opacity-0");
      }
    });

    const inputElement = document.getElementById("nameInputContainer");
    if (inputElement) {
      inputElement.classList.remove("opacity-100");
      inputElement.classList.add("opacity-0");
    }
  };

  const introShown = showGame ? "hidden" : "";
  return (
    <>
      <Game visible={showGame} websocket={websocket} username={fixedName} />
      <div
        className={`h-full w-full flex flex-col justify-center items-center bg-gradient-to-b from-yellow-500 to-pink-800 text-center space-y-6 ${introShown}`}
      >
        {messages.map((msg, index) => (
          <div
            id={`message-${index}`}
            key={index}
            className={`text-3xl font-extrabold text-shadow-md tracking-widest p-3 bg-green-${Math.min(900, (index + 1) * 200)} text-white transform transition-opacity duration-1000 opacity-0`}
          >
            {msg}
          </div>
        ))}
        <div
          id="nameInputContainer"
          className="bg-white w-96 h-64 rounded-3xl overflow-hidden flex flex-col opacity-0 transition-opacity duration-1000"
        >
          <div className="bg-red-600 w-full h-26 text-7xl  font-extrabold text-shadow-md text-white">
            Hello
          </div>

          <div className="bg-red-600 w-full h-8 text-2xl font-extrabold text-shadow-md tracking-widest text-white">
            my name is
          </div>
          <div className="bg-white w-full h-32">
            <input
              id="nameInput"
              className="p-4 focus:outline-none text-center text-3xl font-comic font-extrabold w-full h-full text-black"
              type="text"
              value={name}
              onKeyDown={(event) => {
                if (event.key === "Enter") {
                  handleEnter();
                }
              }}
              onChange={(event) => {
                setName(event.target.value);
                setEnterVisible(true);
              }}
              style={{ caretColor: "purple" }}
            />
          </div>
          <div className="bg-red-600 w-full h-8 w-full"></div>
        </div>
        <div
          className={`relative inline-block cursor-pointer ${enterVisible ? "" : "invisible"}`}
          onClick={handleEnter}
        >
          <img className="h-32" src={sphereImage} />
          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-red-600 text-lg font-bold">
            ENTER
          </div>
        </div>
      </div>
    </>
  );
};
