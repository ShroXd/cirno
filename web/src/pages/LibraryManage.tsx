import type React from 'react'
import { useEffect, useRef, useState } from 'react'
import { Link } from 'react-router-dom'

import {
  AlertCircle,
  ArrowLeft,
  ArrowUpDown,
  BarChart3,
  Clock,
  Database,
  Edit,
  FileQuestion,
  FileVideo,
  FolderOpen,
  FolderPlus,
  HardDrive,
  History,
  Loader2,
  MonitorPlay,
  MoreHorizontal,
  PieChart,
  RefreshCw,
  Search,
  Settings,
  Trash,
  X,
} from 'lucide-react'
import { toast } from 'sonner'

import { LibraryManageDialog } from '~/components/LibraryManageDialog/LibraryManageDialog'
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from '~/components/ui/accordion'
import { Alert, AlertDescription, AlertTitle } from '~/components/ui/alert'
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
import { Checkbox } from '~/components/ui/checkbox'
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '~/components/ui/dialog'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '~/components/ui/dropdown-menu'
import { Input } from '~/components/ui/input'
import { Label } from '~/components/ui/label'
import { Progress } from '~/components/ui/progress'
import { RadioGroup, RadioGroupItem } from '~/components/ui/radio-group'
import { ScrollArea } from '~/components/ui/scroll-area'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { SidebarTrigger } from '~/components/ui/sidebar'
import { Skeleton } from '~/components/ui/skeleton'
import { Toaster } from '~/components/ui/sonner'
import { Switch } from '~/components/ui/switch'
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '~/components/ui/table'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '~/components/ui/tabs'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '~/components/ui/tooltip'

// Mock data for media libraries
const mockLibraries = [
  {
    id: 'lib-1',
    name: '电影库',
    path: '/media/movies',
    type: 'movies',
    itemCount: 87,
    lastScanned: '2023-12-15 14:30',
    status: 'active',
    autoScan: true,
    storageUsed: '256.4 GB',
    storageTotal: '1 TB',
    healthScore: 95,
    mediaTypes: {
      mp4: 45,
      mkv: 32,
      avi: 10,
    },
    resolutions: {
      '4K': 15,
      '1080p': 52,
      '720p': 20,
    },
  },
  {
    id: 'lib-2',
    name: '电视剧库',
    path: '/media/tv-shows',
    type: 'tv',
    itemCount: 24,
    lastScanned: '2023-12-14 09:15',
    status: 'active',
    autoScan: true,
    storageUsed: '512.7 GB',
    storageTotal: '2 TB',
    healthScore: 88,
    mediaTypes: {
      mp4: 10,
      mkv: 14,
    },
    resolutions: {
      '4K': 5,
      '1080p': 15,
      '720p': 4,
    },
  },
  {
    id: 'lib-3',
    name: '动画库',
    path: '/media/animation',
    type: 'movies',
    itemCount: 45,
    lastScanned: '2023-12-10 18:45',
    status: 'active',
    autoScan: false,
    storageUsed: '128.3 GB',
    storageTotal: '500 GB',
    healthScore: 92,
    mediaTypes: {
      mp4: 25,
      mkv: 20,
    },
    resolutions: {
      '1080p': 30,
      '720p': 15,
    },
  },
  {
    id: 'lib-4',
    name: '纪录片库',
    path: '/media/documentaries',
    type: 'movies',
    itemCount: 32,
    lastScanned: '2023-12-05 11:20',
    status: 'error',
    autoScan: true,
    error: '路径不可访问',
    storageUsed: '96.5 GB',
    storageTotal: '500 GB',
    healthScore: 45,
    mediaTypes: {
      mp4: 20,
      mkv: 12,
    },
    resolutions: {
      '1080p': 22,
      '720p': 10,
    },
  },
]

// Mock data for recent scans
const recentScans = [
  {
    id: 'scan-1',
    libraryId: 'lib-1',
    libraryName: '电影库',
    startTime: '2023-12-15 14:30',
    endTime: '2023-12-15 14:45',
    duration: '15 分钟',
    itemsScanned: 87,
    itemsAdded: 3,
    itemsUpdated: 5,
    itemsRemoved: 0,
    status: 'completed',
    initiatedBy: '用户',
  },
  {
    id: 'scan-2',
    libraryId: 'lib-2',
    libraryName: '电视剧库',
    startTime: '2023-12-14 09:15',
    endTime: '2023-12-14 09:30',
    duration: '15 分钟',
    itemsScanned: 24,
    itemsAdded: 2,
    itemsUpdated: 8,
    itemsRemoved: 1,
    status: 'completed',
    initiatedBy: '计划任务',
  },
  {
    id: 'scan-3',
    libraryId: 'lib-4',
    libraryName: '纪录片库',
    startTime: '2023-12-05 11:20',
    endTime: null,
    duration: 'N/A',
    itemsScanned: 0,
    itemsAdded: 0,
    itemsUpdated: 0,
    itemsRemoved: 0,
    status: 'failed',
    error: '路径不可访问',
    initiatedBy: '用户',
  },
  {
    id: 'scan-4',
    libraryId: 'lib-3',
    libraryName: '动画库',
    startTime: '2023-12-10 18:45',
    endTime: '2023-12-10 19:00',
    duration: '15 分钟',
    itemsScanned: 45,
    itemsAdded: 0,
    itemsUpdated: 3,
    itemsRemoved: 0,
    status: 'completed',
    initiatedBy: '用户',
  },
  {
    id: 'scan-5',
    libraryId: 'lib-1',
    libraryName: '电影库',
    startTime: '2023-12-10 10:30',
    endTime: '2023-12-10 10:45',
    duration: '15 分钟',
    itemsScanned: 84,
    itemsAdded: 0,
    itemsUpdated: 0,
    itemsRemoved: 0,
    status: 'completed',
    initiatedBy: '计划任务',
  },
]

// 修改 libraryTypes 数组，确保类型是预定义的
const libraryTypes = [
  { value: 'movies', label: '电影', icon: MonitorPlay },
  { value: 'tv', label: '电视剧', icon: FileVideo },
  { value: 'animation', label: '动画', icon: FileVideo },
  { value: 'documentary', label: '纪录片', icon: FileVideo },
  { value: 'music', label: '音乐', icon: FolderOpen },
  { value: 'other', label: '其他', icon: FolderOpen },
]

// Mock storage data
const storageData = {
  totalSpace: '4 TB',
  usedSpace: '994 GB',
  freeSpace: '3.03 TB',
  usagePercentage: 24.3,
  libraries: [
    { name: '电影库', used: '256.4 GB', percentage: 25.8 },
    { name: '电视剧库', used: '512.7 GB', percentage: 51.6 },
    { name: '动画库', used: '128.3 GB', percentage: 12.9 },
    { name: '纪录片库', used: '96.5 GB', percentage: 9.7 },
  ],
}

