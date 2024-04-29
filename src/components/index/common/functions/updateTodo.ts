import aspida from "@aspida/axios";

import api from "api/$api";
import { type UpdateTodo } from "api/@types";
import { api as axiosApi } from "lib/fetcher";

export const updateTodo = (params: UpdateTodo & { id: number }) => {
  const { id, ...body } = params;
  const client = api(aspida(axiosApi));
  return client.todos._id(id).$patch({ body });
};
