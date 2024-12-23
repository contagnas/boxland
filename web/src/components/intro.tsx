import { useState, useEffect } from 'react';
import sphereImage from '../assets/sphere.svg'

export const Intro = () => {
  const introMessages = [
    "WELCOME 2 BOX LAND",
    "I AM A BOX",
    "YOU ARE A BOX",
    "WHAT IS YOUR NAME?",
  ];

  const [messages, setMessages] = useState(introMessages)

  const [name, setName] = useState(" the Box")
  const [enterVisible, setEnterVisible] = useState(false)

  useEffect(() => {
    messages.forEach((_, index) => {
      setTimeout(() => {
        const element = document.getElementById(`message-${index}`);
        if (element) {
          element.classList.remove('opacity-0');
          element.classList.add('opacity-100');
        }
      }, index * 2000);
    });

    const inputElement = document.getElementById('nameInputContainer');
    if (inputElement) {
      setTimeout(() => {
        inputElement.classList.remove('opacity-0');
        inputElement.classList.add('opacity-100');
        const input = document.getElementById("nameInput")
        if (input) {
          input.focus()
          input.setSelectionRange(0, 0)
        }
      }, messages.length * 2000);
    }
  }, [messages]);

  return (
    <div className="h-full w-full flex flex-col justify-center items-center bg-gradient-to-b from-yellow-500 to-pink-800 text-center space-y-6">
      {messages.map((msg, index) => (
        <div
          id={`message-${index}`}
          key={index}
          className={`text-3xl font-extrabold text-shadow-md tracking-widest p-3 bg-green-${Math.min(900, (index + 1) * 200)} text-white transform transition-opacity duration-1000 opacity-0`}
        >
          {msg}
        </div>
      ))}
      <div id="nameInputContainer" className="bg-white w-96 h-64 rounded-3xl overflow-hidden flex flex-col opacity-0">
        <div className="bg-red-600 w-full h-26 text-7xl  font-extrabold text-shadow-md text-white">
          Hello
        </div>

        <div className="bg-red-600 w-full h-8 text-2xl font-extrabold text-shadow-md tracking-widest text-white">
          my name is
        </div>
        <div className="bg-white w-full h-32">
          <input id="nameInput"
            className="p-4 focus:outline-none text-center text-3xl font-comic font-extrabold w-full h-full text-black"
            type="text"
            value={name}
            onChange={(event) => {
              setName(event.target.value)
              setEnterVisible(true)
            }}
            style={{ caretColor: "purple" }}
          />
        </div>
        <div className="bg-red-600 w-full h-8 w-full">
        </div>
      </div>
      <a href="zootopia.com">
        <div className={`relative inline-block ${enterVisible? "": "invisible"}`}>
          <img className="h-32" src={sphereImage} />
          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 text-red-600 text-lg font-bold">
            ENTER
          </div>
        </div>
      </a>
    </div>
  );
};



/*
      <div className="flex flex-col items-center bg-white border-red-600 rounded-lg shadow-md p-6 transition-opacity duration-1000" id="nameInputContainer">
        <div className="text-white w-full bg-red-600 font-bold text-2xl -mt-6 -ml-6 -mr-6">Hello</div>
        <div className="text-red-600 text-xl mb-4">my name is</div>
        <input
          id="nameInput"
          type="text"
          className="w-full h-12 border-2 border-red-600 rounded px-4 text-xl text-gray-800 focus:outline-none focus:ring-2 focus:ring-red-600"
        />
      </div>
 */
