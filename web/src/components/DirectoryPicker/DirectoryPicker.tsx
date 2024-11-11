import { FolderIcon } from '@heroicons/react/16/solid'
import { IconButton } from '@material-tailwind/react'
import { FC } from 'react'

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
    <IconButton
      size='sm'
      className='rounded'
      variant='text'
      onClick={selectDirectory}
    >
      <FolderIcon className='h-5 w-5' />
    </IconButton>
  )
}
