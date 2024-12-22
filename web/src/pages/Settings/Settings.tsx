import { Select, Option } from '@material-tailwind/react'
import { useTranslation } from 'react-i18next'

export const Settings = () => {
  const { t, i18n } = useTranslation()

  return (
    <div className='ml-6'>
      <Select
        label={t('common.languageSelector')}
        value={i18n.language}
        onChange={value => i18n.changeLanguage(value)}
      >
        <Option value='en-US'>{t('common.language.en')}</Option>
        <Option value='zh-CN'>{t('common.language.zh-CN')}</Option>
        <Option value='jp'>{t('common.language.jp')}</Option>
      </Select>
    </div>
  )
}
