interface HideOnLoadingProps {
  isLoading: boolean
  children: React.ReactNode
}

export const HideOnLoading = ({ isLoading, children }: HideOnLoadingProps) => {
  return <>{isLoading ? null : children}</>
}
