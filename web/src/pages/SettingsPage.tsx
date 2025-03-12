import { useState } from 'react'

import {
  HelpCircle,
  History,
  Info,
  Laptop,
  LogOut,
  Moon,
  RefreshCw,
  Settings,
  Shield,
  Sun,
  Trash2,
  Volume2,
} from 'lucide-react'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '../components/ui/alert-dialog'
import { Badge } from '../components/ui/badge'
import { Button } from '../components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '../components/ui/card'
import { Input } from '../components/ui/input'
import { Label } from '../components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../components/ui/select'
import { Separator } from '../components/ui/separator'
import { Slider } from '../components/ui/slider'
import { Switch } from '../components/ui/switch'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../components/ui/tabs'

// Available languages
const languages = [
  { code: 'en', name: 'English', flag: '🇺🇸' },
  { code: 'es', name: 'Español', flag: '🇪🇸' },
  { code: 'fr', name: 'Français', flag: '🇫🇷' },
  { code: 'de', name: 'Deutsch', flag: '🇩🇪' },
  { code: 'it', name: 'Italiano', flag: '🇮🇹' },
  { code: 'pt', name: 'Português', flag: '🇵🇹' },
  { code: 'ja', name: '日本語', flag: '🇯🇵' },
  { code: 'ko', name: '한국어', flag: '🇰🇷' },
  { code: 'zh', name: '中文', flag: '🇨🇳' },
]

// Video quality options
const videoQualities = [
  { value: 'auto', label: 'Auto (Recommended)' },
  { value: 'low', label: 'Low (480p)' },
  { value: 'medium', label: 'Medium (720p)' },
  { value: 'high', label: 'High (1080p)' },
  { value: 'ultra', label: 'Ultra HD (4K)' },
]

// Subtitle options
const subtitleOptions = [
  { value: 'off', label: 'Off' },
  { value: 'on', label: 'On when available' },
  { value: 'forced', label: 'Forced only' },
]

