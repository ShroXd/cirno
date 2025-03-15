import { Link, useParams } from 'react-router-dom'

import {
  ArrowLeft,
  Calendar,
  Clock,
  Play,
  Plus,
  Share,
  Star,
  ThumbsUp,
} from 'lucide-react'
import { motion } from 'motion/react'

import { MediaItemDto } from '~/bindings/MediaItemDto'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import { SidebarTrigger } from '~/components/ui/sidebar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '~/components/ui/tabs'
import { useFetch } from '~/hooks/useFetch'

export default function ContentDetailPage() {
  const { id } = useParams<{ id: string }>()

  const {
    data: media,
    error: mediaError,
    isLoading: mediaIsLoading,
  } = useFetch<MediaItemDto>(`/media/${id}`)

  if (!media || mediaError) {
    return (
      <div className='container mx-auto px-4 py-12 text-center'>
        <h1 className='mb-4 text-3xl font-bold'>Content Not Found</h1>
        <p>The content you're looking for doesn't exist or has been removed.</p>
      </div>
    )
  }

  // -----

  // const [isWaitingForHlsStream, setIsWaitingForHlsStream] = useState(false)

  // const { t } = useTranslation()
  // const post = usePost()
  // const navigate = useNavigate()
  // const { onEvent } = useEventBus()

  return (
    <div className='flex h-full flex-col bg-background md:px-6'>
      <header className='sticky top-0 z-10 border-b border-border/40 bg-background/80 backdrop-blur-md'>
        <div className='container flex h-16 items-center px-4'>
          <SidebarTrigger className='mr-4 md:hidden' />
          <Button variant='ghost' size='icon' asChild>
            <Link to='/'>
              <ArrowLeft className='h-5 w-5' />
              <span className='sr-only'>Back</span>
            </Link>
          </Button>
          <h1 className='ml-4 text-lg font-medium'>Details</h1>
        </div>
      </header>

      <main className='flex-1 overflow-y-auto px-4 py-6'>
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className='relative mb-8 aspect-[21/9] w-full overflow-hidden rounded-t-xl'
        >
          <img
            src={media?.fanart_path || '/placeholder.svg'}
            alt={media?.title}
            className='absolute inset-0 h-full w-full rounded-t-xl object-cover'
          />
          <div className='absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent' />
          <div className='absolute inset-0 flex items-center justify-center opacity-70'>
            <Button size='lg' className='h-16 w-16 rounded-full p-0'>
              <Play className='h-12 w-12' />
            </Button>
          </div>
        </motion.div>

        <motion.div
          variants={{
            hidden: { opacity: 0 },
            visible: {
              opacity: 1,
              transition: {
                staggerChildren: 0.1,
              },
            },
          }}
          initial='hidden'
          animate={mediaIsLoading ? 'hidden' : 'visible'}
          className='mb-12 grid grid-cols-1 gap-8 lg:grid-cols-3'
        >
          <div className='lg:col-span-2'>
            <motion.div
              variants={{
                hidden: { y: 20, opacity: 0 },
                visible: {
                  y: 0,
                  opacity: 1,
                  transition: {
                    type: 'spring',
                    stiffness: 100,
                    damping: 15,
                  },
                },
              }}
              className='overflow-y-auto'
            >
              <h1 className='mb-2 text-3xl font-bold md:text-4xl'>
                {media?.title}
              </h1>
              <div className='mb-4 flex flex-wrap items-center gap-3 text-sm text-muted-foreground'>
                <span className='flex items-center gap-1'>
                  <Calendar className='h-4 w-4' /> {media?.year}
                </span>
                <span className='flex items-center gap-1'>
                  <Clock className='h-4 w-4' /> {'2h'}
                </span>
                <span className='flex items-center gap-1'>
                  <Star className='h-4 w-4 text-yellow-500' /> {'8.5'}
                </span>
                <Badge variant='outline'>Movie</Badge>
              </div>
              <div className='mb-4 flex flex-wrap gap-2'>
                {media?.genres?.map(genre => (
                  <Badge key={genre} variant='secondary'>
                    {genre}
                  </Badge>
                ))}
              </div>
              <p className='mb-6 text-muted-foreground'>{media?.plot}</p>
              <div className='flex flex-wrap gap-3'>
                <Button asChild size='lg' className='gap-2'>
                  <Link to={`/watch/${media?.id}`}>
                    <Play className='h-4 w-4' /> Watch Now
                  </Link>
                </Button>
                <Button variant='outline' size='lg' className='gap-2'>
                  <Plus className='h-4 w-4' /> Add to Watchlist
                </Button>
                <Button variant='ghost' size='icon' className='ml-auto'>
                  <Share className='h-5 w-5' />
                  <span className='sr-only'>Share</span>
                </Button>
                <Button variant='ghost' size='icon'>
                  <ThumbsUp className='h-5 w-5' />
                  <span className='sr-only'>Like</span>
                </Button>
              </div>
            </motion.div>
          </div>

          <div>
            <motion.div
              variants={{
                hidden: { y: 20, opacity: 0 },
                visible: {
                  y: 0,
                  opacity: 1,
                  transition: {
                    type: 'spring',
                    stiffness: 100,
                    damping: 15,
                  },
                },
              }}
              className='space-y-4'
            >
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Director</h3>
                <p className='text-muted-foreground'>John Doe</p>
              </div>
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Studio</h3>
                <p className='text-muted-foreground'>Studio Doe</p>
              </div>
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Release Date</h3>
                <p className='text-muted-foreground'>2023-01-01</p>
              </div>
              <div>
                <h3 className='mb-1 text-lg font-semibold'>Cast</h3>
                <div className='grid grid-cols-2 gap-2'>
                  {/* {media?.cast.map(person => (
                    <div key={person.name} className='flex items-center gap-2'>
                      <div className='relative h-8 w-8 overflow-hidden rounded-full'>
                        <Image
                          src={person.image || '/placeholder.svg'}
                          alt={person.name}
                          fill
                          className='object-cover'
                        />
                      </div>
                      <div className='overflow-hidden'>
                        <p className='truncate text-sm font-medium'>
                          {person.name}
                        </p>
                        <p className='truncate text-xs text-muted-foreground'>
                          {person.role}
                        </p>
                      </div>
                    </div>
                  ))} */}
                </div>
              </div>
            </motion.div>
          </div>
        </motion.div>
      </main>
    </div>

    // <div className='container mx-auto h-screen overflow-y-auto px-4 py-6 md:px-6'>
    //   {/* Hero Section */}
    //   <div className='relative mb-8 h-[50vh] w-full overflow-hidden rounded-xl'>
    //     <img
    //       src={media?.fanart_path || '/placeholder.svg'}
    //       alt={media?.title}
    //       className='absolute inset-0 h-full w-full object-cover'
    //     />
    //     <div className='absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent' />

    //     <div className='absolute bottom-0 left-0 right-0 p-6 md:p-8'>
    //       <div className='flex max-w-3xl flex-col gap-4'>
    //         <div className='flex flex-wrap gap-2'>
    //           {media?.genres?.map(genre => (
    //             <Badge key={genre} variant='secondary'>
    //               {genre}
    //             </Badge>
    //           ))}
    //         </div>

    //         <h1 className='text-3xl font-bold md:text-5xl'>{media?.title}</h1>

    //         <div className='flex flex-wrap items-center gap-4 text-sm text-muted-foreground'>
    //           {/* <div className='flex items-center'>
    //             <Star className='mr-1 h-4 w-4 fill-yellow-400 text-yellow-400' />
    //             <span>{media?.rating}/10</span>
    //           </div> */}

    //           <div className='flex items-center'>
    //             <Calendar className='mr-1 h-4 w-4' />
    //             <span>{media?.year}</span>
    //           </div>
    //         </div>

    //         <div className='mt-2 flex gap-3'>
    //           <Button className='gap-2' asChild>
    //             <Link to={`/watch/${media?.id}`}>
    //               <Play className='h-4 w-4' />
    //             </Link>
    //           </Button>
    //           <Button variant='outline' className='gap-2'>
    //             <Plus className='h-4 w-4' />
    //             Add to Playlist
    //           </Button>
    //         </div>
    //       </div>
    //     </div>
    //   </div>

    //   {/* Content Details */}
    //   <div className='mb-8 grid grid-cols-1 gap-8 lg:grid-cols-3'>
    //     <div className='lg:col-span-2'>
    //       <h2 className='mb-4 text-2xl font-semibold'>Overview</h2>
    //       <p className='mb-6 text-muted-foreground'>{media?.plot}</p>

    //       {!isMovie && (
    //         <div className='mt-8'>
    //           <h2 className='mb-4 text-2xl font-semibold'>
    //             Seasons & Episodes
    //           </h2>
    //           <Tabs defaultValue={`season-${content.seasons?.[0]?.number}`}>
    //             <TabsList className='mb-4 flex flex-wrap'>
    //               {content.seasons?.map(season => (
    //                 <TabsTrigger
    //                   key={season.number}
    //                   value={`season-${season.number}`}
    //                 >
    //                   Season {season.number}
    //                 </TabsTrigger>
    //               ))}
    //             </TabsList>

    //             {content.seasons?.map(season => (
    //               <TabsContent
    //                 key={season.number}
    //                 value={`season-${season.number}`}
    //               >
    //                 <div className='space-y-4'>
    //                   {season.episodes.map(episode => (
    //                     <div
    //                       key={episode.number}
    //                       className='flex items-center justify-between rounded-lg border p-4 transition-colors hover:bg-muted/50'
    //                     >
    //                       <div className='flex items-center gap-4'>
    //                         <div className='flex h-10 w-10 items-center justify-center rounded-full bg-primary/10 font-medium text-primary'>
    //                           {episode.number}
    //                         </div>
    //                         <div>
    //                           <h4 className='font-medium'>{episode.title}</h4>
    //                           <p className='text-sm text-muted-foreground'>
    //                             {episode.duration}
    //                           </p>
    //                         </div>
    //                       </div>
    //                       <Button size='sm' variant='ghost' asChild>
    //                         <Link to={`/watch/${episode.id}`}>
    //                           <Play className='h-4 w-4' />
    //                         </Link>
    //                       </Button>
    //                     </div>
    //                   ))}
    //                 </div>
    //               </TabsContent>
    //             ))}
    //           </Tabs>
    //         </div>
    //       )}
    //     </div>

    //     <div>
    //       <div className='rounded-lg border p-4'>
    //         <h3 className='mb-4 text-lg font-medium'>Details</h3>
    //         <dl className='space-y-4'>
    //           {isMovie ? (
    //             <>
    //               <div>
    //                 <dt className='text-sm text-muted-foreground'>Director</dt>
    //                 <dd className='mt-1'>{content.director}</dd>
    //               </div>
    //             </>
    //           ) : (
    //             <>
    //               <div>
    //                 <dt className='text-sm text-muted-foreground'>Creator</dt>
    //                 <dd className='mt-1'>{content.creator}</dd>
    //               </div>
    //               <div>
    //                 <dt className='text-sm text-muted-foreground'>Seasons</dt>
    //                 <dd className='mt-1'>{content.seasons?.length}</dd>
    //               </div>
    //             </>
    //           )}

    //           <div>
    //             <dt className='text-sm text-muted-foreground'>Cast</dt>
    //             <dd className='mt-1'>
    //               <ul className='space-y-1'>
    //                 {content.cast.map(actor => (
    //                   <li key={actor}>{actor}</li>
    //                 ))}
    //               </ul>
    //             </dd>
    //           </div>

    //           <div>
    //             <dt className='text-sm text-muted-foreground'>Genres</dt>
    //             <dd className='mt-1 flex flex-wrap gap-1'>
    //               {media?.genres?.map(genre => (
    //                 <Badge key={genre} variant='outline'>
    //                   {genre}
    //                 </Badge>
    //               ))}
    //             </dd>
    //           </div>
    //         </dl>
    //       </div>
    //     </div>
    //   </div>

    //   {/* Related Content Section */}
    //   <div className='mt-12'>
    //     <h2 className='mb-6 text-2xl font-semibold'>You May Also Like</h2>
    //     <div className='grid grid-cols-1 gap-6 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4'>
    //       {/* This would be dynamically generated based on related content */}
    //       {[1, 2, 3, 4].map(i => (
    //         <div key={i} className='overflow-hidden rounded-lg border'>
    //           <div className='relative aspect-video'>
    //             <img
    //               src='/placeholder.svg?height=300&width=500'
    //               alt='Related content'
    //               className='absolute inset-0 h-full w-full object-cover'
    //             />
    //           </div>
    //           <div className='p-3'>
    //             <h3 className='truncate font-medium'>Related Title {i}</h3>
    //             <p className='mt-1 text-xs text-muted-foreground'>
    //               {isMovie ? 'Movie' : 'TV Series'} • {2020 + i}
    //             </p>
    //           </div>
    //         </div>
    //       ))}
    //     </div>
    //   </div>
    // </div>
  )
}
