import { useContext, useEffect } from "react"
import { WebSocketContext } from "../contexts/webSocketContext"

export const useWebSocket = () => {
    const ws = useContext(WebSocketContext)
    if (!ws) {
        throw new Error("WebSocket is not connected")
    }

    const connect = () => {
        console.log("connecting to websocket")

        ws.onopen = () => {
            console.log("connected to websocket")
        }

        ws.onmessage = (message) => {
            console.log("message from websocket: ", message)
        }

        ws.onclose = () => {
            console.log("websocket closed")
        }

        ws.onerror = (error) => {
            console.log("websocket error: ", error)
        }
    }

    useEffect(() => {
        connect()
    }, [])

    const sendMessage = (message: string) => {
        if (ws.readyState === WebSocket.OPEN) {
            ws.send(message)
        } else {
            console.log("websocket is not open")
        }
    }

    return { sendMessage }
}