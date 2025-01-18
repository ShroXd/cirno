import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { ContentCard } from './ContentCard'

describe('ContentCard', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(<ContentCard imageUrl='' title='' />)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly with imageUrl, title', () => {
    const { container } = render(<ContentCard imageUrl='' title='Test title' />)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })
})
