import { Switch } from '~/components/ui/switch'
import { useFeatures } from '~/hooks/feature/useFeatures'

export const FeatureManager = () => {
  const { getAllFeatures, toggleFeature } = useFeatures()

  return (
    <>
      {getAllFeatures().map(feature => (
        <div className='flex items-center justify-between' key={feature.id}>
          <Switch
            id={feature.id}
            checked={feature.enabled}
            onCheckedChange={() => toggleFeature(feature.id)}
          />
        </div>
      ))}
    </>
  )
}
