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
import i18n from '~/i18n'

export default function General() {
  const [theme, setTheme] = useState('system')
  const [autoplay, setAutoplay] = useState(true)
  const [notifications, setNotifications] = useState(true)

  const { t } = useTranslation()

  const handleLanguageChange = (value: string) => {
    console.log('value', value)
    i18n.changeLanguage(value)
  }

  return (
    <>
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
                variant={theme === 'light' ? 'default' : 'outline'}
                size='sm'
                className='gap-2'
                onClick={() => setTheme('light')}
              >
                <Sun className='h-4 w-4' />{' '}
                {t('page.settings.general.theme.light')}
              </Button>
              <Button
                variant={theme === 'dark' ? 'default' : 'outline'}
                size='sm'
                className='gap-2'
                onClick={() => setTheme('dark')}
              >
                <Moon className='h-4 w-4' />{' '}
                {t('page.settings.general.theme.dark')}
              </Button>
              <Button
                variant={theme === 'system' ? 'default' : 'outline'}
                size='sm'
                className='gap-2'
                onClick={() => setTheme('system')}
              >
                <Laptop className='h-4 w-4' />{' '}
                {t('page.settings.general.theme.system')}
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Language */}
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
            <Select value={i18n.language} onValueChange={handleLanguageChange}>
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

      {/* Behavior */}
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
    </>
  )
}
