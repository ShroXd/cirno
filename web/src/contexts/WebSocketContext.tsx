import { createContext, FC } from 'react'

import { useEventBus } from '../hooks/useEventBus'
import { isEventMessage } from '../utils/typeGuards'

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
  const { sendMessage } = useEventBus()

  ws.onmessage = event => {
    const data = JSON.parse(event.data)
    console.log('WebSocket received message:', data)

    // NOTE: rs_ts converts Rust enums into string literal union types in TypeScript.
    // This makes it difficult to ensure type safety and clean code when using these types directly.
    // To solve this, we use type guards to enable proper type inference in the event handling process.
    if (!isEventMessage(data)) {
      return
    }
    switch (data.event) {
      case 'RegisterClient':
        sendMessage({
          event: 'RegisterClient',
          payload: data.payload,
        })
        break
    }
  }

  return (
    <WebSocketContext.Provider value={ws}>{children}</WebSocketContext.Provider>
  )
}
