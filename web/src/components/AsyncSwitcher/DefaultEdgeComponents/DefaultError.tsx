interface DefaultErrorProps {
  error: Error
}

export const DefaultError = ({ error }: DefaultErrorProps) => (
  <div>Error: {error.message}</div>
)
