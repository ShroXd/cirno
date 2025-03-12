import type React from 'react'
import { useCallback, useEffect, useRef, useState } from 'react'
import { Link, useLocation } from 'react-router-dom'

import {
  Compass,
  Film,
  Home,
  Library,
  ListPlus,
  PlaySquare,
  Plus,
  Search,
  Settings,
  Star,
} from 'lucide-react'

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

export default function AppSidebar() {
  const location = useLocation()
  const [open, setOpen] = useState(false)
  const [playlistName, setPlaylistName] = useState('')
  const [sidebarWidth, setSidebarWidth] = useState(240) // Default width
  const sidebarRef = useRef<HTMLDivElement>(null)
  const resizingRef = useRef(false)
  const { state } = useSidebar()

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

  return (
    <div ref={sidebarRef} style={sidebarStyle} className='relative h-full'>
      <Sidebar>
        <SidebarHeader className='p-4'>
          <Link to='/' className='flex items-center gap-2'>
            <Film className='h-6 w-6 text-primary' />
            <span className='text-xl font-bold'>StreamHub</span>
          </Link>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Menu</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/'}
                  >
                    <Link to='/'>
                      <Home className='mr-2 h-4 w-4' />
                      <span>Home</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/discover'}
                  >
                    <Link to='/discover'>
                      <Compass className='mr-2 h-4 w-4' />
                      <span>Discover</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/search'}
                  >
                    <Link to='/search'>
                      <Search className='mr-2 h-4 w-4' />
                      <span>Search</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>

          <SidebarGroup>
            <SidebarGroupLabel>Library</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/library'}
                  >
                    <Link to='/library'>
                      <Library className='mr-2 h-4 w-4' />
                      <span>All Content</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/movies'}
                  >
                    <Link to='/movies'>
                      <Film className='mr-2 h-4 w-4' />
                      <span>Movies</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/tv-shows'}
                  >
                    <Link to='/tv-shows'>
                      <PlaySquare className='mr-2 h-4 w-4' />
                      <span>TV Shows</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/favorites'}
                  >
                    <Link to='/favorites'>
                      <Star className='mr-2 h-4 w-4' />
                      <span>Favorites</span>
                    </Link>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>

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

          <SidebarGroup>
            <SidebarGroupLabel>Settings</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SidebarMenuItem>
                  <SidebarMenuButton
                    asChild
                    isActive={location.pathname === '/settings'}
                  >
                    <Link to='/settings'>
                      <Settings className='mr-2 h-4 w-4' />
                      <span>Settings</span>
                    </Link>
                  </SidebarMenuButton>
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
