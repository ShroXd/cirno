import { createContext, FC } from 'react'

// NOTE: During development mode, React renders components twice for strict mode enforcement.
// This can lead to the creation of multiple WebSocket clients, causing issues with backend negotiation.
// TODO: Optimize this code to ensure proper WebSocket cleanup when the context is updated.
// For now, we're initializing the WebSocket outside the context to prevent duplicate connections.
const ws = new WebSocket('ws://localhost:8000/ws')
export const WebSocketContext = createContext<WebSocket | null>(ws)

interface WebSocketProviderProps {
  children: React.ReactNode
  url?: string
}

export const WebSocketProvider: FC<WebSocketProviderProps> = ({ children }) => {
  return (
    <WebSocketContext.Provider value={ws}>{children}</WebSocketContext.Provider>
  )
}
