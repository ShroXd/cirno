import { Component, ReactNode } from 'react'

import { ExclamationTriangleIcon, XMarkIcon } from '@heroicons/react/24/outline'
import { Alert, Button } from '@material-tailwind/react'

import { Divider } from '../Divider/Divider'

interface ErrorBoundaryState {
  hasError: boolean
  error: Error | null
  errorInfo: React.ErrorInfo | null
  open: boolean
}

export interface ErrorBoundaryProps {
  children: ReactNode
}

class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  private timer: NodeJS.Timeout | null = null

  constructor(props: ErrorBoundaryProps) {
    super(props)
    this.state = { hasError: false, error: null, errorInfo: null, open: false }
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error }
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    this.setState({ error, errorInfo, open: true })
    this.startAutoCloseTimer()

    console.log('ErrorBoundary', error, errorInfo)
    // TODO: publish error to backend
  }

  handleClose = () => {
    this.setState({ open: false })
  }

  startAutoCloseTimer() {
    if (this.timer) {
      clearTimeout(this.timer)
    }

    this.timer = setTimeout(() => {
      this.setState({ open: false })
    }, 10 * 1000) // 10 seconds
  }

  render() {
    return (
      <>
        <Alert
          className='fixed top-6 left-1/2 -translate-x-1/2 w-5/6 max-w-2xl z-[9999999]'
          variant='gradient'
          color='red'
          icon={<ExclamationTriangleIcon className='h-6 w-6' />}
          open={this.state.open}
          action={
            <Button
              variant='text'
              color='white'
              size='sm'
              className='!absolute top-3 right-3'
              onClick={this.handleClose}
            >
              <XMarkIcon className='h-4 w-4' />
            </Button>
          }
        >
          Sorry, something went wrong please try again.
          {process.env.NODE_ENV === 'development' && (
            <div className='mt-2 text-white/90'>
              <Divider />
              <div>
                The following error information is only visible in development
                environment
              </div>
              <div className='mb-1'>
                <span className='font-medium'>Error:</span>{' '}
                <span className='font-mono'>{this.state.error?.message}</span>
              </div>
              <div className='font-mono text-xs whitespace-pre-wrap break-words overflow-auto max-h-[50vh]'>
                {this.state.errorInfo?.componentStack}
              </div>
            </div>
          )}
        </Alert>
        {this.props.children}
      </>
    )
  }
}

export default ErrorBoundary
