import { Notification } from '@/bindings/Notification'

export const isNotification = (
  payload: unknown
): payload is Notification<unknown> =>
  typeof payload === 'object' &&
  payload !== null &&
  'notification_type' in payload &&
  'payload' in payload
