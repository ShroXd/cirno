import { useAxios } from './useAxios'

export const usePost = () => {
  const { axiosInstance } = useAxios()

  const post = async <T>(url: string, data: T) => {
    const response = await axiosInstance.post<T>(url, data)
    return response.data
  }

  return post
}
