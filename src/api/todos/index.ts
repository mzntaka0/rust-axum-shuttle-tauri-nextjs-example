/* eslint-disable */
import type * as Types from '../@types'

export type Methods = {
  get: {
    status: 200
    /** Todos found */
    resBody: Types.Todo[]
  }

  post: {
    status: 201
    /** Created Todo successfully */
    resBody: Types.Todo
    reqBody: Types.CreateTodo
  }
}
