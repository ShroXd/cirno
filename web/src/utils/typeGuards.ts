import { Notification } from '../bindings/Notification'
import { WebSocketEventType } from '../bindings/WebSocketEventType'

export const isWebSocketEventType = (
  payload: unknown
): payload is WebSocketEventType =>
  typeof payload === 'object' && payload !== null && 'RegisterClient' in payload

export const isNotification = (
  payload: unknown
): payload is Notification<unknown> =>
  typeof payload === 'object' &&
  payload !== null &&
  'event' in payload &&
  'payload' in payload
