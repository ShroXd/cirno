import { useCallback, useMemo, useState } from 'react'
import { useLocation, useNavigate } from 'react-router-dom'
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
import useSWR from 'swr'

import { EpisodeDto } from '@bindings/EpisodeDto'
import { usePost } from '@/hooks/usePost'
import { useEventBus } from '@/hooks/useEventBus'

const transformEpisodesToSeasons = (episodes: EpisodeDto[] | undefined) => {
  if (!episodes) return []

  const groupedEpisodes = episodes.reduce(
    (acc: { [key: number]: EpisodeDto[] }, episode) => {
      const seasonNumber = Number(episode.season_number)
      if (!acc[seasonNumber]) {
        acc[seasonNumber] = []
      }
      acc[seasonNumber].push(episode)
      return acc
    },
    {}
  )

  return Object.entries(groupedEpisodes).map(([season, episodes]) => ({
    season: parseInt(season),
    episodes: episodes.sort(
      (a, b) => Number(a.episodes_number) - Number(b.episodes_number)
    ),
  }))
}

// TODO: fetch media item details from backend instead of use the passed state
export const MediaDetail = () => {
  const [sortBy, setSortBy] = useState<string>('episode_number')
  const [isWaitingForHlsStream, setIsWaitingForHlsStream] = useState(false)

  const location = useLocation()
  const { t } = useTranslation()
  const post = usePost()
  const navigate = useNavigate()
  const { onEvent } = useEventBus()

  const { data, error, isLoading } = useSWR<EpisodeDto[]>(
    `/library/${location.state.detail.id}/media/${location.state.detail.id}/episodes`,
    {
      fetcher: (url: string) => fetch(url).then(res => res.json()),
    }
  )

  const seasons = useMemo(() => transformEpisodesToSeasons(data), [data])

  const handlePlay = useCallback(
    (episode: EpisodeDto) => {
      setIsWaitingForHlsStream(true)
      post('/video-player/play', {
        path: episode.video_file_path,
      })

      onEvent('HlsStreamInitialized', () => {
        setIsWaitingForHlsStream(false)
        navigate('/video', {
          state: {
            episode: episode,
          },
        })
      })
    },
    [navigate, onEvent, post]
  )

  // TODO: encapsulate the loading and error states
  if (isLoading || isWaitingForHlsStream) return <div>Loading...</div>
  if (error) return <div>Error: {error.message}</div>

  return (
    <>
      <div className='h-full w-full overflow-y-auto'>
        <div className='bg-base-300 mb-12 rounded-2xl p-4'>
          <div className='flex w-full flex-row items-start gap-10'>
            <img
              className='h-80 w-52 rounded-xl object-cover shadow-xl shadow-blue-gray-900/50'
              src={location.state.detail.poster_path ?? ''}
              alt={location.state.detail.title}
            />
            <div className='flex flex-col justify-center gap-4'>
              <Typography variant='h2'>
                {location.state.detail.title}
              </Typography>
              <div className='flex flex-row gap-2'>
                {location.state.detail.genres.map((genre: string) => (
                  <Chip
                    size='md'
                    variant='outlined'
                    key={genre}
                    value={genre}
                  />
                ))}
              </div>
              <Typography variant='paragraph' className='mb-6'>
                {location.state.detail.plot}
              </Typography>
              <div className='mt-auto flex flex-row flex-wrap gap-4'>
                <Button
                  className='flex flex-row items-center gap-3'
                  variant='gradient'
                >
                  <PlayIcon className='h-5 w-5' />
                  {t('page.detail.watch')}
                </Button>
                <Button
                  className='flex flex-row items-center gap-3'
                  variant='outlined'
                >
                  <HeartIcon className='h-5 w-5 text-deep-orange-600' />
                  {t('page.detail.add_to_favorites')}
                </Button>
              </div>
            </div>
          </div>
        </div>
        <Tabs id='seasons' value='0' className='px-4'>
          <div className='flex flex-row items-center justify-start py-2'>
            <TabsHeader className='w-auto'>
              {seasons?.map((_, index) => (
                <Tab className='w-40' key={index} value={`${index}`}>
                  {t('page.detail.seasons')} {index + 1}
                </Tab>
              ))}
            </TabsHeader>

            <div className='ml-auto w-72'>
              <Select
                label={t('page.detail.sort_by')}
                value={sortBy}
                onChange={value => setSortBy(value ?? 'episode_number')}
              >
                <Option value='title'>{t('page.detail.title')}</Option>
                <Option value='date_added'>
                  {t('page.detail.date_added')}
                </Option>
                <Option value='rating'>{t('page.detail.rating')}</Option>
                <Option value='episode_number'>
                  {t('page.detail.episode_number')}
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
            {seasons?.map((season, index) => (
              <TabPanel key={index} value={`${index}`} className='px-0'>
                <div className='grid grid-cols-4 gap-4 gap-y-10'>
                  {season.episodes.map(episode => (
                    <div className='flex flex-col' key={episode.title}>
                      <div className='w-min-42 relative flex h-40'>
                        <img
                          src={episode.thumb_image ?? ''}
                          alt={episode.title ?? ''}
                          className='h-full w-full rounded-lg object-cover'
                        />
                        <button
                          className='absolute inset-0 flex items-center justify-center rounded-lg bg-black bg-opacity-50 opacity-0 transition-opacity duration-500 ease-in-out hover:opacity-100'
                          onClick={() => handlePlay(episode)}
                        >
                          <PlayIcon className='h-12 w-12 text-white' />
                        </button>
                      </div>
                      <div className='mb-1 mt-2 truncate text-base font-semibold'>
                        {episode.title ?? 'Untitled Episode'}
                      </div>
                      <div className='mb-1 truncate text-xs font-medium text-gray-500'>
                        {t('page.detail.episode_number')}{' '}
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
