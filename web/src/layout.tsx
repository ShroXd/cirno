import { Outlet } from 'react-router-dom'

import AppSidebar from './components/AppSidebar/AppSidebar'
import { SidebarProvider } from './components/ui/sidebar'

export default function Layout() {
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
