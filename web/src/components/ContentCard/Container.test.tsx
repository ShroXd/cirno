import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { ContentCardContainer } from './Container'

describe('ContentCard Container', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(
      <ContentCardContainer>Child</ContentCardContainer>
    )
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly with children', () => {
    const { container } = render(
      <ContentCardContainer>Child</ContentCardContainer>
    )

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })

  it('should render correctly with custom className', () => {
    const { container } = render(
      <ContentCardContainer className='mt-4'>Child</ContentCardContainer>
    )

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })
})
