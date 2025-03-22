import { FC, ReactNode, createContext, useEffect } from 'react'
import { useTranslation } from 'react-i18next'

import axios, { AxiosInstance } from 'axios'
import { toast } from 'sonner'

import { useEventBus } from '~/hooks/useEventBus'

export interface AxiosContextProps {
  axiosInstance: AxiosInstance
}

export const AxiosContext = createContext<AxiosContextProps | undefined>(
  undefined
)

export const AxiosProvider: FC<{ children: ReactNode }> = ({ children }) => {
  // TODO: support auth token
  // const [authToken, setAuthToken] = useState<string | null>(null)

  const { onEvent } = useEventBus()
  const { t } = useTranslation()

  // TODO: config url via env variable
  const axiosInstance = axios.create({})

  useEffect(
    () =>
      onEvent('RegisterClient', payload =>
        axiosInstance.interceptors.request.use(request => {
          request.headers['X-WS-CLIENT-KEY'] = payload.clientKey
          return request
        })
      ),
    [onEvent, axiosInstance.interceptors.request]
  )

  axiosInstance.interceptors.response.use(
    response => response,
    error => {
      console.error(error)
      switch (error.response?.status) {
        case 500:
          toast.error(t('Internal server error'))
          break
        default:
          toast.error(error.response?.data.message || t('common.error.title'))
          break
      }
      return Promise.reject(error)
    }
  )

  return (
    <AxiosContext.Provider value={{ axiosInstance }}>
      {children}
    </AxiosContext.Provider>
  )
}
