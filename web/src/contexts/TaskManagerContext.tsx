import { FC, ReactNode, createContext, useState } from 'react'

type TaskStatus = 'pending' | 'completed' | 'failed'

interface Task {
  id: number
  status: TaskStatus
}

interface TaskManagerContextProps {
  tasks: Task[]
  startTask: (apiCall: () => Promise<number>) => Promise<Task>
}

export const TaskManagerContext = createContext<
  TaskManagerContextProps | undefined
>(undefined)

export const TaskManagerProvider: FC<{ children: ReactNode }> = ({
  children,
}) => {
  const [tasks, setTasks] = useState<Task[]>([])
  // const { listenForMessages } = useEventBus()

  const startTask = async (apiCall: () => Promise<number>): Promise<Task> => {
    // TODO: consider if we want to get the id generated by the api call or use the uuid generated by the client
    const taskId = await apiCall()
    const newTask: Task = { id: taskId, status: 'pending' }
    setTasks(prevTasks => [...prevTasks, newTask])

    // listenForMessages('async_task', (payload: unknown) => {
    //   // TODO: type guard and process the payload
    //   console.log('payload', payload)
    // })

    return newTask
  }

  // Get notification from the event bus

  return (
    <TaskManagerContext.Provider value={{ tasks, startTask }}>
      {children}
    </TaskManagerContext.Provider>
  )
}
