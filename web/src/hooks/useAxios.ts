import { useContext } from 'react'
import {
  AxiosContext,
  AxiosContextProps,
} from '@/contexts/AxiosContext/AxiosContext'

export const useAxios = (): AxiosContextProps => {
  const context = useContext(AxiosContext)
  if (!context) {
    throw new Error('useAxios must be used within an AxiosProvider')
  }

  return context
}
