import { useEffect, useState } from 'react'
import { Link, useLocation } from 'react-router-dom'
import { useTranslation } from 'react-i18next'
import {
  Chip,
  Typography,
  Button,
  Tabs,
  TabsHeader,
  TabsBody,
  Tab,
  TabPanel,
  Select,
  Option,
} from '@material-tailwind/react'
import { HeartIcon, PlayIcon } from '@heroicons/react/16/solid'

import { TVSeriesDTO } from '../../bindings/TVSeriesDTO'
import { SeasonDTO } from '../../bindings/SeasonDTO'
import useSWR from 'swr'

export const Detail = () => {
  const [serie, setSerie] = useState<TVSeriesDTO>()
  const [sortBy, setSortBy] = useState<string>('episode_number')

  const location = useLocation()
  const { t } = useTranslation()

  const { data, error, isLoading } = useSWR<SeasonDTO[]>(
    `/media-library/series/${location.state.detail.id}/seasons`,
    (url: string) => fetch(url).then(res => res.json())
  )

  useEffect(() => {
    setSerie(location.state.detail)
  }, [serie?.title])

  // TODO: encapsulate the loading and error states
  if (isLoading) return <div>Loading...</div>
  if (error) return <div>Error: {error.message}</div>

  return (
    <>
      <div className='h-full w-full overflow-y-auto'>
        <div className='bg-base-300 mb-12 rounded-2xl p-4'>
          <div className='flex w-full flex-row items-start gap-10'>
            <img
              className='h-80 w-52 rounded-xl object-cover shadow-xl shadow-blue-gray-900/50'
              src={serie?.poster_path ?? ''}
              alt={serie?.title}
            />
            <div className='flex flex-col justify-center gap-4'>
              <Typography variant='h2'>{serie?.title}</Typography>
              <div className='flex flex-row gap-2'>
                {serie?.genres.map(genre => (
                  <Chip
                    size='md'
                    variant='outlined'
                    key={genre}
                    value={genre}
                  />
                ))}
              </div>
              <Typography variant='paragraph' className='mb-6'>
                {serie?.plot}
              </Typography>
              <div className='mt-auto flex flex-row flex-wrap gap-4'>
                <Button
                  className='flex flex-row items-center gap-3'
                  variant='gradient'
                >
                  <PlayIcon className='h-5 w-5' />
                  {t('detail.watch')}
                </Button>
                <Button
                  className='flex flex-row items-center gap-3'
                  variant='outlined'
                >
                  <HeartIcon className='h-5 w-5 text-deep-orange-600' />
                  {t('detail.add_to_favorites')}
                </Button>
              </div>
            </div>
          </div>
        </div>
        <Tabs id='seasons' value='0' className='px-4'>
          <div className='flex flex-row items-center justify-start py-2'>
            <TabsHeader className='w-auto'>
              {data?.map((_, index) => (
                <Tab className='w-40' key={index} value={`${index}`}>
                  {t('detail.seasons')} {index + 1}
                </Tab>
              ))}
            </TabsHeader>

            <div className='ml-auto w-72'>
              <Select
                label={t('detail.sort_by')}
                value={sortBy}
                onChange={value => setSortBy(value ?? 'episode_number')}
              >
                <Option value='title'>{t('detail.title')}</Option>
                <Option value='date_added'>{t('detail.date_added')}</Option>
                <Option value='rating'>{t('detail.rating')}</Option>
                <Option value='episode_number'>
                  {t('detail.episode_number')}
                </Option>
              </Select>
            </div>
          </div>
          <TabsBody
            animate={{
              mount: { y: 0 },
              unmount: { y: 100 },
            }}
          >
            {data?.map((season, index) => (
              <TabPanel key={index} value={`${index}`} className='px-0'>
                <div className='grid grid-cols-4 gap-4'>
                  {season.episodes.map(episode => (
                    <div key={episode.title} className='flex flex-col'>
                      <div className='w-min-42 relative flex h-40'>
                        <img
                          src={episode.thumb_image ?? ''}
                          alt={episode.title ?? ''}
                          className='h-full w-full rounded-lg object-cover'
                        />
                        <Link
                          to={`/play`}
                          state={{ file: episode.video_file_path }}
                        >
                          <button className='absolute inset-0 flex items-center justify-center rounded-lg bg-black bg-opacity-50 opacity-0 transition-opacity duration-500 ease-in-out hover:opacity-100'>
                            <PlayIcon className='h-12 w-12 text-white' />
                          </button>
                        </Link>
                      </div>
                      <div className='mb-1 mt-2 truncate text-base font-semibold'>
                        {episode.title ?? 'Untitled Episode'}
                      </div>
                      <div className='mb-1 truncate text-xs font-medium text-gray-500'>
                        {t('detail.episode_number')}{' '}
                        {episode.episodes_number?.toString() ?? 'N/A'}
                      </div>
                    </div>
                  ))}
                </div>
              </TabPanel>
            ))}
          </TabsBody>
        </Tabs>
      </div>
    </>
  )
}