export default function LibrariesPage() {
  const [isLoading, setIsLoading] = useState(true)
  const [libraries, setLibraries] = useState(mockLibraries)
  const [filteredLibraries, setFilteredLibraries] = useState(mockLibraries)
  const [activeScan, setActiveScan] = useState<string | null>(null)
  const [scanProgress, setScanProgress] = useState(0)
  const [showAddDialog, setShowAddDialog] = useState(false)
  const [showEditDialog, setShowEditDialog] = useState(false)
  const [showScanDetailsDialog, setShowScanDetailsDialog] = useState(false)
  const [currentLibrary, setCurrentLibrary] = useState<any>(null)
  const [currentScan, setCurrentScan] = useState<any>(null)
  const [searchQuery, setSearchQuery] = useState('')
  const [statusFilter, setStatusFilter] = useState('all')
  const [typeFilter, setTypeFilter] = useState('all')
  const [sortOption, setSortOption] = useState('name-asc')
  const [selectedLibraries, setSelectedLibraries] = useState<string[]>([])
  const [formData, setFormData] = useState({
    name: '',
    path: '',
    type: 'movies',
    autoScan: true,
  })

  // Ref for the storage chart canvas
  const storageChartRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    // Simulate loading data
    const timer = setTimeout(() => {
      setIsLoading(false)
    }, 1500)

    return () => clearTimeout(timer)
  }, [])

  // Filter and sort libraries
  useEffect(() => {
    let result = [...libraries]

    // Apply search filter
    if (searchQuery) {
      const query = searchQuery.toLowerCase()
      result = result.filter(
        lib =>
          lib.name.toLowerCase().includes(query) ||
          lib.path.toLowerCase().includes(query)
      )
    }

    // Apply status filter
    if (statusFilter !== 'all') {
      result = result.filter(lib =>
        statusFilter === 'active'
          ? lib.status === 'active'
          : lib.status === 'error'
      )
    }

    // Apply type filter
    if (typeFilter !== 'all') {
      result = result.filter(lib => lib.type === typeFilter)
    }

    // Apply sorting
    const [sortField, sortDirection] = sortOption.split('-')
    result.sort((a, b) => {
      let comparison = 0

      switch (sortField) {
        case 'name':
          comparison = a.name.localeCompare(b.name)
          break
        case 'items':
          comparison = a.itemCount - b.itemCount
          break
        case 'lastScanned':
          comparison =
            new Date(a.lastScanned).getTime() -
            new Date(b.lastScanned).getTime()
          break
        case 'health':
          comparison = (a.healthScore || 0) - (b.healthScore || 0)
          break
        default:
          comparison = 0
      }

      return sortDirection === 'asc' ? comparison : -comparison
    })

    setFilteredLibraries(result)
  }, [libraries, searchQuery, statusFilter, typeFilter, sortOption])

  // Simulate an active scan with progress
  useEffect(() => {
    if (activeScan) {
      const interval = setInterval(() => {
        setScanProgress(prev => {
          if (prev >= 100) {
            setActiveScan(null)
            clearInterval(interval)
            toast('扫描完成')
            return 0
          }
          return prev + 5
        })
      }, 500)

      return () => clearInterval(interval)
    }
  }, [activeScan, toast])

  // 修改 useEffect 中的 drawStorageChart 函数，移除中心圆显示未使用空间的部分
  useEffect(() => {
    if (isLoading || !storageChartRef.current) return

    const ctx = storageChartRef.current.getContext('2d')
    if (!ctx) return

    // Clear canvas
    ctx.clearRect(
      0,
      0,
      storageChartRef.current.width,
      storageChartRef.current.height
    )

    // Set dimensions
    const width = storageChartRef.current.width
    const height = storageChartRef.current.height
    const centerX = width / 2
    const centerY = height / 2
    const radius = Math.min(centerX, centerY) - 10

    // Colors for each library
    const colors = [
      'rgba(147, 51, 234, 0.8)', // Purple (primary)
      'rgba(59, 130, 246, 0.8)', // Blue
      'rgba(236, 72, 153, 0.8)', // Pink
      'rgba(16, 185, 129, 0.8)', // Green
      'rgba(245, 158, 11, 0.8)', // Amber
      'rgba(239, 68, 68, 0.8)', // Red
    ]

    // Draw segments
    let startAngle = 0
    storageData.libraries.forEach((lib, index) => {
      const sliceAngle = (lib.percentage / 100) * 2 * Math.PI

      ctx.beginPath()
      ctx.moveTo(centerX, centerY)
      ctx.arc(centerX, centerY, radius, startAngle, startAngle + sliceAngle)
      ctx.closePath()

      ctx.fillStyle = colors[index % colors.length]
      ctx.fill()

      // Add label
      const labelAngle = startAngle + sliceAngle / 2
      const labelX = centerX + radius * 0.7 * Math.cos(labelAngle)
      const labelY = centerY + radius * 0.7 * Math.sin(labelAngle)

      ctx.fillStyle = 'white'
      ctx.font = '12px sans-serif'
      ctx.textAlign = 'center'
      ctx.textBaseline = 'middle'
      ctx.fillText(`${lib.percentage}%`, labelX, labelY)

      startAngle += sliceAngle
    })
  }, [isLoading])

  const handleAddLibrary = () => {
    setFormData({
      name: '',
      path: '',
      type: 'movies',
      autoScan: true,
    })
    setShowAddDialog(true)
  }

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

  const handleDeleteLibrary = (id: string) => {
    // In a real app, you would call an API to delete the library
    setLibraries(libraries.filter(lib => lib.id !== id))
    setSelectedLibraries(selectedLibraries.filter(libId => libId !== id))
    toast('媒体库已删除')
  }

  const handleBatchDelete = () => {
    if (selectedLibraries.length === 0) return

    // In a real app, you would call an API to delete multiple libraries
    setLibraries(libraries.filter(lib => !selectedLibraries.includes(lib.id)))
    setSelectedLibraries([])
    toast('批量删除完成')
  }

  const handleScanLibrary = (id: string) => {
    setActiveScan(id)
    setScanProgress(0)
    toast('扫描已开始')
  }

  const handleBatchScan = () => {
    if (selectedLibraries.length === 0) return

    // In a real app, you would call an API to scan multiple libraries
    // For demo, we'll just scan the first selected library
    handleScanLibrary(selectedLibraries[0])
    toast('批量扫描已开始')
  }

  const handleSaveLibrary = (isNew = true) => {
    if (isNew) {
      // In a real app, you would call an API to add the library
      const newLibrary = {
        id: `lib-${Date.now()}`,
        name: formData.name,
        path: formData.path,
        type: formData.type,
        itemCount: 0,
        lastScanned: '从未',
        status: 'active',
        autoScan: formData.autoScan,
        storageUsed: '0 GB',
        storageTotal: '1 TB',
        healthScore: 100,
        mediaTypes: {},
        resolutions: {},
      }
      //   setLibraries([...libraries, newLibrary])
      setShowAddDialog(false)
      toast('媒体库已添加')
    } else {
      // In a real app, you would call an API to update the library
      setLibraries(
        libraries.map(lib =>
          lib.id === currentLibrary.id ? { ...lib, ...formData } : lib
        )
      )
      setShowEditDialog(false)
      toast('媒体库已更新')
    }
  }

  const handleBrowsePath = () => {
    // In a real app, this would open a file browser dialog
    // For this demo, we'll just simulate it
    setTimeout(() => {
      setFormData({
        ...formData,
        path: '/selected/media/path',
      })
    }, 500)
  }

  const handleViewScanDetails = (scan: any) => {
    setCurrentScan(scan)
    setShowScanDetailsDialog(true)
  }

  const handleSelectLibrary = (id: string, checked: boolean) => {
    if (checked) {
      setSelectedLibraries([...selectedLibraries, id])
    } else {
      setSelectedLibraries(selectedLibraries.filter(libId => libId !== id))
    }
  }

  const handleSelectAllLibraries = (checked: boolean) => {
    if (checked) {
      setSelectedLibraries(filteredLibraries.map(lib => lib.id))
    } else {
      setSelectedLibraries([])
    }
  }

  // Animation variants
  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.05,
      },
    },
  }

  const itemVariants = {
    hidden: { opacity: 0, y: 10 },
    visible: {
      opacity: 1,
      y: 0,
      transition: {
        type: 'spring',
        stiffness: 100,
        damping: 15,
      },
    },
  }

  return (
    <>
      <Toaster />
      <div className='h-screen w-full overflow-y-auto bg-background px-4 pb-6 md:px-6'>
        <header className='sticky top-0 z-10 w-full border-b border-border/40 bg-background/80 backdrop-blur-md'>
          <div className='flex h-16 items-center px-4'>
            <SidebarTrigger className='mr-4 md:hidden' />
            <Button variant='ghost' size='icon' className='mr-2' asChild>
              <Link to='/'>
                <ArrowLeft className='h-5 w-5' />
                <span className='sr-only'>Back</span>
              </Link>
            </Button>
            <h1 className='text-xl font-bold'>媒体库管理</h1>
            <div className='flex-1' />
            <div className='flex items-center gap-2'>
              {selectedLibraries.length > 0 && (
                <>
                  <Button
                    variant='outline'
                    size='sm'
                    className='gap-2'
                    onClick={handleBatchScan}
                    disabled={!!activeScan}
                  >
                    <RefreshCw className='h-4 w-4' />
                    <span>批量扫描</span>
                  </Button>
                  <Button
                    variant='outline'
                    size='sm'
                    className='gap-2 text-destructive hover:text-destructive'
                    onClick={handleBatchDelete}
                  >
                    <Trash className='h-4 w-4' />
                    <span>批量删除</span>
                  </Button>
                  <Button
                    variant='ghost'
                    size='sm'
                    className='gap-2'
                    onClick={() => setSelectedLibraries([])}
                  >
                    <X className='h-4 w-4' />
                    <span>取消选择</span>
                  </Button>
                </>
              )}
              <Button onClick={handleAddLibrary} className='gap-2'>
                <FolderPlus className='h-4 w-4' />
                <span className='hidden sm:inline'>添加媒体库</span>
              </Button>
            </div>
          </div>
        </header>

        <main className='mx-auto max-w-7xl px-4 py-6'>
          {/* Active Scan Alert */}
          {activeScan && (
            <Alert className='mb-6'>
              <Loader2 className='h-4 w-4 animate-spin' />
              <AlertTitle>正在扫描媒体库</AlertTitle>
              <AlertDescription>
                <div className='mt-2'>
                  <div className='mb-1 flex justify-between text-sm'>
                    <span>扫描进度</span>
                    <span>{scanProgress}%</span>
                  </div>
                  <Progress value={scanProgress} className='h-2' />
                </div>
              </AlertDescription>
            </Alert>
          )}

          {/* Main Tabs */}
          <Tabs defaultValue='overview' className='mb-8'>
            <TabsList className='mb-4'>
              <TabsTrigger value='overview' className='gap-2'>
                <BarChart3 className='h-4 w-4' />
                <span>概览</span>
              </TabsTrigger>
              <TabsTrigger value='libraries' className='gap-2'>
                <HardDrive className='h-4 w-4' />
                <span>媒体库</span>
              </TabsTrigger>
              <TabsTrigger value='history' className='gap-2'>
                <History className='h-4 w-4' />
                <span>扫描历史</span>
              </TabsTrigger>
              <TabsTrigger value='settings' className='gap-2'>
                <Settings className='h-4 w-4' />
                <span>设置</span>
              </TabsTrigger>
            </TabsList>

            {/* Overview Tab */}
            <TabsContent value='overview' className='mt-0'>
              <div className='grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3'>
                {/* 存储概览卡片 */}
                <Card className='lg:col-span-1'>
                  <CardHeader>
                    <CardTitle className='flex items-center gap-2'>
                      <Database className='h-5 w-5' />
                      媒体库存储
                    </CardTitle>
                    <CardDescription>各媒体库存储占比</CardDescription>
                  </CardHeader>
                  <CardContent>
                    {isLoading ? (
                      <div className='space-y-4'>
                        <Skeleton className='h-[200px] w-full' />
                      </div>
                    ) : (
                      <div>
                        <div className='relative mx-auto aspect-square max-w-[250px]'>
                          <canvas
                            ref={storageChartRef}
                            width={250}
                            height={250}
                            className='mx-auto'
                          />
                        </div>
                        <div className='mt-4'>
                          <h3 className='mb-2 text-lg font-medium'>
                            媒体库存储分布
                          </h3>
                          <div className='space-y-3'>
                            {storageData.libraries.map((lib, index) => (
                              <div key={index} className='space-y-1'>
                                <div className='flex justify-between text-sm'>
                                  <span className='text-muted-foreground'>
                                    {lib.name}:
                                  </span>
                                  <span className='font-medium'>
                                    {lib.used} ({lib.percentage}%)
                                  </span>
                                </div>
                                <Progress
                                  value={lib.percentage}
                                  className='h-2'
                                  style={
                                    {
                                      backgroundColor: 'rgba(0,0,0,0.1)',
                                      '--progress-background':
                                        index === 0
                                          ? 'rgba(147, 51, 234, 0.8)'
                                          : index === 1
                                            ? 'rgba(59, 130, 246, 0.8)'
                                            : index === 2
                                              ? 'rgba(236, 72, 153, 0.8)'
                                              : 'rgba(16, 185, 129, 0.8)',
                                    } as React.CSSProperties
                                  }
                                />
                              </div>
                            ))}
                          </div>
                        </div>
                      </div>
                    )}
                  </CardContent>
                </Card>

                {/* Library Stats Card */}
                <Card className='lg:col-span-2'>
                  <CardHeader>
                    <CardTitle className='flex items-center gap-2'>
                      <PieChart className='h-5 w-5' />
                      媒体库统计
                    </CardTitle>
                    <CardDescription>媒体库数量和内容统计</CardDescription>
                  </CardHeader>
                  <CardContent>
                    {isLoading ? (
                      <div className='space-y-4'>
                        <Skeleton className='h-10 w-full' />
                        <Skeleton className='h-10 w-full' />
                        <Skeleton className='h-10 w-full' />
                      </div>
                    ) : (
                      <div className='space-y-6'>
                        <div>
                          <h3 className='mb-2 text-lg font-medium'>
                            媒体库概况
                          </h3>
                          <div className='grid grid-cols-2 gap-4 md:grid-cols-4'>
                            <div className='rounded-lg bg-muted/30 p-3 text-center'>
                              <p className='text-3xl font-bold'>
                                {libraries.length}
                              </p>
                              <p className='text-sm text-muted-foreground'>
                                总媒体库
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-3 text-center'>
                              <p className='text-3xl font-bold'>
                                {libraries.reduce(
                                  (sum, lib) => sum + lib.itemCount,
                                  0
                                )}
                              </p>
                              <p className='text-sm text-muted-foreground'>
                                总媒体文件
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-3 text-center'>
                              <p className='text-3xl font-bold'>
                                {
                                  libraries.filter(
                                    lib => lib.status === 'active'
                                  ).length
                                }
                              </p>
                              <p className='text-sm text-muted-foreground'>
                                正常媒体库
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-3 text-center'>
                              <p className='text-3xl font-bold'>
                                {
                                  libraries.filter(
                                    lib => lib.status === 'error'
                                  ).length
                                }
                              </p>
                              <p className='text-sm text-muted-foreground'>
                                错误媒体库
                              </p>
                            </div>
                          </div>
                        </div>

                        <div>
                          <h3 className='mb-2 text-lg font-medium'>
                            媒体类型分布
                          </h3>
                          <div className='grid grid-cols-3 gap-2 text-center md:grid-cols-6'>
                            <div className='rounded-lg bg-muted/30 p-2'>
                              <MonitorPlay className='mx-auto mb-1 h-5 w-5 text-primary' />
                              <p className='text-xl font-bold'>
                                {
                                  libraries.filter(lib => lib.type === 'movies')
                                    .length
                                }
                              </p>
                              <p className='text-xs text-muted-foreground'>
                                电影
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-2'>
                              <FileVideo className='mx-auto mb-1 h-5 w-5 text-primary' />
                              <p className='text-xl font-bold'>
                                {
                                  libraries.filter(lib => lib.type === 'tv')
                                    .length
                                }
                              </p>
                              <p className='text-xs text-muted-foreground'>
                                电视剧
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-2'>
                              <FileVideo className='mx-auto mb-1 h-5 w-5 text-primary' />
                              <p className='text-xl font-bold'>
                                {
                                  libraries.filter(
                                    lib => lib.type === 'animation'
                                  ).length
                                }
                              </p>
                              <p className='text-xs text-muted-foreground'>
                                动画
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-2'>
                              <FileVideo className='mx-auto mb-1 h-5 w-5 text-primary' />
                              <p className='text-xl font-bold'>
                                {
                                  libraries.filter(
                                    lib => lib.type === 'documentary'
                                  ).length
                                }
                              </p>
                              <p className='text-xs text-muted-foreground'>
                                纪录片
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-2'>
                              <FolderOpen className='mx-auto mb-1 h-5 w-5 text-primary' />
                              <p className='text-xl font-bold'>
                                {
                                  libraries.filter(lib => lib.type === 'music')
                                    .length
                                }
                              </p>
                              <p className='text-xs text-muted-foreground'>
                                音乐
                              </p>
                            </div>
                            <div className='rounded-lg bg-muted/30 p-2'>
                              <FolderOpen className='mx-auto mb-1 h-5 w-5 text-primary' />
                              <p className='text-xl font-bold'>
                                {
                                  libraries.filter(lib => lib.type === 'other')
                                    .length
                                }
                              </p>
                              <p className='text-xs text-muted-foreground'>
                                其他
                              </p>
                            </div>
                          </div>
                        </div>

                        <div>
                          <h3 className='mb-2 text-lg font-medium'>
                            媒体库健康状况
                          </h3>
                          <div className='space-y-3'>
                            {libraries.map(lib => (
                              <div key={lib.id} className='space-y-1'>
                                <div className='flex justify-between text-sm'>
                                  <span className='max-w-[70%] truncate text-muted-foreground'>
                                    {lib.name}:
                                  </span>
                                  <span
                                    className={`font-medium ${lib.healthScore >= 90 ? 'text-green-500' : lib.healthScore >= 70 ? 'text-amber-500' : 'text-red-500'}`}
                                  >
                                    {lib.healthScore || 0}%
                                  </span>
                                </div>
                                <Progress
                                  value={lib.healthScore || 0}
                                  className='h-2'
                                  style={
                                    {
                                      backgroundColor: 'rgba(0,0,0,0.1)',
                                      '--progress-background':
                                        lib.healthScore >= 90
                                          ? 'rgba(34, 197, 94, 0.8)'
                                          : lib.healthScore >= 70
                                            ? 'rgba(245, 158, 11, 0.8)'
                                            : 'rgba(239, 68, 68, 0.8)',
                                    } as React.CSSProperties
                                  }
                                />
                              </div>
                            ))}
                          </div>
                        </div>
                      </div>
                    )}
                  </CardContent>
                </Card>

                {/* Recent Activity Card */}
                <Card className='lg:col-span-3'>
                  <CardHeader>
                    <CardTitle className='flex items-center gap-2'>
                      <Clock className='h-5 w-5' />
                      最近活动
                    </CardTitle>
                    <CardDescription>最近的扫描和更改</CardDescription>
                  </CardHeader>
                  <CardContent>
                    {isLoading ? (
                      <div className='space-y-4'>
                        {[...Array(3)].map((_, i) => (
                          <Skeleton key={i} className='h-16 w-full' />
                        ))}
                      </div>
                    ) : (
                      <div className='space-y-4'>
                        {recentScans.slice(0, 3).map(scan => (
                          <div
                            key={scan.id}
                            className={`rounded-lg border p-3 ${scan.status === 'failed' ? 'border-destructive/50 bg-destructive/5' : 'border-border bg-muted/30'}`}
                          >
                            <div className='flex items-start justify-between'>
                              <div>
                                <div className='flex items-center gap-2'>
                                  <h3 className='font-medium'>
                                    {scan.libraryName}
                                  </h3>
                                  {scan.status === 'completed' && (
                                    <Badge
                                      variant='outline'
                                      className='bg-green-500/10 text-green-500 hover:bg-green-500/20 hover:text-green-600'
                                    >
                                      完成
                                    </Badge>
                                  )}
                                  {scan.status === 'failed' && (
                                    <Badge variant='destructive'>失败</Badge>
                                  )}
                                </div>
                                <p className='text-sm text-muted-foreground'>
                                  {scan.startTime}{' '}
                                  {scan.status === 'completed'
                                    ? `(耗时: ${scan.duration})`
                                    : ''}
                                </p>
                              </div>
                              <div className='text-right'>
                                {scan.status === 'completed' && (
                                  <div className='text-sm'>
                                    <span className='text-green-500'>
                                      +{scan.itemsAdded}
                                    </span>{' '}
                                    /
                                    <span className='text-amber-500'>
                                      {' '}
                                      ~{scan.itemsUpdated}
                                    </span>{' '}
                                    /
                                    <span className='text-red-500'>
                                      {' '}
                                      -{scan.itemsRemoved}
                                    </span>
                                  </div>
                                )}
                                {scan.status === 'failed' && (
                                  <p className='text-sm text-destructive'>
                                    {scan.error}
                                  </p>
                                )}
                              </div>
                            </div>
                          </div>
                        ))}
                        <div className='text-center'>
                          <Button variant='ghost' size='sm' onClick={() => {}}>
                            查看所有历史记录
                          </Button>
                        </div>
                      </div>
                    )}
                  </CardContent>
                </Card>
              </div>
            </TabsContent>

            {/* Libraries Tab */}
            <TabsContent value='libraries' className='mt-0'>
              <Card>
                <CardHeader>
                  <div className='flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between'>
                    <div>
                      <CardTitle className='flex items-center gap-2'>
                        <HardDrive className='h-5 w-5' />
                        媒体库列表
                      </CardTitle>
                      <CardDescription>
                        管理您的媒体库路径和扫描设置
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
                        <span>添加媒体库</span>
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
                        placeholder='搜索媒体库...'
                        className='pl-9'
                        value={searchQuery}
                        onChange={e => setSearchQuery(e.target.value)}
                      />
                    </div>
                    <div className='flex gap-2'>
                      <Select
                        value={statusFilter}
                        onValueChange={setStatusFilter}
                      >
                        <SelectTrigger className='w-[130px]'>
                          <SelectValue placeholder='状态' />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value='all'>所有状态</SelectItem>
                          <SelectItem value='active'>正常</SelectItem>
                          <SelectItem value='error'>错误</SelectItem>
                        </SelectContent>
                      </Select>

                      {/* 修改 Select 组件中的类型选项，确保与 libraryTypes 数组一致 */}
                      <Select value={typeFilter} onValueChange={setTypeFilter}>
                        <SelectTrigger className='w-[130px]'>
                          <SelectValue placeholder='类型' />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value='all'>所有类型</SelectItem>
                          <SelectItem value='movies'>电影</SelectItem>
                          <SelectItem value='tv'>电视剧</SelectItem>
                          <SelectItem value='animation'>动画</SelectItem>
                          <SelectItem value='documentary'>纪录片</SelectItem>
                          <SelectItem value='music'>音乐</SelectItem>
                          <SelectItem value='other'>其他</SelectItem>
                        </SelectContent>
                      </Select>

                      <DropdownMenu>
                        <DropdownMenuTrigger asChild>
                          <Button variant='outline' size='icon'>
                            <ArrowUpDown className='h-4 w-4' />
                          </Button>
                        </DropdownMenuTrigger>
                        <DropdownMenuContent align='end'>
                          <DropdownMenuLabel>排序方式</DropdownMenuLabel>
                          <DropdownMenuSeparator />
                          <DropdownMenuItem
                            onClick={() => setSortOption('name-asc')}
                          >
                            名称 (A-Z)
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            onClick={() => setSortOption('name-desc')}
                          >
                            名称 (Z-A)
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            onClick={() => setSortOption('items-desc')}
                          >
                            项目数 (多-少)
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            onClick={() => setSortOption('items-asc')}
                          >
                            项目数 (少-多)
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            onClick={() => setSortOption('lastScanned-desc')}
                          >
                            最近扫描
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            onClick={() => setSortOption('health-desc')}
                          >
                            健康状况 (高-低)
                          </DropdownMenuItem>
                          <DropdownMenuItem
                            onClick={() => setSortOption('health-asc')}
                          >
                            健康状况 (低-高)
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
                      <h3 className='text-lg font-medium'>没有找到媒体库</h3>
                      <p className='mb-4 text-muted-foreground'>
                        {searchQuery ||
                        statusFilter !== 'all' ||
                        typeFilter !== 'all'
                          ? '尝试调整搜索条件或筛选器'
                          : '添加您的第一个媒体库以开始管理您的媒体内容'}
                      </p>
                      {!searchQuery &&
                        statusFilter === 'all' &&
                        typeFilter === 'all' && (
                          <Button onClick={handleAddLibrary} className='gap-2'>
                            <FolderPlus className='h-4 w-4' />
                            <span>添加媒体库</span>
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
                                  selectedLibraries.length ===
                                    filteredLibraries.length
                                }
                                onCheckedChange={handleSelectAllLibraries}
                                aria-label='Select all libraries'
                              />
                            </TableHead>
                            <TableHead>名称</TableHead>
                            <TableHead className='hidden md:table-cell'>
                              路径
                            </TableHead>
                            <TableHead className='hidden md:table-cell'>
                              类型
                            </TableHead>
                            <TableHead className='hidden lg:table-cell'>
                              项目数
                            </TableHead>
                            <TableHead className='hidden lg:table-cell'>
                              上次扫描
                            </TableHead>
                            <TableHead className='hidden xl:table-cell'>
                              健康状况
                            </TableHead>
                            <TableHead className='hidden xl:table-cell'>
                              存储使用
                            </TableHead>
                            <TableHead className='text-right'>操作</TableHead>
                          </TableRow>
                        </TableHeader>
                        <TableBody>
                          {filteredLibraries.map(library => (
                            <TableRow key={library.id}>
                              <TableCell>
                                <Checkbox
                                  checked={selectedLibraries.includes(
                                    library.id
                                  )}
                                  onCheckedChange={checked =>
                                    handleSelectLibrary(library.id, !!checked)
                                  }
                                  aria-label={`Select ${library.name}`}
                                />
                              </TableCell>
                              <TableCell>
                                <div className='flex items-center gap-2'>
                                  {library.type === 'movies' ? (
                                    <MonitorPlay className='h-4 w-4 text-primary' />
                                  ) : library.type === 'tv' ? (
                                    <FileVideo className='h-4 w-4 text-primary' />
                                  ) : (
                                    <FolderOpen className='h-4 w-4 text-primary' />
                                  )}
                                  <div>
                                    <div className='font-medium'>
                                      {library.name}
                                    </div>
                                    {library.status === 'error' && (
                                      <Badge
                                        variant='destructive'
                                        className='mt-1'
                                      >
                                        错误
                                      </Badge>
                                    )}
                                  </div>
                                </div>
                              </TableCell>
                              <TableCell className='hidden md:table-cell'>
                                <span className='block max-w-[200px] truncate text-sm text-muted-foreground'>
                                  {library.path}
                                </span>
                              </TableCell>
                              <TableCell className='hidden md:table-cell'>
                                <Badge variant='secondary'>
                                  {library.type === 'movies'
                                    ? '电影'
                                    : library.type === 'tv'
                                      ? '电视剧'
                                      : '混合内容'}
                                </Badge>
                              </TableCell>
                              <TableCell className='hidden lg:table-cell'>
                                {library.itemCount}
                              </TableCell>
                              <TableCell className='hidden lg:table-cell'>
                                <span className='text-sm text-muted-foreground'>
                                  {library.lastScanned}
                                </span>
                              </TableCell>
                              <TableCell className='hidden xl:table-cell'>
                                {library.status === 'active' ? (
                                  <div className='flex items-center gap-2'>
                                    <Progress
                                      value={library.healthScore || 0}
                                      className='h-2 w-16'
                                      style={
                                        {
                                          backgroundColor: 'rgba(0,0,0,0.1)',
                                          '--progress-background':
                                            library.healthScore >= 90
                                              ? 'rgba(34, 197, 94, 0.8)'
                                              : library.healthScore >= 70
                                                ? 'rgba(245, 158, 11, 0.8)'
                                                : 'rgba(239, 68, 68, 0.8)',
                                        } as React.CSSProperties
                                      }
                                    />
                                    <span
                                      className={`text-sm ${library.healthScore >= 90 ? 'text-green-500' : library.healthScore >= 70 ? 'text-amber-500' : 'text-red-500'}`}
                                    >
                                      {library.healthScore || 0}%
                                    </span>
                                  </div>
                                ) : (
                                  <span className='text-sm text-destructive'>
                                    N/A
                                  </span>
                                )}
                              </TableCell>
                              <TableCell className='hidden xl:table-cell'>
                                <span className='text-sm text-muted-foreground'>
                                  {library.storageUsed}
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
                                            library.status === 'error'
                                          }
                                          onClick={() =>
                                            handleScanLibrary(library.id)
                                          }
                                        >
                                          <RefreshCw className='h-4 w-4' />
                                        </Button>
                                      </TooltipTrigger>
                                      <TooltipContent>
                                        <p>扫描媒体库</p>
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
                                            handleEditLibrary(library)
                                          }
                                        >
                                          <Edit className='h-4 w-4' />
                                        </Button>
                                      </TooltipTrigger>
                                      <TooltipContent>
                                        <p>编辑媒体库</p>
                                      </TooltipContent>
                                    </Tooltip>
                                  </TooltipProvider>

                                  <DropdownMenu>
                                    <DropdownMenuTrigger asChild>
                                      <Button variant='ghost' size='icon'>
                                        <MoreHorizontal className='h-4 w-4' />
                                      </Button>
                                    </DropdownMenuTrigger>
                                    <DropdownMenuContent align='end'>
                                      <DropdownMenuLabel>
                                        媒体库操作
                                      </DropdownMenuLabel>
                                      <DropdownMenuSeparator />
                                      <DropdownMenuItem
                                        onClick={() =>
                                          handleScanLibrary(library.id)
                                        }
                                        disabled={
                                          !!activeScan ||
                                          library.status === 'error'
                                        }
                                      >
                                        <RefreshCw className='mr-2 h-4 w-4' />
                                        扫描
                                      </DropdownMenuItem>
                                      <DropdownMenuItem
                                        onClick={() =>
                                          handleEditLibrary(library)
                                        }
                                      >
                                        <Edit className='mr-2 h-4 w-4' />
                                        编辑
                                      </DropdownMenuItem>
                                      <DropdownMenuItem asChild>
                                        <Link
                                          to={`/media?library=${library.id}`}
                                        >
                                          <Search className='mr-2 h-4 w-4' />
                                          浏览内容
                                        </Link>
                                      </DropdownMenuItem>
                                      <DropdownMenuSeparator />
                                      <DropdownMenuItem
                                        className='text-destructive focus:text-destructive'
                                        onClick={() =>
                                          handleDeleteLibrary(library.id)
                                        }
                                      >
                                        <Trash className='mr-2 h-4 w-4' />
                                        删除
                                      </DropdownMenuItem>
                                    </DropdownMenuContent>
                                  </DropdownMenu>
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
                      ? `显示 ${filteredLibraries.length} 个媒体库 (共 ${libraries.length} 个)`
                      : `共 ${libraries.length} 个媒体库`}
                  </div>
                  <Button
                    variant='outline'
                    size='sm'
                    onClick={handleAddLibrary}
                    className='gap-2'
                  >
                    <FolderPlus className='h-4 w-4' />
                    <span>添加媒体库</span>
                  </Button>
                </CardFooter>
              </Card>
            </TabsContent>

            {/* History Tab */}
            <TabsContent value='history' className='mt-0'>
              <Card>
                <CardHeader>
                  <CardTitle className='flex items-center gap-2'>
                    <History className='h-5 w-5' />
                    扫描历史
                  </CardTitle>
                  <CardDescription>
                    查看媒体库扫描的历史记录和结果
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  {isLoading ? (
                    <div className='space-y-4'>
                      {[...Array(3)].map((_, i) => (
                        <Skeleton key={i} className='h-24 w-full' />
                      ))}
                    </div>
                  ) : recentScans.length === 0 ? (
                    <div className='py-8 text-center'>
                      <Clock className='mx-auto mb-2 h-12 w-12 text-muted-foreground' />
                      <h3 className='text-lg font-medium'>没有扫描历史</h3>
                      <p className='text-muted-foreground'>
                        扫描您的媒体库以查看历史记录
                      </p>
                    </div>
                  ) : (
                    <div className='rounded-md border'>
                      <Table>
                        <TableHeader>
                          <TableRow>
                            <TableHead>媒体库</TableHead>
                            <TableHead>开始时间</TableHead>
                            <TableHead className='hidden md:table-cell'>
                              状态
                            </TableHead>
                            <TableHead className='hidden lg:table-cell'>
                              耗时
                            </TableHead>
                            <TableHead className='hidden lg:table-cell'>
                              扫描项目
                            </TableHead>
                            <TableHead className='hidden md:table-cell'>
                              变更
                            </TableHead>
                            <TableHead className='hidden xl:table-cell'>
                              发起方式
                            </TableHead>
                            <TableHead className='text-right'>操作</TableHead>
                          </TableRow>
                        </TableHeader>
                        <TableBody>
                          {recentScans.map(scan => (
                            <TableRow key={scan.id}>
                              <TableCell>
                                <div className='font-medium'>
                                  {scan.libraryName}
                                </div>
                              </TableCell>
                              <TableCell>
                                <div className='text-sm'>{scan.startTime}</div>
                              </TableCell>
                              <TableCell className='hidden md:table-cell'>
                                {scan.status === 'completed' ? (
                                  <Badge
                                    variant='outline'
                                    className='bg-green-500/10 text-green-500 hover:bg-green-500/20 hover:text-green-600'
                                  >
                                    完成
                                  </Badge>
                                ) : (
                                  <Badge variant='destructive'>失败</Badge>
                                )}
                              </TableCell>
                              <TableCell className='hidden lg:table-cell'>
                                <span className='text-sm text-muted-foreground'>
                                  {scan.duration || 'N/A'}
                                </span>
                              </TableCell>
                              <TableCell className='hidden lg:table-cell'>
                                <span className='text-sm text-muted-foreground'>
                                  {scan.itemsScanned}
                                </span>
                              </TableCell>
                              <TableCell className='hidden md:table-cell'>
                                {scan.status === 'completed' ? (
                                  <div className='text-sm'>
                                    <span className='text-green-500'>
                                      +{scan.itemsAdded}
                                    </span>{' '}
                                    /
                                    <span className='text-amber-500'>
                                      {' '}
                                      ~{scan.itemsUpdated}
                                    </span>{' '}
                                    /
                                    <span className='text-red-500'>
                                      {' '}
                                      -{scan.itemsRemoved}
                                    </span>
                                  </div>
                                ) : (
                                  <span className='text-sm text-destructive'>
                                    错误
                                  </span>
                                )}
                              </TableCell>
                              <TableCell className='hidden xl:table-cell'>
                                <span className='text-sm text-muted-foreground'>
                                  {scan.initiatedBy}
                                </span>
                              </TableCell>
                              <TableCell className='text-right'>
                                <Button
                                  variant='ghost'
                                  size='sm'
                                  onClick={() => handleViewScanDetails(scan)}
                                >
                                  详情
                                </Button>
                              </TableCell>
                            </TableRow>
                          ))}
                        </TableBody>
                      </Table>
                    </div>
                  )}
                </CardContent>
              </Card>
            </TabsContent>

            {/* Settings Tab */}
            <TabsContent value='settings' className='mt-0'>
              <div className='grid grid-cols-1 gap-6 lg:grid-cols-3'>
                <Card className='lg:col-span-2'>
                  <CardHeader>
                    <CardTitle className='flex items-center gap-2'>
                      <Settings className='h-5 w-5' />
                      扫描设置
                    </CardTitle>
                    <CardDescription>配置媒体库扫描的全局设置</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <div className='grid grid-cols-1 gap-8 md:grid-cols-2'>
                      <div className='space-y-6'>
                        <div className='space-y-2'>
                          <div className='flex items-center justify-between'>
                            <Label htmlFor='auto-scan'>
                              添加媒体库后自动扫描
                            </Label>
                            <Switch id='auto-scan' defaultChecked />
                          </div>
                          <p className='text-sm text-muted-foreground'>
                            添加新媒体库后立即开始扫描
                          </p>
                        </div>

                        <div className='space-y-2'>
                          <div className='flex items-center justify-between'>
                            <Label htmlFor='schedule-scan'>定时扫描</Label>
                            <Switch id='schedule-scan' defaultChecked />
                          </div>
                          <p className='text-sm text-muted-foreground'>
                            按计划自动扫描所有媒体库
                          </p>
                          <div className='pt-2'>
                            <Select defaultValue='daily'>
                              <SelectTrigger>
                                <SelectValue placeholder='选择扫描频率' />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value='hourly'>每小时</SelectItem>
                                <SelectItem value='daily'>每天</SelectItem>
                                <SelectItem value='weekly'>每周</SelectItem>
                                <SelectItem value='custom'>自定义</SelectItem>
                              </SelectContent>
                            </Select>
                          </div>
                        </div>

                        <div className='space-y-2'>
                          <div className='flex items-center justify-between'>
                            <Label htmlFor='scan-on-startup'>启动时扫描</Label>
                            <Switch id='scan-on-startup' />
                          </div>
                          <p className='text-sm text-muted-foreground'>
                            应用启动时自动扫描所有媒体库
                          </p>
                        </div>
                      </div>

                      <div className='space-y-6'>
                        <div className='space-y-2'>
                          <div className='flex items-center justify-between'>
                            <Label htmlFor='deep-scan'>深度扫描</Label>
                            <Switch id='deep-scan' />
                          </div>
                          <p className='text-sm text-muted-foreground'>
                            扫描时重新检查所有文件，而不仅是新文件（较慢但更彻底）
                          </p>
                        </div>

                        <div className='space-y-2'>
                          <div className='flex items-center justify-between'>
                            <Label htmlFor='auto-metadata'>
                              自动获取元数据
                            </Label>
                            <Switch id='auto-metadata' defaultChecked />
                          </div>
                          <p className='text-sm text-muted-foreground'>
                            扫描时自动从在线数据库获取缺失的元数据
                          </p>
                        </div>

                        <div className='space-y-2'>
                          <div className='flex items-center justify-between'>
                            <Label htmlFor='notify-changes'>变更通知</Label>
                            <Switch id='notify-changes' defaultChecked />
                          </div>
                          <p className='text-sm text-muted-foreground'>
                            扫描发现媒体库变更时发送通知
                          </p>
                        </div>
                      </div>
                    </div>
                  </CardContent>
                  <CardFooter>
                    <Button>保存设置</Button>
                  </CardFooter>
                </Card>

                <Card>
                  <CardHeader>
                    <CardTitle className='flex items-center gap-2'>
                      <FolderOpen className='h-5 w-5' />
                      高级选项
                    </CardTitle>
                    <CardDescription>配置媒体库的高级扫描选项</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <Accordion type='single' collapsible className='w-full'>
                      <AccordionItem value='item-1'>
                        <AccordionTrigger>文件类型过滤</AccordionTrigger>
                        <AccordionContent>
                          <div className='space-y-4'>
                            <p className='text-sm text-muted-foreground'>
                              选择扫描时要包含的文件类型
                            </p>
                            <div className='grid grid-cols-2 gap-2'>
                              <div className='flex items-center space-x-2'>
                                <Checkbox id='video-mp4' defaultChecked />
                                <Label htmlFor='video-mp4'>.mp4</Label>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <Checkbox id='video-mkv' defaultChecked />
                                <Label htmlFor='video-mkv'>.mkv</Label>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <Checkbox id='video-avi' defaultChecked />
                                <Label htmlFor='video-avi'>.avi</Label>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <Checkbox id='video-mov' defaultChecked />
                                <Label htmlFor='video-mov'>.mov</Label>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <Checkbox id='video-wmv' defaultChecked />
                                <Label htmlFor='video-wmv'>.wmv</Label>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <Checkbox id='video-m4v' defaultChecked />
                                <Label htmlFor='video-m4v'>.m4v</Label>
                              </div>
                            </div>
                          </div>
                        </AccordionContent>
                      </AccordionItem>
                      <AccordionItem value='item-2'>
                        <AccordionTrigger>排除路径</AccordionTrigger>
                        <AccordionContent>
                          <div className='space-y-4'>
                            <p className='text-sm text-muted-foreground'>
                              设置扫描时要排除的文件夹路径
                            </p>
                            <div className='space-y-2'>
                              <div className='flex items-center space-x-2'>
                                <Input
                                  placeholder='/path/to/exclude'
                                  className='flex-1'
                                />
                                <Button variant='outline' size='icon'>
                                  <Trash className='h-4 w-4' />
                                </Button>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <Input
                                  placeholder='/another/path'
                                  className='flex-1'
                                />
                                <Button variant='outline' size='icon'>
                                  <Trash className='h-4 w-4' />
                                </Button>
                              </div>
                              <Button
                                variant='outline'
                                size='sm'
                                className='mt-2 w-full'
                              >
                                添加排除路径
                              </Button>
                            </div>
                          </div>
                        </AccordionContent>
                      </AccordionItem>
                      <AccordionItem value='item-3'>
                        <AccordionTrigger>扫描优先级</AccordionTrigger>
                        <AccordionContent>
                          <div className='space-y-4'>
                            <p className='text-sm text-muted-foreground'>
                              设置媒体库扫描的优先级
                            </p>
                            <RadioGroup defaultValue='normal'>
                              <div className='flex items-center space-x-2'>
                                <RadioGroupItem
                                  value='high'
                                  id='priority-high'
                                />
                                <Label htmlFor='priority-high'>高优先级</Label>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <RadioGroupItem
                                  value='normal'
                                  id='priority-normal'
                                />
                                <Label htmlFor='priority-normal'>
                                  正常优先级
                                </Label>
                              </div>
                              <div className='flex items-center space-x-2'>
                                <RadioGroupItem value='low' id='priority-low' />
                                <Label htmlFor='priority-low'>低优先级</Label>
                              </div>
                            </RadioGroup>
                          </div>
                        </AccordionContent>
                      </AccordionItem>
                    </Accordion>
                  </CardContent>
                  <CardFooter>
                    <Button variant='outline' className='w-full'>
                      应用高级设置
                    </Button>
                  </CardFooter>
                </Card>
              </div>
            </TabsContent>
          </Tabs>
        </main>

        {/* Add Library Dialog */}

        {/* Edit Library Dialog */}
        {/* <Dialog open={showEditDialog} onOpenChange={setShowEditDialog}>
          <DialogContent className='sm:max-w-[550px]'>
            <DialogHeader>
              <DialogTitle>编辑媒体库</DialogTitle>
              <DialogDescription>修改媒体库的设置</DialogDescription>
            </DialogHeader>
            <div className='grid gap-4 py-4'>
              <div className='grid gap-2'>
                <Label htmlFor='edit-name'>媒体库名称</Label>
                <Input
                  id='edit-name'
                  value={formData.name}
                  onChange={e =>
                    setFormData({ ...formData, name: e.target.value })
                  }
                />
              </div>
              <div className='grid gap-2'>
                <Label htmlFor='edit-path'>媒体库路径</Label>
                <div className='flex gap-2'>
                  <Input
                    id='edit-path'
                    value={formData.path}
                    onChange={e =>
                      setFormData({ ...formData, path: e.target.value })
                    }
                    className='flex-1'
                  />
                  <Button variant='outline' onClick={handleBrowsePath}>
                    浏览...
                  </Button>
                </div>
              </div>
              <div className='grid grid-cols-1 gap-4 md:grid-cols-2'>
                <div className='grid gap-2'>
                  <Label htmlFor='edit-type'>媒体库类型</Label>
                  <Select
                    value={formData.type}
                    onValueChange={value =>
                      setFormData({ ...formData, type: value })
                    }
                  >
                    <SelectTrigger id='edit-type'>
                      <SelectValue placeholder='选择媒体库类型' />
                    </SelectTrigger>
                    <SelectContent>
                      {libraryTypes.map(type => (
                        <SelectItem key={type.value} value={type.value}>
                          <div className='flex items-center gap-2'>
                            <type.icon className='h-4 w-4' />
                            <span>{type.label}</span>
                          </div>
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div className='flex items-center space-x-2 self-end'>
                  <Switch
                    id='auto-scan-edit'
                    checked={formData.autoScan}
                    onCheckedChange={checked =>
                      setFormData({ ...formData, autoScan: checked })
                    }
                  />
                  <Label htmlFor='auto-scan-edit'>启用自动扫描</Label>
                </div>
              </div>
            </div>
            <DialogFooter>
              <DialogClose asChild>
                <Button variant='outline'>取消</Button>
              </DialogClose>
              <Button
                onClick={() => handleSaveLibrary(false)}
                disabled={!formData.name || !formData.path}
              >
                保存更改
              </Button>
            </DialogFooter>
          </DialogContent>
        </Dialog> */}

        <LibraryManageDialog
          open={showAddDialog}
          dialogHandler={setShowAddDialog}
          onClose={() => setShowAddDialog(false)}
        />

        {/* Scan Details Dialog */}
        <Dialog
          open={showScanDetailsDialog}
          onOpenChange={setShowScanDetailsDialog}
        >
          <DialogContent className='sm:max-w-[600px]'>
            <DialogHeader>
              <DialogTitle>扫描详情</DialogTitle>
              <DialogDescription>查看媒体库扫描的详细信息</DialogDescription>
            </DialogHeader>
            {currentScan && (
              <ScrollArea className='max-h-[60vh]'>
                <div className='space-y-4 py-2'>
                  <div className='flex items-center justify-between'>
                    <h3 className='text-lg font-medium'>
                      {currentScan.libraryName}
                    </h3>
                    {currentScan.status === 'completed' ? (
                      <Badge
                        variant='outline'
                        className='bg-green-500/10 text-green-500 hover:bg-green-500/20 hover:text-green-600'
                      >
                        完成
                      </Badge>
                    ) : (
                      <Badge variant='destructive'>失败</Badge>
                    )}
                  </div>

                  <div className='grid grid-cols-2 gap-4'>
                    <div>
                      <p className='text-sm text-muted-foreground'>开始时间</p>
                      <p className='font-medium'>{currentScan.startTime}</p>
                    </div>
                    <div>
                      <p className='text-sm text-muted-foreground'>结束时间</p>
                      <p className='font-medium'>
                        {currentScan.endTime || 'N/A'}
                      </p>
                    </div>
                    <div>
                      <p className='text-sm text-muted-foreground'>耗时</p>
                      <p className='font-medium'>
                        {currentScan.duration || 'N/A'}
                      </p>
                    </div>
                    <div>
                      <p className='text-sm text-muted-foreground'>发起方式</p>
                      <p className='font-medium'>{currentScan.initiatedBy}</p>
                    </div>
                  </div>

                  <div className='border-t pt-4'>
                    <h4 className='mb-2 font-medium'>扫描结果</h4>
                    <div className='grid grid-cols-2 gap-4 sm:grid-cols-4'>
                      <div className='rounded-lg bg-muted/30 p-3 text-center'>
                        <p className='text-2xl font-bold'>
                          {currentScan.itemsScanned}
                        </p>
                        <p className='text-xs text-muted-foreground'>
                          扫描项目
                        </p>
                      </div>
                      <div className='rounded-lg bg-muted/30 p-3 text-center'>
                        <p className='text-2xl font-bold text-green-500'>
                          {currentScan.itemsAdded}
                        </p>
                        <p className='text-xs text-muted-foreground'>
                          新增项目
                        </p>
                      </div>
                      <div className='rounded-lg bg-muted/30 p-3 text-center'>
                        <p className='text-2xl font-bold text-amber-500'>
                          {currentScan.itemsUpdated}
                        </p>
                        <p className='text-xs text-muted-foreground'>
                          更新项目
                        </p>
                      </div>
                      <div className='rounded-lg bg-muted/30 p-3 text-center'>
                        <p className='text-2xl font-bold text-red-500'>
                          {currentScan.itemsRemoved}
                        </p>
                        <p className='text-xs text-muted-foreground'>
                          移除项目
                        </p>
                      </div>
                    </div>
                  </div>

                  {currentScan.status === 'failed' && (
                    <div className='border-t pt-4'>
                      <h4 className='mb-2 font-medium'>错误信息</h4>
                      <div className='rounded-lg bg-destructive/10 p-3 text-destructive'>
                        <AlertCircle className='mb-1 h-5 w-5' />
                        <p>{currentScan.error}</p>
                      </div>
                    </div>
                  )}

                  {currentScan.status === 'completed' && (
                    <div className='border-t pt-4'>
                      <h4 className='mb-2 font-medium'>扫描日志</h4>
                      <div className='font-mono h-32 overflow-y-auto rounded-lg bg-muted/30 p-3 text-sm'>
                        <p className='text-muted-foreground'>
                          [{currentScan.startTime}] 开始扫描{' '}
                          {currentScan.libraryName}
                        </p>
                        <p className='text-muted-foreground'>
                          [{currentScan.startTime}] 扫描路径:{' '}
                          {
                            libraries.find(
                              lib => lib.id === currentScan.libraryId
                            )?.path
                          }
                        </p>
                        <p className='text-muted-foreground'>
                          [{currentScan.startTime}] 发现{' '}
                          {currentScan.itemsScanned} 个媒体文件
                        </p>
                        {currentScan.itemsAdded > 0 && (
                          <p className='text-green-500'>
                            [{currentScan.endTime}] 新增{' '}
                            {currentScan.itemsAdded} 个媒体文件
                          </p>
                        )}
                        {currentScan.itemsUpdated > 0 && (
                          <p className='text-amber-500'>
                            [{currentScan.endTime}] 更新{' '}
                            {currentScan.itemsUpdated} 个媒体文件
                          </p>
                        )}
                        {currentScan.itemsRemoved > 0 && (
                          <p className='text-red-500'>
                            [{currentScan.endTime}] 移除{' '}
                            {currentScan.itemsRemoved} 个媒体文件
                          </p>
                        )}
                        <p className='text-muted-foreground'>
                          [{currentScan.endTime}] 扫描完成，耗时{' '}
                          {currentScan.duration}
                        </p>
                      </div>
                    </div>
                  )}
                </div>
              </ScrollArea>
            )}
            <DialogFooter>
              <Button
                variant='outline'
                onClick={() => setShowScanDetailsDialog(false)}
              >
                关闭
              </Button>
              {currentScan && currentScan.status === 'failed' && (
                <Button
                  onClick={() => {
                    handleScanLibrary(currentScan.libraryId)
                    setShowScanDetailsDialog(false)
                  }}
                >
                  重新扫描
                </Button>
              )}
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>
    </>
  )
}
