import { useAxios } from '~/hooks/useAxios'

export const usePut = () => {
  const { axiosInstance } = useAxios()
  const put = async <T>(url: string, data?: T) => {
    const response = await axiosInstance.put<T>(url, data)
    return response.data
  }

  return put
}
