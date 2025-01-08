import { useState } from 'react'

import { Button } from '@material-tailwind/react'

import { Variation } from '../NotificationItem/constants'
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

  const handleAddNotification = () =>
    addNotification(
      {
        title: 'Test title',
        message: 'Test',
      },
      Variation.Success
    )

  if (hasError) {
    throw new Error('Test error')
  }

  return (
    <>
      <Button onClick={() => setHasErrorState(true)}>Throw error</Button>
      <Button onClick={() => handleAddNotification()}>
        Show test notification
      </Button>
      <Button onClick={() => handleAddScanning()}>Add scanning</Button>
      <Button onClick={() => handleDeleteScanning()}>Delete scanning</Button>
    </>
  )
}