export default function SettingsPage() {
  // General settings
  const [theme, setTheme] = useState('system')
  const [language, setLanguage] = useState('en')
  const [autoplay, setAutoplay] = useState(true)
  const [notifications, setNotifications] = useState(true)

  // Playback settings
  const [defaultVideoQuality, setDefaultVideoQuality] = useState('auto')
  const [subtitles, setSubtitles] = useState('on')
  const [volume, setVolume] = useState([80])
  const [playbackSpeed, setPlaybackSpeed] = useState('1.0')

  // Download settings
  const [downloadQuality, setDownloadQuality] = useState('medium')
  const [wifiOnly, setWifiOnly] = useState(true)
  const [autoDelete, setAutoDelete] = useState(false)

  // Privacy settings
  const [saveWatchHistory, setSaveWatchHistory] = useState(true)
  const [collectAnalytics, setCollectAnalytics] = useState(true)
  const [showRecommendations, setShowRecommendations] = useState(true)

  // Account settings
  const [email, setEmail] = useState('user@example.com')
  const [name, setName] = useState('John Doe')

  // Reset functions
  const resetAllSettings = () => {
    // Reset all settings to defaults
    setTheme('system')
    setLanguage('en')
    setAutoplay(true)
    setNotifications(true)
    setDefaultVideoQuality('auto')
    setSubtitles('on')
    setVolume([80])
    setPlaybackSpeed('1.0')
    setDownloadQuality('medium')
    setWifiOnly(true)
    setAutoDelete(false)
    setSaveWatchHistory(true)
    setCollectAnalytics(true)
    setShowRecommendations(true)
  }

  const clearWatchHistory = () => {
    // In a real app, this would call an API to clear watch history
    console.log('Watch history cleared')
  }

  const clearDownloads = () => {
    // In a real app, this would call an API to clear downloads
    console.log('Downloads cleared')
  }

  return (
    <div className='container mx-auto px-4 py-6 md:px-6'>
      <div className='mb-6 flex items-center'>
        <Settings className='mr-2 h-6 w-6' />
        <h1 className='text-3xl font-bold'>Settings</h1>
      </div>

      <Tabs defaultValue='general' className='space-y-6'>
        <TabsList className='grid grid-cols-2 gap-2 md:grid-cols-6'>
          <TabsTrigger value='general'>General</TabsTrigger>
          <TabsTrigger value='playback'>Playback</TabsTrigger>
          <TabsTrigger value='downloads'>Downloads</TabsTrigger>
          <TabsTrigger value='privacy'>Privacy</TabsTrigger>
          <TabsTrigger value='account'>Account</TabsTrigger>
          <TabsTrigger value='about'>About</TabsTrigger>
        </TabsList>

        {/* General Settings */}
        <TabsContent value='general' className='animate-fade-in space-y-6'>
          <Card>
            <CardHeader>
              <CardTitle>Appearance</CardTitle>
              <CardDescription>Customize how StreamHub looks</CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <Label>Theme</Label>
                <div className='flex flex-wrap gap-2'>
                  <Button
                    variant={theme === 'light' ? 'default' : 'outline'}
                    size='sm'
                    className='gap-2'
                    onClick={() => setTheme('light')}
                  >
                    <Sun className='h-4 w-4' /> Light
                  </Button>
                  <Button
                    variant={theme === 'dark' ? 'default' : 'outline'}
                    size='sm'
                    className='gap-2'
                    onClick={() => setTheme('dark')}
                  >
                    <Moon className='h-4 w-4' /> Dark
                  </Button>
                  <Button
                    variant={theme === 'system' ? 'default' : 'outline'}
                    size='sm'
                    className='gap-2'
                    onClick={() => setTheme('system')}
                  >
                    <Laptop className='h-4 w-4' /> System
                  </Button>
                </div>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Language</CardTitle>
              <CardDescription>Choose your preferred language</CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <Label>Interface Language</Label>
                <Select value={language} onValueChange={setLanguage}>
                  <SelectTrigger className='w-full'>
                    <SelectValue placeholder='Select language' />
                  </SelectTrigger>
                  <SelectContent>
                    {languages.map(lang => (
                      <SelectItem key={lang.code} value={lang.code}>
                        <span className='flex items-center gap-2'>
                          <span>{lang.flag}</span>
                          <span>{lang.name}</span>
                        </span>
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Behavior</CardTitle>
              <CardDescription>Control how StreamHub behaves</CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='flex items-center justify-between'>
                <div className='space-y-0.5'>
                  <Label htmlFor='autoplay'>Autoplay next episode</Label>
                  <p className='text-sm text-muted-foreground'>
                    Automatically play the next episode when the current one
                    ends
                  </p>
                </div>
                <Switch
                  id='autoplay'
                  checked={autoplay}
                  onCheckedChange={setAutoplay}
                />
              </div>

              <Separator />

              <div className='flex items-center justify-between'>
                <div className='space-y-0.5'>
                  <Label htmlFor='notifications'>Notifications</Label>
                  <p className='text-sm text-muted-foreground'>
                    Receive notifications about new content and updates
                  </p>
                </div>
                <Switch
                  id='notifications'
                  checked={notifications}
                  onCheckedChange={setNotifications}
                />
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* Playback Settings */}
        <TabsContent value='playback' className='animate-fade-in space-y-6'>
          <Card>
            <CardHeader>
              <CardTitle>Video Quality</CardTitle>
              <CardDescription>Manage video playback quality</CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <Label>Default Video Quality</Label>
                <Select
                  value={defaultVideoQuality}
                  onValueChange={setDefaultVideoQuality}
                >
                  <SelectTrigger className='w-full'>
                    <SelectValue placeholder='Select quality' />
                  </SelectTrigger>
                  <SelectContent>
                    {videoQualities.map(quality => (
                      <SelectItem key={quality.value} value={quality.value}>
                        {quality.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <p className='text-sm text-muted-foreground'>
                  Higher quality uses more data. Auto adjusts based on your
                  connection.
                </p>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Audio & Subtitles</CardTitle>
              <CardDescription>
                Manage audio and subtitle preferences
              </CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <Label>Default Volume</Label>
                <div className='flex items-center gap-4'>
                  <Volume2 className='h-5 w-5 text-muted-foreground' />
                  <Slider
                    value={volume}
                    max={100}
                    step={1}
                    onValueChange={setVolume}
                    className='flex-1'
                  />
                  <span className='w-12 text-center'>{volume}%</span>
                </div>
              </div>

              <Separator />

              <div className='space-y-2'>
                <Label>Subtitles</Label>
                <Select value={subtitles} onValueChange={setSubtitles}>
                  <SelectTrigger className='w-full'>
                    <SelectValue placeholder='Select subtitle option' />
                  </SelectTrigger>
                  <SelectContent>
                    {subtitleOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <Separator />

              <div className='space-y-2'>
                <Label>Playback Speed</Label>
                <Select value={playbackSpeed} onValueChange={setPlaybackSpeed}>
                  <SelectTrigger className='w-full'>
                    <SelectValue placeholder='Select playback speed' />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value='0.5'>0.5x</SelectItem>
                    <SelectItem value='0.75'>0.75x</SelectItem>
                    <SelectItem value='1.0'>Normal (1x)</SelectItem>
                    <SelectItem value='1.25'>1.25x</SelectItem>
                    <SelectItem value='1.5'>1.5x</SelectItem>
                    <SelectItem value='2.0'>2x</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* Downloads Settings */}
        <TabsContent value='downloads' className='animate-fade-in space-y-6'>
          <Card>
            <CardHeader>
              <CardTitle>Download Settings</CardTitle>
              <CardDescription>
                Manage how content is downloaded
              </CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <Label>Download Quality</Label>
                <Select
                  value={downloadQuality}
                  onValueChange={setDownloadQuality}
                >
                  <SelectTrigger className='w-full'>
                    <SelectValue placeholder='Select download quality' />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value='low'>Low (Uses less storage)</SelectItem>
                    <SelectItem value='medium'>Medium (Recommended)</SelectItem>
                    <SelectItem value='high'>
                      High (Uses more storage)
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Separator />

              <div className='flex items-center justify-between'>
                <div className='space-y-0.5'>
                  <Label htmlFor='wifi-only'>Download on Wi-Fi only</Label>
                  <p className='text-sm text-muted-foreground'>
                    Only download content when connected to Wi-Fi
                  </p>
                </div>
                <Switch
                  id='wifi-only'
                  checked={wifiOnly}
                  onCheckedChange={setWifiOnly}
                />
              </div>

              <Separator />

              <div className='flex items-center justify-between'>
                <div className='space-y-0.5'>
                  <Label htmlFor='auto-delete'>
                    Auto-delete watched downloads
                  </Label>
                  <p className='text-sm text-muted-foreground'>
                    Automatically delete downloads after you've watched them
                  </p>
                </div>
                <Switch
                  id='auto-delete'
                  checked={autoDelete}
                  onCheckedChange={setAutoDelete}
                />
              </div>
            </CardContent>
            <CardFooter>
              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <Button variant='destructive' className='gap-2'>
                    <Trash2 className='h-4 w-4' /> Clear All Downloads
                  </Button>
                </AlertDialogTrigger>
                <AlertDialogContent>
                  <AlertDialogHeader>
                    <AlertDialogTitle>Clear all downloads?</AlertDialogTitle>
                    <AlertDialogDescription>
                      This will delete all downloaded content from your device.
                      This action cannot be undone.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction onClick={clearDownloads}>
                      Clear Downloads
                    </AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            </CardFooter>
          </Card>
        </TabsContent>

        {/* Privacy Settings */}
        <TabsContent value='privacy' className='animate-fade-in space-y-6'>
          <Card>
            <CardHeader>
              <CardTitle>Privacy Settings</CardTitle>
              <CardDescription>Manage your privacy preferences</CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='flex items-center justify-between'>
                <div className='space-y-0.5'>
                  <Label htmlFor='watch-history'>Save watch history</Label>
                  <p className='text-sm text-muted-foreground'>
                    Keep track of what you've watched and where you left off
                  </p>
                </div>
                <Switch
                  id='watch-history'
                  checked={saveWatchHistory}
                  onCheckedChange={setSaveWatchHistory}
                />
              </div>

              <Separator />

              <div className='flex items-center justify-between'>
                <div className='space-y-0.5'>
                  <Label htmlFor='analytics'>Usage analytics</Label>
                  <p className='text-sm text-muted-foreground'>
                    Help improve StreamHub by sharing anonymous usage data
                  </p>
                </div>
                <Switch
                  id='analytics'
                  checked={collectAnalytics}
                  onCheckedChange={setCollectAnalytics}
                />
              </div>

              <Separator />

              <div className='flex items-center justify-between'>
                <div className='space-y-0.5'>
                  <Label htmlFor='recommendations'>
                    Personalized recommendations
                  </Label>
                  <p className='text-sm text-muted-foreground'>
                    Show content recommendations based on your viewing habits
                  </p>
                </div>
                <Switch
                  id='recommendations'
                  checked={showRecommendations}
                  onCheckedChange={setShowRecommendations}
                />
              </div>
            </CardContent>
            <CardFooter>
              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <Button variant='outline' className='gap-2'>
                    <History className='h-4 w-4' /> Clear Watch History
                  </Button>
                </AlertDialogTrigger>
                <AlertDialogContent>
                  <AlertDialogHeader>
                    <AlertDialogTitle>Clear watch history?</AlertDialogTitle>
                    <AlertDialogDescription>
                      This will remove all your viewing history and reset your
                      progress on all content. This action cannot be undone.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction onClick={clearWatchHistory}>
                      Clear History
                    </AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            </CardFooter>
          </Card>
        </TabsContent>

        {/* Account Settings */}
        <TabsContent value='account' className='animate-fade-in space-y-6'>
          <Card>
            <CardHeader>
              <CardTitle>Profile Information</CardTitle>
              <CardDescription>Manage your account details</CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <Label htmlFor='name'>Name</Label>
                <Input
                  id='name'
                  value={name}
                  onChange={e => setName(e.target.value)}
                />
              </div>

              <div className='space-y-2'>
                <Label htmlFor='email'>Email</Label>
                <Input
                  id='email'
                  type='email'
                  value={email}
                  onChange={e => setEmail(e.target.value)}
                />
              </div>
            </CardContent>
            <CardFooter>
              <Button>Save Changes</Button>
            </CardFooter>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Password</CardTitle>
              <CardDescription>Change your password</CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <Label htmlFor='current-password'>Current Password</Label>
                <Input id='current-password' type='password' />
              </div>

              <div className='space-y-2'>
                <Label htmlFor='new-password'>New Password</Label>
                <Input id='new-password' type='password' />
              </div>

              <div className='space-y-2'>
                <Label htmlFor='confirm-password'>Confirm New Password</Label>
                <Input id='confirm-password' type='password' />
              </div>
            </CardContent>
            <CardFooter>
              <Button>Update Password</Button>
            </CardFooter>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Linked Devices</CardTitle>
              <CardDescription>
                Manage devices connected to your account
              </CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-4'>
                {[
                  {
                    name: 'Chrome on Windows',
                    lastActive: 'Now',
                    current: true,
                  },
                  {
                    name: 'iPhone 13',
                    lastActive: '2 hours ago',
                    current: false,
                  },
                  {
                    name: 'Samsung TV',
                    lastActive: 'Yesterday',
                    current: false,
                  },
                ].map((device, index) => (
                  <div
                    key={index}
                    className='flex items-center justify-between'
                  >
                    <div>
                      <p className='font-medium'>{device.name}</p>
                      <p className='text-sm text-muted-foreground'>
                        Last active: {device.lastActive}
                      </p>
                    </div>
                    <div className='flex items-center gap-2'>
                      {device.current && (
                        <Badge
                          variant='outline'
                          className='bg-green-500/10 text-green-500'
                        >
                          Current Device
                        </Badge>
                      )}
                      {!device.current && (
                        <Button variant='ghost' size='sm'>
                          <LogOut className='h-4 w-4' />
                          <span className='sr-only'>Sign out</span>
                        </Button>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        {/* About & Help */}
        <TabsContent value='about' className='animate-fade-in space-y-6'>
          <Card>
            <CardHeader>
              <CardTitle>About StreamHub</CardTitle>
              <CardDescription>
                Information about the application
              </CardDescription>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-2'>
                <div className='flex justify-between'>
                  <span className='text-muted-foreground'>Version</span>
                  <span>2.4.1</span>
                </div>
                <div className='flex justify-between'>
                  <span className='text-muted-foreground'>Build</span>
                  <span>2024.03.15.1</span>
                </div>
                <div className='flex justify-between'>
                  <span className='text-muted-foreground'>Device ID</span>
                  <span>SH-12345-ABCDE</span>
                </div>
              </div>

              <Separator />

              <div className='space-y-2'>
                <Button
                  variant='outline'
                  className='w-full justify-start gap-2'
                >
                  <Info className='h-4 w-4' /> Terms of Service
                </Button>
                <Button
                  variant='outline'
                  className='w-full justify-start gap-2'
                >
                  <Shield className='h-4 w-4' /> Privacy Policy
                </Button>
                <Button
                  variant='outline'
                  className='w-full justify-start gap-2'
                >
                  <HelpCircle className='h-4 w-4' /> Help Center
                </Button>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Reset Application</CardTitle>
              <CardDescription>
                Reset all settings to default values
              </CardDescription>
            </CardHeader>
            <CardContent>
              <p className='mb-4 text-sm text-muted-foreground'>
                This will reset all your preferences to their default values.
                Your account information and content will not be affected.
              </p>
              <AlertDialog>
                <AlertDialogTrigger asChild>
                  <Button variant='destructive' className='gap-2'>
                    <RefreshCw className='h-4 w-4' /> Reset All Settings
                  </Button>
                </AlertDialogTrigger>
                <AlertDialogContent>
                  <AlertDialogHeader>
                    <AlertDialogTitle>Reset all settings?</AlertDialogTitle>
                    <AlertDialogDescription>
                      This will reset all your preferences to their default
                      values. Your account information and content will not be
                      affected. This action cannot be undone.
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction onClick={resetAllSettings}>
                      Reset Settings
                    </AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  )
}
