export const Divider = ({
  variant = 'horizontal',
  className = '',
}: {
  variant?: 'horizontal' | 'vertical'
  className?: string
}) => {
  if (variant === 'horizontal') {
    return <div className={`h-px bg-gray-300 ${className}`} />
  } else {
    return <div className={`h-full w-px bg-gray-300 ${className}`} />
  }
}
