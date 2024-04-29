import aspida from "@aspida/axios";

import api from "api/$api";
import { type CreateTodo } from "api/@types";
import { api as axiosApi } from "lib/fetcher";

export const createTodo = (params: CreateTodo) => {
  const client = api(aspida(axiosApi));
  return client.todos.$post({ body: params });
};
