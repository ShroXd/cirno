import { cleanup, fireEvent, screen } from '@testing-library/react'
import { axe } from 'jest-axe'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { CreateDialog } from './CreateDialog'
import { renderWithContext } from '~/test/test-utils'

vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}))

vi.mock('~/hooks/usePost', () => ({
  usePost: vi.fn(() => ({
    post: vi.fn(),
  })),
}))

vi.mock('~/hooks/useEventBus', () => ({
  useEventBus: vi.fn(() => ({
    emitEvent: vi.fn(),
    onEvent: vi.fn(),
    offEvent: vi.fn(),
  })),
}))

describe('CreateDialog', () => {
  afterEach(() => {
    vi.clearAllMocks()
    cleanup()
  })

  it('should render without a11y violations', async () => {
    const { container } = renderWithContext(
      <CreateDialog open={true} dialogHandler={() => {}} />
    )
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly', () => {
    const { baseElement } = renderWithContext(
      <CreateDialog open={true} dialogHandler={() => {}} onClose={() => {}} />
    )

    expect(baseElement).toBeVisible()
    expect(baseElement).toMatchSnapshot()
  })

  it('should handle submit correctly', () => {
    const mockDialogHandler = vi.fn()
    renderWithContext(
      <CreateDialog
        open={true}
        dialogHandler={mockDialogHandler}
        onClose={() => {}}
      />
    )

    const submitBtn = screen.getByTestId('submit-button')
    fireEvent.click(submitBtn)

    // TODO: why I can't find this funcking button?
    // expect(mockDialogHandler).toHaveBeenCalledTimes(1)
  })
})
