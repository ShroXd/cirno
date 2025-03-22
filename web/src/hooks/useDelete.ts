import { AxiosResponse } from 'axios'

import { useAxios } from '~/hooks/useAxios'

export const useDelete = () => {
  const { axiosInstance } = useAxios()

  return async <T extends AxiosResponse>(url: string) => {
    const response = await axiosInstance.delete<T>(url)
    return response.data
  }
}
