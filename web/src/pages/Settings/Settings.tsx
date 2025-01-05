import { useTranslation } from 'react-i18next'

import { Option, Select, Typography } from '@material-tailwind/react'

import { Container } from '~/components/Container/Container'
import { FeatureManager } from '~/components/FeatureManager/FeatureManager'
import { SettingContainer } from '~/components/SettingContainer/SettingContainer'

export const Settings = () => {
  const { t, i18n } = useTranslation()

  return (
    <Container>
      <Typography variant='h4'>{t('page.settings.title')}</Typography>
      <SettingContainer
        title={t('page.settings.general.title')}
        description={t('page.settings.general.description')}
      >
        <Select
          label={t('common.language.languageSelector')}
          value={i18n.language}
          onChange={value => i18n.changeLanguage(value)}
        >
          <Option value='en-US'>{t('common.language.en')}</Option>
          <Option value='zh-CN'>{t('common.language.zh-CN')}</Option>
          <Option value='jp'>{t('common.language.jp')}</Option>
        </Select>
      </SettingContainer>

      <SettingContainer
        title={t('page.settings.features.title')}
        description={t('page.settings.features.description')}
      >
        <FeatureManager />
      </SettingContainer>
    </Container>
  )
}
