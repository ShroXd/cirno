import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { MediaDetail } from './MediaDetail'
import { renderWithContext } from '~/test/test-utils'

describe('MediaDetail', () => {
  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(<MediaDetail />)
    expect(await axe(container)).toHaveNoViolations()
  })
})
