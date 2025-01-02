import { useCallback, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { NavLink, useNavigate, useParams } from 'react-router-dom'

import { Cog6ToothIcon, TrashIcon } from '@heroicons/react/24/outline'
import { Button, Typography } from '@material-tailwind/react'

import { LibraryDto } from '~/bindings/LibraryDto'
import { MediaItemDto } from '~/bindings/MediaItemDto'
import { Container } from '~/components/Container/Container'
import { ContentCard } from '~/components/ContentCard/ContentCard'
import { DeleteConfirmationDialog } from '~/components/DeleteConfirmationDialog/DeleteConfirmationDialog'
import { useDelete } from '~/hooks/useDelete'
import { useFetch } from '~/hooks/useFetch'
import { wrapInGrid } from '~/pages/utils'

export const LibraryDetail = () => {
  const [showDeleteConfirmation, setShowDeleteConfirmation] = useState(false)

  const { libraryId } = useParams()

  const {
    data: detail,
    error: detailError,
    isLoading: detailIsLoading,
  } = useFetch<LibraryDto>(`/library/${libraryId}`)

  const { data, error, isLoading } = useFetch<MediaItemDto[]>(
    `/library/${libraryId}/media`
  )
  const { t } = useTranslation()
  const navigate = useNavigate()
  const del = useDelete()

  const container = useCallback(wrapInGrid, [])

  if (isLoading || detailIsLoading) return <div>Loading...</div>
  if (error || detailError) return <div>Error: {error?.message}</div>

  const handleDeleteConfirmation = async () => {
    await del(`/library/${libraryId}`)
    setShowDeleteConfirmation(false)
    navigate('/')
  }

  const handleDeleteCancel = () => {
    setShowDeleteConfirmation(false)
  }

  return (
    <>
      <DeleteConfirmationDialog
        title={t('component.deleteConfirmationDialog.title')}
        description={t('component.deleteConfirmationDialog.description')}
        open={showDeleteConfirmation}
        handleConfirm={handleDeleteConfirmation}
        handleCancel={handleDeleteCancel}
      />
      <Container>
        <div className='flex flex-row items-center gap-2 mb-4 mt-2'>
          <Typography className='mr-2' variant='h4' color='blue-gray'>
            {detail?.name}
          </Typography>
          <Button variant='text' size='sm' ripple={false}>
            <Cog6ToothIcon className='h-4 w-4' />
          </Button>
          <Button
            className='ml-auto'
            variant='text'
            color='red'
            size='sm'
            ripple={false}
            onClick={() => setShowDeleteConfirmation(true)}
          >
            <TrashIcon className='h-4 w-4' />
          </Button>
        </div>
        {container(
          data?.map(item => (
            <NavLink
              to={`/library/${libraryId}/media/${item.id}`}
              key={item.id.toString()}
            >
              <ContentCard
                imageUrl={item.poster_path ?? ''}
                title={item.title}
              />
            </NavLink>
          ))
        )}
      </Container>
    </>
  )
}
