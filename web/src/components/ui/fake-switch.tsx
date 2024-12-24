import { useState } from "react";

type Props = {
  onClick?: () => unknown;
  onUnclick?: () => unknown;
};

export function FakeSwitch({ onClick, onUnclick }: Props) {
  const [isSwitching, setIsSwitching] = useState(false);
  const [isShaking, setIsShaking] = useState(false);

  const handleClick = () => {
    setIsSwitching(true);
    if (onClick) {
      onClick(); // Call the provided callback
    }
    setTimeout(() => {
      setIsSwitching(false);
      // Trigger shake animation after switching back
      setIsShaking(true);
      if (onUnclick) {
        onUnclick();
      }
      setTimeout(() => {
        setIsShaking(false); // Remove shake animation after it completes
      }, 500); // Duration of the shake animation
    }, 300); // Reset switching after 300ms
  };

  return (
    <div
      className={`relative w-12 h-6 bg-gray-300 rounded-full cursor-pointer transition-all duration-300 ${
        isShaking ? "animate-shake" : ""
      }`}
      onClick={handleClick}
    >
      <style>
        {`
        @keyframes shake {
        25% { transform: rotate(0.05turn) translateY(5px); }
        50% { transform: rotate(0.0turn) translateY(-5px); }
        75% { transform: rotate(0.05turn) translateY(5px); }
        }

        .animate-shake {
          animation: shake 0.5s ease-in-out;
        }
        `}
      </style>
      <div
        className={`absolute top-1/2 left-1 w-4 h-4 bg-white rounded-full shadow-md transition-transform duration-300 transform ${
          isSwitching
            ? "translate-x-6 -translate-y-1/2"
            : "translate-x-0 -translate-y-1/2"
        }`}
      ></div>
    </div>
  );
}

export default FakeSwitch;
