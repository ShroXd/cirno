import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import {
  ArrowLeft,
  HelpCircle,
  History,
  Info,
  RefreshCw,
  Shield,
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
} from '../../components/ui/alert-dialog'
import { Button } from '../../components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '../../components/ui/card'
import { Label } from '../../components/ui/label'
import { Separator } from '../../components/ui/separator'
import { Switch } from '../../components/ui/switch'
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '../../components/ui/tabs'
import General from './General'
import { settingsTabs } from './constants'
import { FadeIn } from '~/components/TransitionContainer/FadeIn'
import { MatrialFadeIn } from '~/components/TransitionContainer/MatrialFadeInContainer'
import { SidebarTrigger } from '~/components/ui/sidebar'

export default function SettingsPage() {
  // Privacy settings
  const [saveWatchHistory, setSaveWatchHistory] = useState(true)
  const [collectAnalytics, setCollectAnalytics] = useState(true)
  const [showRecommendations, setShowRecommendations] = useState(true)

  const { t } = useTranslation()

  // Reset functions
  const resetAllSettings = () => {
    setSaveWatchHistory(true)
    setCollectAnalytics(true)
    setShowRecommendations(true)
  }

  const clearWatchHistory = () => {
    // In a real app, this would call an API to clear watch history
    console.log('Watch history cleared')
  }

  return (
    <div className='flex h-full flex-col bg-background md:px-6'>
      <FadeIn>
        <header className='sticky top-0 z-10 w-full border-b border-border/40 bg-background/80 backdrop-blur-md'>
          <div className='flex h-16 items-center px-4'>
            <SidebarTrigger className='mr-4 md:hidden' />
            <Button variant='ghost' size='icon' className='mr-2' asChild>
              <Link to='/'>
                <ArrowLeft className='h-5 w-5' />
                <span className='sr-only'>Back</span>
              </Link>
            </Button>
            <h1 className='text-xl font-bold'>
              {t('page.settings.header.title')}
            </h1>
          </div>
        </header>
      </FadeIn>

      <main className='container mx-auto px-4 py-6'>
        <Tabs defaultValue='general'>
          <MatrialFadeIn>
            <TabsList className='mb-4'>
              {settingsTabs.map(tab => (
                <TabsTrigger key={tab.value} value={tab.value}>
                  {t(tab.i18nLabel)}
                </TabsTrigger>
              ))}
            </TabsList>
          </MatrialFadeIn>

          <TabsContent value='general' className='animate-fade-in space-y-6'>
            <General />
          </TabsContent>

          <MatrialFadeIn delay={0.1}>
            <TabsContent value='privacy' className='animate-fade-in space-y-6'>
              <Card>
                <CardHeader>
                  <CardTitle>Privacy Settings</CardTitle>
                  <CardDescription>
                    Manage your privacy preferences
                  </CardDescription>
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
                        Show content recommendations based on your viewing
                        habits
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
                        <AlertDialogTitle>
                          Clear watch history?
                        </AlertDialogTitle>
                        <AlertDialogDescription>
                          This will remove all your viewing history and reset
                          your progress on all content. This action cannot be
                          undone.
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
          </MatrialFadeIn>

          <MatrialFadeIn delay={0.1}>
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
                    This will reset all your preferences to their default
                    values. Your account information and content will not be
                    affected.
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
                          values. Your account information and content will not
                          be affected. This action cannot be undone.
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
          </MatrialFadeIn>
        </Tabs>
      </main>
    </div>
  )
}
