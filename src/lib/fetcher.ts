//import axiosbase, { AxiosRequestConfig } from "axios";
import axiosbase from "axios";

// TODO: add and dispatch tauri url
export const api = axiosbase.create({
  baseURL: process.env.API_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

//api.interceptors.request.use(
//  // @ts-ignore
//  async (config: AxiosRequestConfig) => {
//    const token = process.env.BEARER || "";
//    return {
//      ...config,
//      headers: {
//        ...config.headers,
//        Authorization: `Bearer ${token}`,
//      },
//    };
//  },
//  (error) => Promise.reject(error),
//);

export const fetcher = async (endPoint: string, options = {}) => {
  const result = await api.get(endPoint, options).then((res) => {
    if (!(res.status === 200)) throw new Error(res.statusText);
    return res.data;
  });
  return result;
};
