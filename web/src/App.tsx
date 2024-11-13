import { HashRouter } from 'react-router-dom'
import { Home } from './pages/Home/Home'
import { WebSocketProvider } from './contexts/WebSocketContext.tsx'

function App() {
  return (
    <WebSocketProvider>
      <HashRouter>
        <Home />
      </HashRouter>
    </WebSocketProvider>
  )
}

export default App
