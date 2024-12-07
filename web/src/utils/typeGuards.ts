import { RegisterClient } from '../bindings/RegisterClient'
import { Notification as NotificationType } from '../bindings/Notification'
import { EventMessage } from '../bindings/EventMessage'

export const isEventMessage = (
  data: unknown
): data is EventMessage<unknown> => {
  return typeof data === 'object' && data !== null && 'event' in data
}

export const isRegisterClientPayload = (
  payload: unknown
): payload is RegisterClient => {
  return typeof payload === 'object' && payload !== null && 'key' in payload
}

export const isMediaLibraryScannedPayload = (
  payload: unknown
): payload is NotificationType['MediaLibraryScanned'] => {
  return typeof payload === 'object' && payload !== null && 'id' in payload
}
