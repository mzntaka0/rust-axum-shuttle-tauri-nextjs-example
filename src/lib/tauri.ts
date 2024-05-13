import { invoke } from "@tauri-apps/api/core";

export type Fetch = typeof fetch;

export const tauriFetch: Fetch = async (
  input: string | Request | URL,
  init?: RequestInit,
): Promise<Response> => {
  if (input instanceof URL) {
    input = input.toString();
  }

  if (input instanceof Request) {
    throw Error("type Request is not allowed for tauri");
  }
  return call(input, init);
};

export async function call(
  url: string,
  init?: RequestInit,
  on_progress_id?: string,
): Promise<any> {
  const params = init?.body;
  return invoke("call", { url, params, progress: on_progress_id });
}
