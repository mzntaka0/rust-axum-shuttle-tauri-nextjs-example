import type { AspidaClient, BasicHeaders } from 'aspida';
import type { Methods as Methods_1l17dp7 } from './todos';
import type { Methods as Methods_1hrbyhx } from './todos/_id@number';

const api = <T>({ baseURL, fetch }: AspidaClient<T>) => {
  const prefix = (baseURL === undefined ? '' : baseURL).replace(/\/$/, '');
  const PATH0 = '/todos';
  const GET = 'GET';
  const POST = 'POST';
  const DELETE = 'DELETE';
  const PATCH = 'PATCH';

  return {
    todos: {
      _id: (val1: number) => {
        const prefix1 = `${PATH0}/${val1}`;

        return {
          /**
           * @returns Todo found
           */
          get: (option?: { config?: T | undefined } | undefined) =>
            fetch<Methods_1hrbyhx['get']['resBody'], BasicHeaders, Methods_1hrbyhx['get']['status']>(prefix, prefix1, GET, option).json(),
          /**
           * @returns Todo found
           */
          $get: (option?: { config?: T | undefined } | undefined) =>
            fetch<Methods_1hrbyhx['get']['resBody'], BasicHeaders, Methods_1hrbyhx['get']['status']>(prefix, prefix1, GET, option).json().then(r => r.body),
          /**
           * @returns todo successfully deleted
           */
          delete: (option?: { config?: T | undefined } | undefined) =>
            fetch<Methods_1hrbyhx['delete']['resBody'], BasicHeaders, Methods_1hrbyhx['delete']['status']>(prefix, prefix1, DELETE, option).json(),
          /**
           * @returns todo successfully deleted
           */
          $delete: (option?: { config?: T | undefined } | undefined) =>
            fetch<Methods_1hrbyhx['delete']['resBody'], BasicHeaders, Methods_1hrbyhx['delete']['status']>(prefix, prefix1, DELETE, option).json().then(r => r.body),
          /**
           * @returns todo successfully updated
           */
          patch: (option: { body: Methods_1hrbyhx['patch']['reqBody'], config?: T | undefined }) =>
            fetch<Methods_1hrbyhx['patch']['resBody'], BasicHeaders, Methods_1hrbyhx['patch']['status']>(prefix, prefix1, PATCH, option).json(),
          /**
           * @returns todo successfully updated
           */
          $patch: (option: { body: Methods_1hrbyhx['patch']['reqBody'], config?: T | undefined }) =>
            fetch<Methods_1hrbyhx['patch']['resBody'], BasicHeaders, Methods_1hrbyhx['patch']['status']>(prefix, prefix1, PATCH, option).json().then(r => r.body),
          $path: () => `${prefix}${prefix1}`,
        };
      },
      /**
       * @returns Todos found
       */
      get: (option?: { config?: T | undefined } | undefined) =>
        fetch<Methods_1l17dp7['get']['resBody'], BasicHeaders, Methods_1l17dp7['get']['status']>(prefix, PATH0, GET, option).json(),
      /**
       * @returns Todos found
       */
      $get: (option?: { config?: T | undefined } | undefined) =>
        fetch<Methods_1l17dp7['get']['resBody'], BasicHeaders, Methods_1l17dp7['get']['status']>(prefix, PATH0, GET, option).json().then(r => r.body),
      /**
       * @returns Created Todo successfully
       */
      post: (option: { body: Methods_1l17dp7['post']['reqBody'], config?: T | undefined }) =>
        fetch<Methods_1l17dp7['post']['resBody'], BasicHeaders, Methods_1l17dp7['post']['status']>(prefix, PATH0, POST, option).json(),
      /**
       * @returns Created Todo successfully
       */
      $post: (option: { body: Methods_1l17dp7['post']['reqBody'], config?: T | undefined }) =>
        fetch<Methods_1l17dp7['post']['resBody'], BasicHeaders, Methods_1l17dp7['post']['status']>(prefix, PATH0, POST, option).json().then(r => r.body),
      $path: () => `${prefix}${PATH0}`,
    },
  };
};

export type ApiInstance = ReturnType<typeof api>;
export default api;
