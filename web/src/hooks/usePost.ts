import { useAxios } from '~/hooks/useAxios'

export const usePost = () => {
  const { axiosInstance } = useAxios()
  const post = async <D, R>(url: string, data?: D) => {
    const response = await axiosInstance.post<R>(url, data)
    return response.data
  }

  return post
}
