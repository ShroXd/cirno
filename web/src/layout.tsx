import { useCallback, useEffect } from 'react'
import { useTranslation } from 'react-i18next'
import { Outlet } from 'react-router-dom'

import { toast } from 'sonner'

import AppSidebar from './components/AppSidebar/AppSidebar'
import { SidebarProvider } from './components/ui/sidebar'
import { useEventBus } from './hooks/useEventBus'

export default function Layout() {
  const { onEvent, offEvent } = useEventBus()
  const { t } = useTranslation()

  const handleLibrarySaved = useCallback(
    (payload: { libraryName: string }) => {
      toast.success(
        t('common.event_message.library_saved', {
          libraryName: payload.libraryName,
        })
      )
    },
    [t]
  )

  useEffect(() => {
    onEvent('LibrarySaved', handleLibrarySaved)
    return () => offEvent('LibrarySaved', handleLibrarySaved)
  }, [onEvent, offEvent, handleLibrarySaved])

  return (
    <SidebarProvider>
      <div className='flex w-full'>
        <AppSidebar />
        <main className='flex-1'>
          <Outlet />
        </main>
      </div>
    </SidebarProvider>
  )
}
