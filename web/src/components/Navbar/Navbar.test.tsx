import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { StickyNavbar } from './Navbar'
import { renderWithContext } from '~/test/test-utils'

describe('Navbar', () => {
  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(<StickyNavbar />)
    expect(
      await axe(container, {
        rules: {
          // TODO: Fix this after we have a better design to handle the nested interactive elements
          'nested-interactive': { enabled: false },
        },
      })
    ).toHaveNoViolations()
  })
})
