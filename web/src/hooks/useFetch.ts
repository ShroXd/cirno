import { AxiosInstance } from 'axios'
import useSWR, { SWRConfiguration } from 'swr'

import { useAxios } from '~/hooks/useAxios'

const fetcher =
  (axios: AxiosInstance) =>
  async <T>(url: string) => {
    const response = await axios.get<T>(url)
    return response.data
  }

export const useFetch = <T>(url: string | null, config?: SWRConfiguration) => {
  const { axiosInstance } = useAxios()

  return useSWR<T>(url, fetcher(axiosInstance), config)
}
