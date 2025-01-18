import { ThemeProvider } from '@material-tailwind/react'
import { render } from '@testing-library/react'
import { axe } from 'jest-axe'
import { describe, expect, it } from 'vitest'

import { BaseDialog } from './BaseDialog'

describe('BaseDialog', async () => {
  it('should render without a11y violations', async () => {
    const { container } = render(
      <BaseDialog
        title='Test'
        description='Test'
        submitButtonText='Test'
        open={true}
        onClose={() => {}}
        onSubmit={() => Promise.resolve()}
        dialogHandler={() => {}}
      />
    )

    expect(await axe(container)).toHaveNoViolations()
  })

  it('should render correctly', () => {
    const { baseElement } = render(
      <ThemeProvider>
        <BaseDialog
          title='Test title'
          description='Test description'
          submitButtonText='Test submit button text'
          open={true}
          onClose={() => {}}
          onSubmit={() => Promise.resolve()}
          dialogHandler={() => {}}
        />
      </ThemeProvider>
    )

    expect(baseElement).toMatchSnapshot()
  })
})
