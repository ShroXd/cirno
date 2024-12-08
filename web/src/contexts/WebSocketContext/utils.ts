import { Notification } from '../../bindings/Notification'

export const isNotification = (
  payload: unknown
): payload is Notification<unknown> =>
  typeof payload === 'object' &&
  payload !== null &&
  'event' in payload &&
  'payload' in payload
