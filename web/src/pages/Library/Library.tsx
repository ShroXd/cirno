import { ReactNode, useCallback } from 'react'
import { Typography } from '@material-tailwind/react'
import { NavLink } from 'react-router-dom'

import { TVSeriesDTO } from '../../bindings/TVSeriesDTO'
import { useFetch } from '../../hooks/useFetch'

export const Library = () => {
  const { data, error, isLoading } = useFetch<TVSeriesDTO[]>(
    '/media-libraries/media-items'
  )

  const container = useCallback(
    (children: ReactNode) => (
      // TODO: optimize the grid layout for different screen sizes
      <div className='grid grid-cols-1 gap-4 gap-y-12 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6'>
        {children}
      </div>
    ),
    []
  )

  if (isLoading) {
    const skeletons = Array.from({ length: 10 }, (_, i) => (
      <div className='skeleton h-72 w-48 animate-pulse rounded-lg' key={i} />
    ))
    return container(skeletons)
  }
  if (error) return container(<div>Error: {error.message}</div>)

  return (
    <>
      <Typography className='mb-4 mt-2' variant='h4' color='blue-gray'>
        Recent
      </Typography>
      {container(
        data?.map((serie: TVSeriesDTO) => (
          <div
            className='group flex cursor-pointer select-none flex-col overflow-hidden pb-2'
            key={serie.id.toString()}
          >
            <NavLink to={`/detail/${serie.id}`} state={{ detail: serie }}>
              <figure className='relative h-72'>
                <img
                  className='h-full w-full rounded-xl object-cover object-center'
                  src={serie.poster_path ?? ''}
                  alt={serie.title ?? ''}
                />
                <figcaption className='absolute bottom-2 left-2/4 flex w-[calc(100%-1rem)] -translate-x-2/4 justify-between rounded-xl border border-white bg-white/75 px-2 py-1 shadow-lg shadow-black/5 saturate-200 backdrop-blur-sm transition-transform duration-500 ease-in-out group-hover:-translate-y-2'>
                  <div>
                    <Typography variant='paragraph'>{serie.title}</Typography>
                    <Typography variant='small' color='gray'>
                      {serie.year}
                    </Typography>
                  </div>
                </figcaption>
              </figure>
            </NavLink>
          </div>
        ))
      )}
    </>
  )
}
