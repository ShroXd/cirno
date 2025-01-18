import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { Library } from './Library'
import { renderWithContext } from '~/test/test-utils'

describe('Library', () => {
  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(<Library />)
    expect(await axe(container)).toHaveNoViolations()
  })
})
