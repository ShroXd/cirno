import { useCallback, useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useNavigate, useParams } from 'react-router-dom'

import { HeartIcon, PlayIcon } from '@heroicons/react/16/solid'
import {
  Button,
  Chip,
  Option,
  Select,
  Tab,
  TabPanel,
  Tabs,
  TabsBody,
  TabsHeader,
  Typography,
} from '@material-tailwind/react'

import { EpisodeDto } from '~/bindings/EpisodeDto'
import { MediaItemDto } from '~/bindings/MediaItemDto'
import { Divider } from '~/components/Divider/Divider'
import { useEventBus } from '~/hooks/useEventBus'
import { useFetch } from '~/hooks/useFetch'
import { usePost } from '~/hooks/usePost'

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
      (a, b) => Number(a.episode_number) - Number(b.episode_number)
    ),
  }))
}

export const MediaDetail = () => {
  const [sortBy, setSortBy] = useState<string>('episode_number')
  const [isWaitingForHlsStream, setIsWaitingForHlsStream] = useState(false)

  const { t } = useTranslation()
  const post = usePost()
  const navigate = useNavigate()
  const { onEvent } = useEventBus()

  const { libraryId, mediaId } = useParams()

  const {
    data: media,
    error: mediaError,
    isLoading: mediaIsLoading,
  } = useFetch<MediaItemDto>(`/library/${libraryId}/media/${mediaId}`)

  const {
    data: episodes,
    error: episodesError,
    isLoading: episodesIsLoading,
  } = useFetch<EpisodeDto[]>(`/library/${libraryId}/media/${mediaId}/episodes`)

  const seasons = useMemo(
    () => transformEpisodesToSeasons(episodes),
    [episodes]
  )

  const handlePlay = useCallback(
    (episode: EpisodeDto) => {
      setIsWaitingForHlsStream(true)
      post('/video-player/play', {
        path: episode.video_file_path,
      })

      onEvent('HlsStreamInitialized', () => {
        setIsWaitingForHlsStream(false)
        navigate(`/library/${libraryId}/media/${mediaId}/video/${episode.id}`, {
          state: {
            episode: episode,
          },
        })
      })
    },
    [navigate, onEvent, post, libraryId, mediaId]
  )

  if (mediaIsLoading || episodesIsLoading || isWaitingForHlsStream)
    return <div>Loading...</div>
  if (mediaError || episodesError)
    return <div>Error: {mediaError?.message}</div>

  return (
    <>
      <div className='h-full w-full pb-12'>
        <div className='bg-base-300 rounded-2xl py-4 px-6'>
          <div className='flex w-full flex-row items-start gap-10'>
            <img
              className='h-80 w-52 rounded-xl object-cover shadow-xl shadow-blue-gray-900/50'
              src={media?.poster_path ?? ''}
              alt={media?.title}
            />
            <div className='flex flex-col justify-center gap-4'>
              <Typography variant='h2'>{media?.title}</Typography>
              <div className='flex flex-row gap-2'>
                {media?.genres.map((genre: string) => (
                  <Chip
                    size='md'
                    variant='outlined'
                    key={genre}
                    value={genre}
                  />
                ))}
              </div>
              <Typography variant='paragraph' className='mb-6'>
                {media?.plot}
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
        <Divider className='my-6 mx-6' />
        <Tabs id='seasons' value='0' className='px-6'>
          <div className='flex flex-row items-center justify-start py-2'>
            {seasons.length > 1 && (
              <TabsHeader className='w-auto'>
                {seasons.map((_, index) => (
                  <Tab className='w-40' key={index} value={`${index}`}>
                    {t('page.detail.seasons')} {index + 1}
                  </Tab>
                ))}
              </TabsHeader>
            )}

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
                        {episode.episode_number?.toString() ?? 'N/A'}
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
