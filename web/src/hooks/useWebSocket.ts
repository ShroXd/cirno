import { useContext, useEffect, useRef } from 'react'
import { WebSocketContext } from '../contexts/webSocketContext'
import { WebSocketMessage } from '../bindings/WebSocketMessage'

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

    ws.onmessage = event => {
      resetHeartbeat()
      const message = JSON.parse(event.data) as WebSocketMessage
      console.log('Received message:', message)
      // TODO: handle heartbeat seperately
      // if (
      //   message.message_type === "Heartbeat" &&
      //   isHeartbeat(message.payload)
      // ) {
      //   if (message.payload.Heartbeat === "Ping") {
      //     ws.send(
      //       JSON.stringify({
      //         message_type: "HeartbeatResponse",
      //         payload: {
      //           HeartbeatResponse: "Pong",
      //         },
      //       })
      //     );
      //   } else if (message.payload.Heartbeat === "Pong") {
      //     console.log("Received HeartbeatResponse");
      //   } else {
      //     console.error("Received unknown Heartbeat message");
      //   }
      // } else if (isWebRtc(message.payload)) {
      //   eventBus.emit("webrtcMessage", message);
      // }
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

    reconnectTimer.current = setTimeout(() => {
      connect()
    }, interval)
  }

  const startHeartbeat = () => {
    heartbeatTimer.current = setInterval(() => {
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

  const resetHeartbeat = () => {
    stopHeartbeat()
    startHeartbeat()
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
