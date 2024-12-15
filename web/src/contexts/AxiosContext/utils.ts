import { WebSocketEventType } from '@/bindings/WebSocketEventType'

export const isWebSocketEventType = (
  payload: unknown
): payload is WebSocketEventType =>
  typeof payload === 'object' && payload !== null && 'RegisterClient' in payload
