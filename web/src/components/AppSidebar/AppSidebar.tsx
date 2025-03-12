import { useState } from 'react'
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

import { Button } from '~/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '~/components/ui/dialog'
import { Input } from '~/components/ui/input'
import { Label } from '~/components/ui/label'
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
} from '~/components/ui/sidebar'

export default function AppSidebar() {
  const location = useLocation()
  const [open, setOpen] = useState(false)
  const [playlistName, setPlaylistName] = useState('')

  const handleCreatePlaylist = () => {
    // In a real app, you would save this to a database
    console.log('Creating playlist:', playlistName)
    setOpen(false)
    setPlaylistName('')
  }

  return (
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
                <SidebarMenuButton asChild isActive={location.pathname === '/'}>
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
  )
}
