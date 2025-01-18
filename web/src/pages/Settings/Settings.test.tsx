import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { Settings } from './Settings'
import { renderWithContext } from '~/test/test-utils'

describe('Settings', () => {
  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(<Settings />)
    expect(await axe(container)).toHaveNoViolations()
  })
})
