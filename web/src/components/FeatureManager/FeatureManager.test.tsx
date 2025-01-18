import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { FeatureManager } from './FeatureManager'
import { renderWithContext } from '~/test/test-utils'

describe('FeatureManager', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(<FeatureManager />)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly', () => {
    const { container } = renderWithContext(<FeatureManager />)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })
})
