import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { Button } from "@/components/ui/button"
import { ThemeProvider } from "@/components/theme-provider"
import { ModeToggle } from "@/components/mode-toggle"

function App() {
  const [count, setCount] = useState(0)

  return (
    <>
    <ThemeProvider>
    <ModeToggle />
        <p>{count}</p>
        <Button onClick={() => setCount(count + 1)}>click me</Button>
        </ThemeProvider>
    </>
  )
}

export default App
