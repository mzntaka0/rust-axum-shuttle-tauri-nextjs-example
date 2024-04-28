/* eslint-disable */
import type * as Types from '../@types'

export type Methods = {
  get: {
    status: 200
    /** Todos were found */
    resBody: Types.Todo
  }

  post: {
    status: 201
    /** Created Todo found successfully */
    resBody: Types.Todo
  }
}
