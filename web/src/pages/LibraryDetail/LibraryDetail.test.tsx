import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { LibraryDetail } from './LibraryDetail'
import { renderWithContext } from '~/test/test-utils'

describe('LibraryDetail', () => {
  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(<LibraryDetail />)
    expect(await axe(container)).toHaveNoViolations()
  })
})
