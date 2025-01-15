import { cleanup, render } from '@testing-library/react'
import { afterEach, describe, expect, it } from 'vitest'

import { AsyncSwitcher } from './AsyncSwitcher'

describe('AsyncSwitcher', () => {
  afterEach(() => {
    cleanup()
  })

  it('should render loading component when loading is true', () => {
    const { getByRole } = render(
      <AsyncSwitcher loading={true} children={undefined} />
    )
    expect(getByRole('status')).toHaveTextContent('Loading...')
  })

  it('should render error component when error is true', () => {
    const { getByRole } = render(
      <AsyncSwitcher error={new Error('test error')} children={undefined} />
    )
    expect(getByRole('alert')).toHaveTextContent('Error: test error')
  })

  it('should render empty component when isEmpty is true', () => {
    const { getByRole } = render(
      <AsyncSwitcher
        isEmpty={() => true}
        children={undefined}
        data={undefined}
      />
    )
    expect(getByRole('status')).toHaveTextContent('Empty')
  })

  it('should render children when loading, error, and isEmpty are false', () => {
    const { queryByRole, getByText } = render(
      <AsyncSwitcher
        loading={false}
        error={null}
        isEmpty={() => false}
        children={<div>Test children</div>}
      />
    )
    expect(queryByRole('status')).not.toBeInTheDocument()
    expect(queryByRole('alert')).not.toBeInTheDocument()
    expect(queryByRole('status')).not.toBeInTheDocument()
    expect(getByText('Test children')).toBeVisible()
  })
})
