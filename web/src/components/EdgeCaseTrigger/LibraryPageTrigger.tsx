import { useState } from 'react'

import { directoryOpen } from 'browser-fs-access'

import { Variation } from '../NotificationItem/constants'
import { Button } from '~/components/ui/button'
import { useNotification } from '~/hooks/useNotification'

interface TriggerOnLibraryProps {
  scanningLibraryIds: Set<number>
  handleScanning: (payload: { libraryId: number }) => void
  handleSaved: (payload: { libraryId: number }) => void
}

export const LibraryPageTrigger = ({
  scanningLibraryIds,
  handleScanning,
  handleSaved,
}: TriggerOnLibraryProps) => {
  const [hasError, setHasErrorState] = useState(false)

  const { addNotification } = useNotification()

  const handleAddScanning = () => {
    const testLibraryId = Math.floor(Math.random() * 1000)
    handleScanning({ libraryId: testLibraryId })
  }

  const handleDeleteScanning = () => {
    const firstId = Array.from(scanningLibraryIds)[0]
    if (!firstId) return
    handleSaved({ libraryId: firstId })
  }

  const handleAddSuccessNotification = () =>
    addNotification(
      {
        title: 'Success title',
        message: 'Success message',
      },
      Variation.Success
    )

  const handleAddErrorNotification = () =>
    addNotification(
      {
        title: 'Error title',
        message: 'Error message',
      },
      Variation.Error
    )

  const handleSelectPath = async () => {
    try {
      const dirHandle = await directoryOpen({
        mode: 'read',
        recursive: false,
      })

      console.log(dirHandle)
    } catch (error) {
      console.error(error)
    }
  }

  const handleGetEnv = () => {
    alert(`Backend service url: ${import.meta.env.VITE_API_URL}`)
  }

  if (hasError) {
    throw new Error('Test error')
  }

  return (
    <div className='flex flex-wrap gap-4'>
      <Button onClick={() => setHasErrorState(true)}>Throw error</Button>
      <Button onClick={() => handleAddSuccessNotification()}>
        Show success notification
      </Button>
      <Button onClick={() => handleAddErrorNotification()}>
        Show error notification
      </Button>
      <Button onClick={() => handleAddScanning()}>Add scanning</Button>
      <Button onClick={() => handleDeleteScanning()}>Delete scanning</Button>
      <Button onClick={() => handleSelectPath()}>Select path</Button>
      <Button onClick={() => handleGetEnv()}>Get env</Button>
    </div>
  )
}
