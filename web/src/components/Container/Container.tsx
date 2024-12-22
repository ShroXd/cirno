export const Container = ({
  children,
  className,
}: {
  children: React.ReactNode
  className?: string
}) => <div className={`px-6 ${className}`}>{children}</div>
