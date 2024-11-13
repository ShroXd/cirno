import { createContext, FC, ReactNode } from 'react'
import axios, { AxiosInstance } from 'axios'

export interface AxiosContextProps {
  axiosInstance: AxiosInstance
}

export const AxiosContext = createContext<AxiosContextProps | undefined>(
  undefined
)

export const AxiosProvider: FC<{ children: ReactNode }> = ({ children }) => {
  // TODO: support auth token
  // const [authToken, setAuthToken] = useState<string | null>(null)

  // TODO: config url via env variable
  const axiosInstance = axios.create({
    baseURL: 'http://localhost:8000',
  })

  axiosInstance.interceptors.request.use(request => {
    // TODO: add auth token
    return request
  })

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
