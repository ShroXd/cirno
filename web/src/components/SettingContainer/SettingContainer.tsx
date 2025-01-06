import { ReactNode } from 'react'

import { Card, CardBody, Typography } from '@material-tailwind/react'

interface SettingContainerProps {
  title: string
  description: string
  children: ReactNode
}

export const SettingContainer = ({
  title,
  description,
  children,
}: SettingContainerProps) => {
  return (
    <>
      <Card className='mt-6'>
        <CardBody>
          <Typography variant='h5'>{title}</Typography>
          <Typography variant='small' className='mb-6 mt-2'>
            {description}
          </Typography>
          <div className='mt-6 p-4'>{children}</div>
        </CardBody>
      </Card>
    </>
  )
}
