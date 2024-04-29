import aspida from "@aspida/axios";

import api from "api/$api";
import { api as axiosApi } from "lib/fetcher";

export const deleteTodo = (params: { id: number }) => {
  const client = api(aspida(axiosApi));
  return client.todos._id(params.id).$delete();
};
