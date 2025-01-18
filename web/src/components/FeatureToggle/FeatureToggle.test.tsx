import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it, vi } from 'vitest'

import { FeatureToggle } from './FeatureToggle'
import { FeatureProvider } from '~/contexts/FeatureContext/FeatureContext'

vi.mock('~/hooks/feature/useFeatureFlag', () => ({
  useFeatureFlag: vi.fn((id: string) => id === 'feature1'),
}))

describe('FeatureToggle', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(<FeatureToggle featureId='feature1' />)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly with passed props', () => {
    const { queryByTestId, container } = render(
      <FeatureProvider>
        <FeatureToggle featureId='feature1'>
          <div data-testid='text-content-1'>Text content 1</div>
        </FeatureToggle>
        <FeatureToggle featureId='feature2'>
          <div data-testid='text-content-2'>Text content 2</div>
        </FeatureToggle>
      </FeatureProvider>
    )

    expect(queryByTestId('text-content-1')).toBeVisible()
    expect(queryByTestId('text-content-2')).toBeNull()

    expect(container).toMatchSnapshot()
  })
})
