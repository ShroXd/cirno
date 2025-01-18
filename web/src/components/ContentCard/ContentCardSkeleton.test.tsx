import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { ContentCardSkeleton } from './ContentCardSkeleton'

describe('ContentCard skeleton', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(<ContentCardSkeleton />)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly', () => {
    const { container } = render(<ContentCardSkeleton />)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })

  it('should render correctly with custom className', () => {
    const { container } = render(<ContentCardSkeleton className='mt-4' />)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })
})
