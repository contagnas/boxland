import { useEffect } from 'react'
import './App.css'
import { ThemeProvider } from "@/components/theme-provider"
import { Header } from "@/components/header"
import { BouncingBall } from "@/components/bouncing-ball"
import { Game } from "@/components/game"

function App() {

  return (
    <ThemeProvider defaultTheme="dark">
      <div className="flex flex-col items-center h-full relative">
        <Header />
        <BouncingBall className="absolute top-0 left-0 w-full h-full z-[-1]" />
        <Game />
      </div>
    </ThemeProvider>
  )
}

export default App
