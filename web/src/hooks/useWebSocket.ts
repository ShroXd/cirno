import { useContext, useEffect, useRef } from 'react'

import { WebSocketContext } from '~/contexts/WebSocketContext/WebSocketContext.tsx'
import { WebSocketMessage } from '~/bindings/WebSocketMessage'

export interface WebSocketOptions {
  reconnectInterval?: number
  maxReconnectInterval?: number
  heartbeatInterval?: number
  heartbeatMessage?: string
}

export const useWebSocket = (options: WebSocketOptions = {}) => {
  const ws = useContext(WebSocketContext)
  if (!ws) {
    throw new Error('useWebSocket must be used within a WebSocketProvider')
  }

  const reconnectTimer = useRef<number | null>(null)
  const heartbeatTimer = useRef<number | null>(null)
  const reconnectAttempts = useRef(0)

  const {
    reconnectInterval = 1000,
    maxReconnectInterval = 10000,
    heartbeatInterval = 10000,
  } = options

  const connect = () => {
    console.log('Connecting to WebSocket...')
    ws.onopen = () => {
      // TODO: pass useful log to backend
      console.log('WebSocket connection opened.')
      reconnectAttempts.current = 0
      startHeartbeat()
    }

    ws.onclose = () => {
      console.log('WebSocket connection closed.')
      stopHeartbeat()
      attemptReconnect()
    }

    ws.onerror = error => {
      console.error('WebSocket error:', error)
      stopHeartbeat()
      ws.close()
    }
  }

  const attemptReconnect = () => {
    const interval = Math.min(
      reconnectInterval * Math.pow(2, reconnectAttempts.current),
      maxReconnectInterval
    )
    reconnectAttempts.current += 1
    console.log(`Reconnecting in ${interval}ms...`)

    if (reconnectTimer.current) {
      clearTimeout(reconnectTimer.current)
    }
    reconnectTimer.current = window.setTimeout(() => {
      connect()
    }, interval)
  }

  const startHeartbeat = () => {
    heartbeatTimer.current = window.setInterval(() => {
      if (ws.readyState === WebSocket.OPEN) {
        console.log('Sending heartbeat message...')
        ws.send(
          JSON.stringify({
            message_type: 'Heartbeat',
            payload: {
              Heartbeat: 'Ping',
            },
          })
        )
      }
    }, heartbeatInterval)
  }

  const stopHeartbeat = () => {
    if (heartbeatTimer.current) {
      clearInterval(heartbeatTimer.current)
    }
  }

  useEffect(() => {
    connect()
    // TODO: handle cleanup
  }, [])

  const sendMessage = (message: WebSocketMessage) => {
    if (ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(message))
    } else {
      console.error('WebSocket is not open')
    }
  }

  return { sendMessage }
}
