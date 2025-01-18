import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { Home } from './Home'
import { renderWithContext } from '~/test/test-utils'

describe('Home', () => {
  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(<Home />)
    expect(
      await axe(container, {
        rules: {
          // TODO: caused by StickyNavbar
          'nested-interactive': { enabled: false },
        },
      })
    ).toHaveNoViolations()
  })
})
