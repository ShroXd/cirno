import type React from 'react'
import { useCallback, useEffect, useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Link, useLocation } from 'react-router-dom'

import { Film, ListPlus, Plus, Settings } from 'lucide-react'

import { Button } from '../ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '../ui/dialog'
import { Input } from '../ui/input'
import { Label } from '../ui/label'
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
  useSidebar,
} from '../ui/sidebar'
import { libraryItems, menuItems } from './constants'
import { useFeatures } from '~/hooks/feature/useFeatures'

export default function AppSidebar() {
  const location = useLocation()
  const [open, setOpen] = useState(false)
  const [playlistName, setPlaylistName] = useState('')
  const [sidebarWidth, setSidebarWidth] = useState(240) // Default width
  const sidebarRef = useRef<HTMLDivElement>(null)
  const resizingRef = useRef(false)
  const { state } = useSidebar()

  const { isFeatureEnabled } = useFeatures()
  const { t } = useTranslation()

  const handleCreatePlaylist = () => {
    // In a real app, you would save this to a database
    console.log('Creating playlist:', playlistName)
    setOpen(false)
    setPlaylistName('')
  }

  const startResize = useCallback((e: React.MouseEvent) => {
    e.preventDefault()
    resizingRef.current = true
    document.addEventListener('mousemove', handleResize)
    document.addEventListener('mouseup', stopResize)
  }, [])

  const handleResize = useCallback((e: MouseEvent) => {
    if (resizingRef.current) {
      const newWidth = e.clientX
      // Set min and max constraints
      if (newWidth >= 180 && newWidth <= 400) {
        setSidebarWidth(newWidth)
        if (sidebarRef.current) {
          sidebarRef.current.style.width = `${newWidth}px`
          sidebarRef.current.style.minWidth = `${newWidth}px`
        }
      }
    }
  }, [])

  const stopResize = useCallback(() => {
    resizingRef.current = false
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)
  }, [handleResize])

  // Clean up event listeners
  useEffect(() => {
    return () => {
      document.removeEventListener('mousemove', handleResize)
      document.removeEventListener('mouseup', stopResize)
    }
  }, [handleResize, stopResize])

  // Don't apply custom width when sidebar is collapsed
  const sidebarStyle =
    state === 'collapsed'
      ? {}
      : { width: `${sidebarWidth}px`, minWidth: `${sidebarWidth}px` }

  const renderSidebarMenuButton = (
    label: string,
    icon: React.ReactNode,
    path: string
  ) => (
    <SidebarMenuButton
      className='transition-all duration-200 hover:translate-x-1'
      asChild
      isActive={location.pathname === path}
    >
      <Link to={path}>
        {icon}
        <span>{t(label)}</span>
      </Link>
    </SidebarMenuButton>
  )

  const renderLibraryGroup = () => {
    return (
      <SidebarGroup>
        <SidebarGroupLabel>{t('sidebar.library')}</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            {libraryItems.map(item => (
              <SidebarMenuItem key={item.label}>
                {renderSidebarMenuButton(item.label, item.icon, item.path)}
              </SidebarMenuItem>
            ))}
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    )
  }

  const renderPlaylistsGroup = () => {
    return (
      <SidebarGroup>
        <div className='flex items-center justify-between px-2'>
          <SidebarGroupLabel>Playlists</SidebarGroupLabel>
          <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
              <Button variant='ghost' size='icon' className='h-6 w-6'>
                <Plus className='h-4 w-4' />
                <span className='sr-only'>Create Playlist</span>
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>Create New Playlist</DialogTitle>
                <DialogDescription>
                  Give your playlist a name to get started.
                </DialogDescription>
              </DialogHeader>
              <div className='grid gap-4 py-4'>
                <div className='grid gap-2'>
                  <Label htmlFor='playlist-name'>Playlist Name</Label>
                  <Input
                    id='playlist-name'
                    value={playlistName}
                    onChange={e => setPlaylistName(e.target.value)}
                    placeholder='My Awesome Playlist'
                  />
                </div>
              </div>
              <DialogFooter>
                <Button
                  onClick={handleCreatePlaylist}
                  disabled={!playlistName.trim()}
                >
                  Create Playlist
                </Button>
              </DialogFooter>
            </DialogContent>
          </Dialog>
        </div>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton
                asChild
                isActive={location.pathname === '/playlists'}
              >
                <Link to='/playlists'>
                  <ListPlus className='mr-2 h-4 w-4' />
                  <span>All Playlists</span>
                </Link>
              </SidebarMenuButton>
            </SidebarMenuItem>
            {/* Example playlists - in a real app these would be dynamically generated */}
            <SidebarMenuItem>
              <SidebarMenuButton asChild>
                <Link to='/playlists/weekend'>
                  <span>Weekend Watchlist</span>
                </Link>
              </SidebarMenuButton>
            </SidebarMenuItem>
            <SidebarMenuItem>
              <SidebarMenuButton asChild>
                <Link to='/playlists/sci-fi'>
                  <span>Sci-Fi Collection</span>
                </Link>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    )
  }

  return (
    <div ref={sidebarRef} style={sidebarStyle} className='relative h-full'>
      <Sidebar>
        <SidebarHeader className='p-4'>
          <Link to='/' className='flex items-center gap-2'>
            <img src='/cirno_logo.png' alt='Logo' className='mr-2 h-10 w-10' />
            <span className='text-xl font-bold'>Cirno</span>
          </Link>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>{t('sidebar.menu')}</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {menuItems.map(item => (
                  <SidebarMenuItem key={item.label}>
                    {renderSidebarMenuButton(item.label, item.icon, item.path)}
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>

          {isFeatureEnabled('library') && renderLibraryGroup()}
          {isFeatureEnabled('playlists') && renderPlaylistsGroup()}

          <SidebarGroup>
            <SidebarGroupLabel>{t('sidebar.settings')}</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  {renderSidebarMenuButton(
                    t('sidebar.settings'),
                    <Settings className='mr-2 h-4 w-4' />,
                    '/settings'
                  )}
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
        <SidebarRail />
      </Sidebar>

      {/* Resize handle - only show when sidebar is expanded */}
      {state === 'expanded' && (
        <div
          className='absolute right-0 top-0 h-full w-1 cursor-ew-resize bg-border transition-colors hover:bg-primary/50'
          onMouseDown={startResize}
        />
      )}
    </div>
  )
}
