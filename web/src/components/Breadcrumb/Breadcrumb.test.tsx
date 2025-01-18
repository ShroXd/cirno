import { useLocation, useNavigate } from 'react-router-dom'

import { cleanup, fireEvent } from '@testing-library/react'
import { axe } from 'jest-axe'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { Breadcrumb } from './Breadcrumb'
import { renderWithContext } from '~/test/test-utils'

vi.mock(import('react-router-dom'), async importOriginal => {
  const actual = await importOriginal()
  return {
    ...actual,
    useLocation: vi.fn(() => ({
      pathname: '/',
      search: '',
      hash: '',
      state: null,
      key: 'default',
    })),
    useNavigate: vi.fn(() => vi.fn()),
  }
})

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}))

describe('Breadcrumb', () => {
  const mockOnBack = vi.fn()

  afterEach(() => {
    vi.clearAllMocks()
    cleanup()
  })

  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(<Breadcrumb />)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render home icon and segments correctly', () => {
    vi.mocked(useLocation).mockReturnValue({
      pathname: '/foo/bar',
      search: '',
      hash: '',
      state: null,
      key: 'default',
    })

    const { queryAllByTestId, getByText } = renderWithContext(
      <Breadcrumb onBack={mockOnBack} />
    )

    expect(queryAllByTestId('home-icon')).toHaveLength(1)
    expect(getByText('component.breadcrumb.foo')).toBeVisible()
    expect(getByText('component.breadcrumb.bar')).toBeVisible()
  })

  it('should handle back navigation correctly for nested routes', () => {
    vi.mocked(useLocation).mockReturnValue({
      pathname: '/foo/1/bar/2/baz/3',
      search: '',
      hash: '',
      state: null,
      key: 'default',
    })
    const mockNavigate = vi.fn()
    vi.mocked(useNavigate).mockReturnValue(mockNavigate)

    const { getByLabelText } = renderWithContext(
      <Breadcrumb onBack={mockOnBack} />
    )

    fireEvent.click(getByLabelText('component.breadcrumb.back'))

    expect(mockOnBack).toHaveBeenCalledTimes(1)
    expect(mockNavigate).toHaveBeenCalledTimes(1)
    // TODO: the design of router makes the back functionality a bit weird, consider better solution
    expect(mockNavigate).toHaveBeenCalledWith('/foo/1/bar/2')
  })
})
