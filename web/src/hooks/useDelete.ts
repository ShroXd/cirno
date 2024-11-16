import { useAxios } from './useAxios'

export const useDelete = () => {
  const { axiosInstance } = useAxios()

  return async <T>(url: string) => {
    const response = await axiosInstance.delete<T>(url)
    return response.data
  }
}
