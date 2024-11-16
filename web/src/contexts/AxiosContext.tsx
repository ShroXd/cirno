import { createContext, FC, ReactNode, useEffect, useState } from 'react'
import axios, { AxiosInstance } from 'axios'
import { useEventBus } from '../hooks/useEventBus'

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

  const { listenForMessages } = useEventBus()
  listenForMessages('create_ws_client', (payload: unknown) => {
    // TODO: type guard and process the payload
    console.log('create_ws_client', payload)
    const payloadObj = JSON.parse(payload as string)

    if (
      payloadObj &&
      typeof payloadObj === 'object' &&
      'key' in payloadObj &&
      typeof payloadObj.key === 'string'
    ) {
      console.log('create_ws_client', payloadObj)

      setWsClientKey(payloadObj.key)
    }
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

  // axiosInstance.interceptors.request.use(request => {
  //   request.headers['X-WS-CLIENT-KEY'] = 'hello world'
  //   return request
  // })

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
