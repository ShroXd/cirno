import { FC } from 'react'

import { FolderIcon } from 'lucide-react'

import { Button } from '~/components/ui/button'

interface DirectoryPickerProps {
  setDirectoryHandle: (handle: FileSystemDirectoryHandle | null) => void
}

export const DirectoryPicker: FC<DirectoryPickerProps> = ({
  setDirectoryHandle,
}) => {
  const selectDirectory = async () => {
    try {
      const handle = await window.showDirectoryPicker()
      setDirectoryHandle(handle)
    } catch (error) {
      console.error('Error selecting directory:', error)
    }
  }

  return (
    <Button
      size='sm'
      className='rounded'
      variant='outline'
      onClick={selectDirectory}
    >
      <FolderIcon className='h-5 w-5' />
    </Button>
  )
}
