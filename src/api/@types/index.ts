/* eslint-disable */
export type CreateTodo = {
  text: string
}

export type Todo = {
  completed: boolean
  id: number
  text: string
}

export type UpdateTodo = {
  completed?: boolean | null | undefined
  text?: string | null | undefined
}
