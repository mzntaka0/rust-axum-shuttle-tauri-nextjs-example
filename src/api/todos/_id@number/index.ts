/* eslint-disable */
import type * as Types from '../../@types'

export type Methods = {
  get: {
    status: 200
    /** Todo found */
    resBody: Types.Todo
  }

  delete: {
    status: 200
    /** todo successfully deleted */
    resBody: Types.Todo
  }

  patch: {
    status: 200
    /** todo successfully updated */
    resBody: Types.Todo
    reqBody: Types.UpdateTodo
  }
}
