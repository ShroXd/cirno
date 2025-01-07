import { NotificationType } from '~/bindings/NotificationType'
import { LibraryManageDialogPayloadMap } from '~/components/LibraryManageDialog/event'

interface NotificationPayloadMap
  extends Partial<Record<NotificationType, unknown>> {
  LibrarySaved: {
    libraryId: number
  }
  RegisterClient: {
    clientKey: string
  }
  Error: {
    title: string
    message: string
    tryAgain?: () => void
  }
}

export enum VideoPlayerEventType {
  Play = 'Play',
  Stop = 'Stop',
}

interface VideoPlayerPayloadMap extends Record<VideoPlayerEventType, unknown> {
  Play: {
    videoId: string
  }
}

export type PayloadMap = NotificationPayloadMap &
  VideoPlayerPayloadMap &
  LibraryManageDialogPayloadMap

export type EventHandler<E extends EventType> = (payload: PayloadMap[E]) => void

export type EventType = keyof PayloadMap

export type EventBusType = {
  on: <E extends EventType>(event: E, handler: EventHandler<E>) => void
  off: <E extends EventType>(event: E, handler: EventHandler<E>) => void
  emit: <E extends EventType>(event: E, payload: PayloadMap[E]) => void
}
