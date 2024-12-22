import { useState } from 'react'
import './App.css'
import { Button } from "@/components/ui/button"
import { ThemeProvider } from "@/components/theme-provider"
import { Header } from "@/components/header"
import { BouncingBall } from "@/components/bouncing-ball"

function App() {
  const [count, setCount] = useState(0)

  return (
    <ThemeProvider className="relative">
      <div className="flex flex-col items-center h-full relative">
        <Header />
        <BouncingBall className="absolute top-0 left-0 w-full h-full z-[-1]" />
        <p>{count}</p>
        <Button onClick={() => setCount(count + 1)}>click me</Button>
      </div>
    </ThemeProvider>
  )
}

export default App
