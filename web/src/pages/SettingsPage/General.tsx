import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Laptop, Moon, Sun } from 'lucide-react'

import { Button } from '../../components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '../../components/ui/card'
import { Label } from '../../components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../../components/ui/select'
import { Separator } from '../../components/ui/separator'
import { Switch } from '../../components/ui/switch'
import { languages } from './constants'
import { MatrialFadeIn } from '~/components/TransitionContainer/MatrialFadeInContainer'
import i18n from '~/i18n'

enum Theme {
  Light = 'light',
  Dark = 'dark',
  System = 'system',
}

export default function General() {
  const [theme, setTheme] = useState(Theme.System)
  const [autoplay, setAutoplay] = useState(true)
  const [notifications, setNotifications] = useState(true)

  const { t } = useTranslation()

  const handleLanguageChange = (value: string) => {
    console.log('value', value)
    i18n.changeLanguage(value)
  }

  const handleThemeChange = (value: Theme) => {
    console.log('value', value)

    switch (value) {
      case Theme.Light:
        setTheme(Theme.Light)
        document.documentElement.classList.remove('dark')
        break
      case Theme.Dark:
        setTheme(Theme.Dark)
        document.documentElement.classList.add('dark')
        break
      case Theme.System:
        setTheme(Theme.System)
        document.documentElement.classList.remove('dark')
        break
      default:
        setTheme(Theme.System)
        break
    }
  }

  return (
    <>
      <MatrialFadeIn delay={0.1}>
        <Card>
          <CardHeader>
            <CardTitle>{t('page.settings.general.title')}</CardTitle>
            <CardDescription>
              {t('page.settings.general.description')}
            </CardDescription>
          </CardHeader>

          {/* Theme */}
          <CardContent className='space-y-4'>
            <div className='space-y-2'>
              <Label>{t('page.settings.general.theme.title')}</Label>
              <div className='flex flex-wrap gap-2'>
                <Button
                  variant={theme === Theme.Light ? 'default' : 'outline'}
                  size='sm'
                  className='gap-2'
                  onClick={() => handleThemeChange(Theme.Light)}
                >
                  <Sun className='h-4 w-4' />{' '}
                  {t('page.settings.general.theme.light')}
                </Button>
                <Button
                  variant={theme === Theme.Dark ? 'default' : 'outline'}
                  size='sm'
                  className='gap-2'
                  onClick={() => handleThemeChange(Theme.Dark)}
                >
                  <Moon className='h-4 w-4' />{' '}
                  {t('page.settings.general.theme.dark')}
                </Button>
                <Button
                  variant={theme === Theme.System ? 'default' : 'outline'}
                  size='sm'
                  className='gap-2'
                  onClick={() => handleThemeChange(Theme.System)}
                >
                  <Laptop className='h-4 w-4' />{' '}
                  {t('page.settings.general.theme.system')}
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </MatrialFadeIn>

      <MatrialFadeIn delay={0.2}>
        <Card>
          <CardHeader>
            <CardTitle>{t('page.settings.language.title')}</CardTitle>
            <CardDescription>
              {t('page.settings.language.description')}
            </CardDescription>
          </CardHeader>
          <CardContent className='space-y-4'>
            <div className='space-y-2'>
              <Label>{t('page.settings.language.interface')}</Label>
              <Select
                value={i18n.language}
                onValueChange={handleLanguageChange}
              >
                <SelectTrigger className='w-full'>
                  <SelectValue placeholder='Select language' />
                </SelectTrigger>
                <SelectContent>
                  {languages.map(lang => (
                    <SelectItem key={lang.code} value={lang.code}>
                      <span className='flex items-center gap-2'>
                        <span>{lang.flag}</span>
                        <span>{t(lang.name)}</span>
                      </span>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </CardContent>
        </Card>
      </MatrialFadeIn>

      <MatrialFadeIn delay={0.3}>
        <Card>
          <CardHeader>
            <CardTitle>{t('page.settings.behavior.title')}</CardTitle>
            <CardDescription>
              {t('page.settings.behavior.description')}
            </CardDescription>
          </CardHeader>
          <CardContent className='space-y-4'>
            <div className='flex items-center justify-between'>
              <div className='space-y-0.5'>
                <Label htmlFor='autoplay'>
                  {t('page.settings.behavior.autoplay')}
                </Label>
                <p className='text-sm text-muted-foreground'>
                  {t('page.settings.behavior.autoplay_description')}
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
                <Label htmlFor='notifications'>
                  {t('page.settings.behavior.notifications')}
                </Label>
                <p className='text-sm text-muted-foreground'>
                  {t('page.settings.behavior.notifications_description')}
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
      </MatrialFadeIn>
    </>
  )
}
