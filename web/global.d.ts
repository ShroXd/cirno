import React from 'react'

declare module 'react' {
  interface HTMLAttributes<T> {
    placeholder?: string
    onPointerEnterCapture?: React.PointerEventHandler<T>
    onPointerLeaveCapture?: React.PointerEventHandler<T>
  }
}
