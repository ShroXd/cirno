import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { Divider } from './Divider'

describe('Divider', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(<Divider />)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly with horizontal variant', () => {
    const { container } = render(<Divider variant='horizontal' />)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })

  it('should render correctly with vertical variant', () => {
    const { container } = render(<Divider variant='vertical' />)

    expect(container).toBeVisible()
    expect(container.firstChild).toMatchSnapshot()
  })
})
