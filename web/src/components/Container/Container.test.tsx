import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { Container } from './Container'

describe('Container', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(<Container>Child</Container>)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly with children', () => {
    const { container } = render(<Container>Child</Container>)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })

  it('should render correctly with custom className', () => {
    const { container } = render(<Container className='mt-4'>Child</Container>)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })
})
