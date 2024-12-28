import { FC, ReactNode, createContext, useEffect, useState } from 'react'

import axios, { AxiosInstance } from 'axios'

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
  const [wsClientKey, setWsClientKey] = useState<string | null>(null)

  const { onEvent } = useEventBus()
  onEvent('RegisterClient', payload => {
    setWsClientKey(payload.clientKey)
  })

  // TODO: config url via env variable
  const axiosInstance = axios.create({
    baseURL: 'http://localhost:8000',
  })

  useEffect(() => {
    if (wsClientKey) {
      axiosInstance.interceptors.request.use(request => {
        request.headers['X-WS-CLIENT-KEY'] = wsClientKey
        return request
      })
    }
  }, [wsClientKey, axiosInstance.interceptors.request])

  axiosInstance.interceptors.response.use(
    response => response,
    error => {
      // TODO: integrate with backend error handling
      console.error(error)
      return Promise.reject(error)
    }
  )

  return (
    <AxiosContext.Provider value={{ axiosInstance }}>
      {children}
    </AxiosContext.Provider>
  )
}
