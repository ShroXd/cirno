import { useTranslation } from 'react-i18next'

import { Switch, Typography } from '@material-tailwind/react'

import { useFeatures } from '~/hooks/feature/useFeatures'

export const FeatureManager = () => {
  const { features, toggleFeature } = useFeatures()
  const { t } = useTranslation()

  return (
    <>
      {features.map(feature => (
        <div className='flex items-center justify-between' key={feature.id}>
          <Switch
            ripple={false}
            label={
              <div>
                <Typography color='blue-gray' className='font-medium'>
                  {t(feature.name)}
                </Typography>
                <Typography
                  variant='small'
                  color='gray'
                  className='font-normal'
                >
                  {t(feature.description)}
                </Typography>
              </div>
            }
            onChange={() => toggleFeature(feature.id)}
            checked={feature.enabled}
          />
        </div>
      ))}
    </>
  )
}
