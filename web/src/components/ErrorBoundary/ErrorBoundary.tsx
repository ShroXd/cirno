import { Component, ReactNode } from 'react'

import { Divider } from '../Divider/Divider'
import { Alert } from '~/components/ui/alert'

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
    if (!this.state.hasError) {
      return this.props.children
    }
    return (
      <>
        <Alert
          className='fixed left-1/2 top-6 z-[9999999] w-5/6 max-w-2xl -translate-x-1/2'
          variant='destructive'
          color='red'
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
              <div className='font-mono max-h-[50vh] overflow-auto whitespace-pre-wrap break-words text-xs'>
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
