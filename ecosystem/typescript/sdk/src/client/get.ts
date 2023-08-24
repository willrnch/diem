import { diemRequest } from "./core";
import { DiemRequest, DiemResponse } from "./types";

export type GetRequestOptions = Omit<DiemRequest, "body" | "method">;

/**
 * Main function to do a Get request
 *
 * @param options GetRequestOptions
 * @returns
 */
export async function get<Req, Res>(options: GetRequestOptions): Promise<DiemResponse<Req, Res>> {
  const response: DiemResponse<Req, Res> = await diemRequest<Req, Res>({ ...options, method: "GET" });
  return response;
}
