import { useTranslation } from 'react-i18next'

import { Typography } from '@material-tailwind/react'

import { useFeatures } from '~/hooks/feature/useFeatures'

export const FeatureManager = () => {
  const { features, toggleFeature } = useFeatures()
  const { t } = useTranslation()

  return (
    <>
      <Typography variant='h4'>
        {t('component.featureManager.title')}
      </Typography>
      <Typography variant='paragraph'>
        {t('component.featureManager.description')}
      </Typography>
      {features.map(feature => (
        <div key={feature.id}>
          <p>{feature.name}</p>
          <p>{feature.description}</p>
          <button onClick={() => toggleFeature(feature.id)}>
            {feature.enabled
              ? t('component.featureManager.disable')
              : t('component.featureManager.enable')}
          </button>
        </div>
      ))}
    </>
  )
}
