import { useEffect } from 'react'
import './App.css'
import { ThemeProvider } from "@/components/theme-provider"
import { Header } from "@/components/header"
import { BouncingBall } from "@/components/bouncing-ball"
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

function App() {
  useEffect(() => {runGame()}, [])

  return (
    <ThemeProvider defaultTheme="dark">
      <div className="flex flex-col items-center h-full relative">
        <Header />
        <BouncingBall className="absolute top-0 left-0 w-full h-full z-[-1]" />
        <canvas id="game" />
      </div>
    </ThemeProvider>
  )
}

export default App
