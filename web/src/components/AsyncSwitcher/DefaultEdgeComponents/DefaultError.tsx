interface DefaultErrorProps {
  error: Error
}

export const DefaultError = ({ error }: DefaultErrorProps) => (
  <div role='alert' aria-live='assertive'>
    Error: {error.message}
  </div>
)
