import { FC, useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Checkbox } from '@radix-ui/react-checkbox'
import {
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@radix-ui/react-tooltip'
import { format, parseISO } from 'date-fns'
import {
  ArrowUpDown,
  Edit,
  FileQuestion,
  FileVideo,
  FolderOpen,
  FolderPlus,
  HardDrive,
  MonitorPlay,
  RefreshCw,
  Search,
  Trash,
} from 'lucide-react'
import { toast } from 'sonner'

import { LibraryDto } from '~/bindings/LibraryDto'
import { Badge } from '~/components/ui/badge'
import { Button } from '~/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { Input } from '~/components/ui/input'
import { Progress } from '~/components/ui/progress'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { Skeleton } from '~/components/ui/skeleton'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '~/components/ui/table'
import { Tooltip } from '~/components/ui/tooltip'
import { useFetch } from '~/hooks/useFetch'

interface LibrariesProps {
  handleAddLibrary: () => void
}

export const Libraries: FC<LibrariesProps> = ({ handleAddLibrary }) => {
  const [filteredLibraries, setFilteredLibraries] = useState<LibraryDto[]>([])
  const [activeScan, setActiveScan] = useState<string | null>(null)
  const [_scanProgress, setScanProgress] = useState(0)
  const [_showAddDialog, _setShowAddDialog] = useState(false)
  const [_showEditDialog, setShowEditDialog] = useState(false)
  const [_showScanDetailsDialog, _setShowScanDetailsDialog] = useState(false)
  const [_currentLibrary, setCurrentLibrary] = useState<any>(null)
  const [_currentScan, _setCurrentScan] = useState<any>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [statusFilter, setStatusFilter] = useState('all')
  const [typeFilter, setTypeFilter] = useState('all')
  const [_sortOption, setSortOption] = useState('name-asc')
  const [selectedLibraries, setSelectedLibraries] = useState<string[]>([])
  const [_formData, setFormData] = useState({
    name: '',
    path: '',
    type: 'movies',
    autoScan: true,
  })

  const { t } = useTranslation()

  const {
    data: libraries,
    error,
    isLoading,
  } = useFetch<LibraryDto[]>('/library/')
  console.log('libraries data', libraries)
  console.log('libraries error', error)

  useEffect(() => {
    if (libraries) {
      setFilteredLibraries(libraries)
    }
  }, [libraries])

  const handleEditLibrary = (library: any) => {
    setCurrentLibrary(library)
    setFormData({
      name: library.name,
      path: library.path,
      type: library.type,
      autoScan: library.autoScan,
    })
    setShowEditDialog(true)
  }

  const handleDeleteLibrary = (_id: string) => {
    // In a real app, you would call an API to delete the library
    toast('媒体库已删除')
  }

  const handleSelectAllLibraries = (checked: boolean) => {
    if (checked) {
      setSelectedLibraries(filteredLibraries.map(lib => lib.id.toString()))
    } else {
      setSelectedLibraries([])
    }
  }

  const handleSelectLibrary = (id: string, checked: boolean) => {
    if (checked) {
      setSelectedLibraries([...selectedLibraries, id])
    } else {
      setSelectedLibraries(selectedLibraries.filter(libId => libId !== id))
    }
  }

  const handleScanLibrary = (id: string) => {
    setActiveScan(id)
    setScanProgress(0)
    toast('扫描已开始')
  }

  return (
    <Card>
      <CardHeader>
        <div className='flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between'>
          <div>
            <CardTitle className='mb-2 flex items-center gap-2'>
              <HardDrive className='h-5 w-5' />
              {t('page.library.libraries.title')}
            </CardTitle>
            <CardDescription>
              {t('page.library.libraries.description')}
            </CardDescription>
          </div>
          <div className='flex items-center gap-2'>
            <Button
              variant='outline'
              size='sm'
              onClick={handleAddLibrary}
              className='gap-2'
            >
              <FolderPlus className='h-4 w-4' />
              <span>{t('page.library.libraries.add_library')}</span>
            </Button>
          </div>
        </div>
      </CardHeader>

      <CardContent>
        {/* Search and Filter */}
        <div className='mb-6 flex flex-col gap-4 sm:flex-row'>
          <div className='relative flex-1'>
            <Search className='absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground' />
            <Input
              type='search'
              placeholder={t('page.library.libraries.search')}
              className='pl-9'
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
            />
          </div>
          <div className='flex gap-2'>
            <Select value={statusFilter} onValueChange={setStatusFilter}>
              <SelectTrigger className='w-[130px]'>
                <SelectValue
                  placeholder={t('page.library.libraries.status.all')}
                />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value='all'>
                  {t('page.library.libraries.status.all')}
                </SelectItem>
                <SelectItem value='active'>
                  {t('page.library.libraries.status.active')}
                </SelectItem>
                <SelectItem value='error'>
                  {t('page.library.libraries.status.error')}
                </SelectItem>
              </SelectContent>
            </Select>

            <Select value={typeFilter} onValueChange={setTypeFilter}>
              <SelectTrigger className='w-[130px]'>
                <SelectValue
                  placeholder={t('page.library.libraries.type.all')}
                />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value='all'>
                  {t('page.library.libraries.type.all')}
                </SelectItem>
                <SelectItem value='movies'>
                  {t('page.library.libraries.type.movies')}
                </SelectItem>
                <SelectItem value='tv'>
                  {t('page.library.libraries.type.tv')}
                </SelectItem>
                <SelectItem value='anime'>
                  {t('page.library.libraries.type.anime')}
                </SelectItem>
              </SelectContent>
            </Select>

            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant='outline' size='icon'>
                  <ArrowUpDown className='h-4 w-4' />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align='end'>
                <DropdownMenuLabel>
                  {t('page.library.libraries.sort.title')}
                </DropdownMenuLabel>
                <DropdownMenuSeparator />
                <DropdownMenuItem onClick={() => setSortOption('name-asc')}>
                  {t('page.library.libraries.sort.name_asc')}
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setSortOption('name-desc')}>
                  {t('page.library.libraries.sort.name_desc')}
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setSortOption('items-desc')}>
                  {t('page.library.libraries.sort.items_desc')}
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setSortOption('items-asc')}>
                  {t('page.library.libraries.sort.items_asc')}
                </DropdownMenuItem>
                <DropdownMenuItem
                  onClick={() => setSortOption('lastScanned-desc')}
                >
                  {t('page.library.libraries.sort.lastScanned_desc')}
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setSortOption('health-desc')}>
                  {t('page.library.libraries.sort.health_desc')}
                </DropdownMenuItem>
                <DropdownMenuItem onClick={() => setSortOption('health-asc')}>
                  {t('page.library.libraries.sort.health_asc')}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>

        {isLoading ? (
          <div className='space-y-4'>
            {[...Array(3)].map((_, i) => (
              <Skeleton key={i} className='h-24 w-full' />
            ))}
          </div>
        ) : filteredLibraries.length === 0 ? (
          <div className='py-8 text-center'>
            <FileQuestion className='mx-auto mb-2 h-12 w-12 text-muted-foreground' />
            <h3 className='text-lg font-medium'>
              {t('page.library.libraries.table.no_libraries.title')}
            </h3>
            <p className='mb-4 text-muted-foreground'>
              {searchQuery || statusFilter !== 'all' || typeFilter !== 'all'
                ? t('page.library.libraries.table.no_libraries.adjust_search')
                : t('page.library.libraries.table.no_libraries.add_library')}
            </p>
            {!searchQuery && statusFilter === 'all' && typeFilter === 'all' && (
              <Button onClick={handleAddLibrary} className='gap-2'>
                <FolderPlus className='h-4 w-4' />
                <span>{t('page.library.libraries.add_library')}</span>
              </Button>
            )}
          </div>
        ) : (
          <div className='rounded-md border'>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className='w-[40px]'>
                    <Checkbox
                      checked={
                        selectedLibraries.length > 0 &&
                        selectedLibraries.length === filteredLibraries.length
                      }
                      onCheckedChange={handleSelectAllLibraries}
                      aria-label='Select all libraries'
                    />
                  </TableHead>
                  <TableHead>
                    {t('page.library.libraries.table.columns.name')}
                  </TableHead>
                  <TableHead className='hidden md:table-cell'>
                    {t('page.library.libraries.table.columns.path')}
                  </TableHead>
                  <TableHead className='hidden md:table-cell'>
                    {t('page.library.libraries.table.columns.type')}
                  </TableHead>
                  <TableHead className='hidden lg:table-cell'>
                    {t('page.library.libraries.table.columns.items')}
                  </TableHead>
                  <TableHead className='hidden lg:table-cell'>
                    {t('page.library.libraries.table.columns.lastScanned')}
                  </TableHead>
                  <TableHead className='hidden xl:table-cell'>
                    {t('page.library.libraries.table.columns.health')}
                  </TableHead>
                  <TableHead className='hidden xl:table-cell'>
                    {t('page.library.libraries.table.columns.storage')}
                  </TableHead>
                  <TableHead className='text-right'>
                    {t('page.library.libraries.table.columns.actions')}
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {filteredLibraries.map(library => (
                  <TableRow key={library.id.toString()}>
                    <TableCell>
                      <Checkbox
                        checked={selectedLibraries.includes(
                          library.id.toString()
                        )}
                        onCheckedChange={checked =>
                          handleSelectLibrary(library.id.toString(), !!checked)
                        }
                        aria-label={`Select ${library.name}`}
                      />
                    </TableCell>
                    <TableCell>
                      <div className='flex items-center gap-2'>
                        {library.category === 'Movie' ? (
                          <MonitorPlay className='h-4 w-4 text-primary' />
                        ) : library.category === 'TvShow' ? (
                          <FileVideo className='h-4 w-4 text-primary' />
                        ) : (
                          <FolderOpen className='h-4 w-4 text-primary' />
                        )}
                        <div>
                          <div className='font-medium'>{library.name}</div>
                          {library.current_status === 'Error' && (
                            <Badge variant='destructive' className='mt-1'>
                              {t('page.library.libraries.table.cell.error')}
                            </Badge>
                          )}
                        </div>
                      </div>
                    </TableCell>
                    <TableCell className='hidden md:table-cell'>
                      <span className='block max-w-[200px] truncate text-sm text-muted-foreground'>
                        {library.directory}
                      </span>
                    </TableCell>
                    <TableCell className='hidden md:table-cell'>
                      <Badge variant='secondary'>
                        {library.category === 'Movie'
                          ? t('page.library.libraries.table.cell.movies')
                          : library.category === 'TvShow'
                            ? t('page.library.libraries.table.cell.tv')
                            : t('page.library.libraries.table.cell.mixed')}
                      </Badge>
                    </TableCell>
                    <TableCell className='hidden lg:table-cell'>
                      {library.item_count.toString()}
                    </TableCell>
                    <TableCell className='hidden lg:table-cell'>
                      <span className='text-sm text-muted-foreground'>
                        {library.last_scanned
                          ? format(
                              parseISO(library.last_scanned),
                              'yyyy-MM-dd HH:mm:ss'
                            )
                          : 'N/A'}
                      </span>
                    </TableCell>
                    <TableCell className='hidden xl:table-cell'>
                      {library.current_status === 'Active' ? (
                        <div className='flex items-center gap-2'>
                          <Progress
                            value={Number(library.health_score) || 0}
                            className='h-2 w-16'
                            style={
                              {
                                backgroundColor: 'rgba(0,0,0,0.1)',
                                '--progress-background':
                                  Number(library.health_score) >= 90
                                    ? 'rgba(34, 197, 94, 0.8)'
                                    : Number(library.health_score) >= 70
                                      ? 'rgba(245, 158, 11, 0.8)'
                                      : 'rgba(239, 68, 68, 0.8)',
                              } as React.CSSProperties
                            }
                          />
                          <span
                            className={`text-sm ${Number(library.health_score) >= 90 ? 'text-green-500' : Number(library.health_score) >= 70 ? 'text-amber-500' : 'text-red-500'}`}
                          >
                            {Number(library.health_score) || 0}%
                          </span>
                        </div>
                      ) : (
                        <span className='text-sm text-destructive'>N/A</span>
                      )}
                    </TableCell>
                    <TableCell className='hidden xl:table-cell'>
                      <span className='text-sm text-muted-foreground'>
                        {library.storage_used.toString()}
                      </span>
                    </TableCell>
                    <TableCell className='text-right'>
                      <div className='flex items-center justify-end gap-2'>
                        <TooltipProvider>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Button
                                variant='ghost'
                                size='icon'
                                disabled={
                                  !!activeScan ||
                                  library.current_status === 'Error'
                                }
                                onClick={() =>
                                  handleScanLibrary(library.id.toString())
                                }
                              >
                                <RefreshCw className='h-4 w-4' />
                              </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                              <p>
                                {t('page.library.libraries.table.action.scan')}
                              </p>
                            </TooltipContent>
                          </Tooltip>
                        </TooltipProvider>

                        <TooltipProvider>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Button
                                variant='ghost'
                                size='icon'
                                onClick={() => handleEditLibrary(library)}
                              >
                                <Edit className='h-4 w-4' />
                              </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                              <p>
                                {t('page.library.libraries.table.action.edit')}
                              </p>
                            </TooltipContent>
                          </Tooltip>
                        </TooltipProvider>

                        <TooltipProvider>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Button
                                variant='ghost'
                                size='icon'
                                onClick={() =>
                                  handleDeleteLibrary(library.id.toString())
                                }
                              >
                                <Trash className='h-4 w-4 text-destructive' />
                              </Button>
                            </TooltipTrigger>
                            <TooltipContent>
                              <p>
                                {t(
                                  'page.library.libraries.table.action.delete'
                                )}
                              </p>
                            </TooltipContent>
                          </Tooltip>
                        </TooltipProvider>
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        )}
      </CardContent>

      <CardFooter className='flex justify-between'>
        <div className='text-sm text-muted-foreground'>
          {filteredLibraries.length > 0
            ? `${t('page.library.libraries.footer.displayed', {
                count: filteredLibraries.length,
                total: libraries?.length,
              })}`
            : `${t('page.library.libraries.footer.total', {
                total: libraries?.length,
              })}`}
        </div>
      </CardFooter>
    </Card>
  )
}
