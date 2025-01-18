import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { DeleteConfirmationDialog } from './DeleteConfirmationDialog'

describe('DeleteConfirmationDialog', () => {
  it('should render without a11y violations', async () => {
    const { container } = render(
      <DeleteConfirmationDialog
        title='Delete Confirmation'
        description='Are you sure you want to delete this item?'
        open={true}
        handleConfirm={() => {}}
        handleCancel={() => {}}
      />
    )
    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly with passed props', () => {
    const { baseElement } = render(
      <DeleteConfirmationDialog
        title='Delete Confirmation'
        description='Are you sure you want to delete this item?'
        open={true}
        handleConfirm={() => {}}
        handleCancel={() => {}}
      />
    )

    expect(baseElement).toBeVisible()
    expect(baseElement).toMatchSnapshot()
  })
})
