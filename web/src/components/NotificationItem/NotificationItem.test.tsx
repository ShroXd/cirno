import { act } from 'react-dom/test-utils'

import { cleanup, fireEvent, render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { NotificationItem, NotificationItemProps } from './NotificationItem'
import { Variation } from './constants'

describe('NotificationItem', () => {
  const defaultProps: NotificationItemProps = {
    id: '1',
    message: 'Test message',
    onRemove: vi.fn(),
    title: 'Test title',
  }

  afterEach(() => {
    cleanup()
    vi.resetAllMocks()
  })

  it('should render without a11y violations', async () => {
    const { container } = render(<NotificationItem {...defaultProps} />)
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly with default props', () => {
    const { getByText } = render(<NotificationItem {...defaultProps} />)

    expect(getByText('Test title')).toBeVisible()
    expect(getByText('Test message')).toBeVisible()
  })

  it('should render success variation correctly', () => {
    const { getByTestId } = render(
      <NotificationItem {...defaultProps} variation={Variation.Success} />
    )
    expect(getByTestId('success-icon')).toBeVisible()
  })

  it('should render error variation correctly', () => {
    const { getByTestId } = render(
      <NotificationItem {...defaultProps} variation={Variation.Error} />
    )
    expect(getByTestId('error-icon')).toBeVisible()
  })

  it('should call onRemove when close button is clicked', async () => {
    vi.useFakeTimers()

    const mockOnRemove = vi.fn()
    const { getByRole } = render(
      <NotificationItem {...defaultProps} onRemove={mockOnRemove} />
    )
    fireEvent.click(getByRole('button'))

    expect(mockOnRemove).not.toHaveBeenCalled()

    await act(async () => {
      vi.runAllTimers()
    })

    expect(mockOnRemove).toHaveBeenCalledTimes(1)
    expect(mockOnRemove).toHaveBeenCalledWith(defaultProps.id)
  })

  it('should auto-remove after duration', async () => {
    vi.useFakeTimers()

    const { queryByTestId } = render(<NotificationItem {...defaultProps} />)

    await act(async () => {
      vi.runAllTimers()
    })

    expect(queryByTestId('success-icon')).not.toBeInTheDocument()
    expect(defaultProps.onRemove).toHaveBeenCalledTimes(1)
    expect(defaultProps.onRemove).toHaveBeenCalledWith(defaultProps.id)
  })

  it('should cleanup timers on unmount', () => {
    vi.useFakeTimers()
    const { unmount } = render(<NotificationItem {...defaultProps} />)
    unmount()

    expect(vi.getTimerCount()).toBe(0)
  })
})
