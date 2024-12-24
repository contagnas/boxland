import { FakeSwitch } from "@/components/ui/fake-switch";
import { Toaster } from "@/components/ui/sonner";
import { toast } from "sonner";
import { useTheme } from "@/components/theme-provider";
// just build-wasm
import { send_event } from "@/game/game";

function SunIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 64 64"
      width="20"
      height="20"
      fill="none"
    >
      {/* Sun Core */}
      <circle cx="32" cy="32" r="14" fill="#00FF00" />

      {/* Sun Rays */}
      {Array.from({ length: 8 }).map((_, i) => (
        <rect
          key={i}
          x="30"
          y="2"
          width="4"
          height="12"
          fill="cyan"
          transform={`rotate(${i * 45} 32 32)`}
        />
      ))}
    </svg>
  );
}

export function Header() {
  const { setTheme } = useTheme();
  return (
    <header className="w-full flex justify-between items-center px-4 py-1 bg-[#FF00FF] text-[#00FFFF] border-8 border-dotted border-transparent animate-border-dance">
      {/* Left Side: Marquee-like Text */}
      <div className="flex items-center">
        <h1 className="text-4xl font-extrabold italic underline tracking-widest">
          BOX LAND
        </h1>
      </div>

      {/* Right Side: Giant, Flashy Switch */}
      <div className="relative bg-black p-4 border-4 border-dashed border-cyan shadow-[0_0_10px_magenta] animate-glow flex gap-2 items-center">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 64 64"
          width="20"
          height="20"
          fill="none"
        >
          {/* Circle for the moon */}
          <circle cx="32" cy="32" r="30" fill="red" />
          {/* Cut-out to make it crescent */}
          <circle cx="40" cy="32" r="25" fill="black" />
        </svg>
        <FakeSwitch
          onClick={() => {
            toast("A mysterious force prevents light mode!");
            setTheme("light");
            send_event("light");
          }}
          onUnclick={() => {
            setTheme("dark");
            send_event("dark");
          }}
        />
        <SunIcon />

        <Toaster />
      </div>

      {/* Custom CSS for Animations */}
      <style>
        {`
        @keyframes border-dance {
        0% { border-color: red; }
        25% { border-color: yellow; }
        50% { border-color: green; }
        75% { border-color: blue; }
        100% { border-color: red; }
        }

        @keyframes glow {
        from { box-shadow: 0 0 10px white; }
        to { box-shadow: 0 0 30px cyan; }
        }

        .animate-border-dance {
        animation: border-dance 2s infinite linear;
        }

        .animate-glow {
        animation: glow 1s infinite alternate;
        }
      `}
      </style>
    </header>
  );
}
