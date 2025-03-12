import { Outlet } from 'react-router-dom'

import AppSidebar from './components/AppSidebar/AppSidebar'
import { SidebarProvider } from './components/ui/sidebar'

export default function Layout() {
  return (
    <SidebarProvider>
      <div className='flex min-h-screen w-full overflow-hidden'>
        <AppSidebar />
        <main className='flex-1 overflow-auto'>
          <Outlet />
        </main>
      </div>
    </SidebarProvider>
  )
}
