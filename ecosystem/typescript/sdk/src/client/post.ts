import { diemRequest } from "./core";
import { DiemRequest, DiemResponse } from "./types";

export type PostRequestOptions = Omit<DiemRequest, "method">;

/**
 * Main function to do a Post request
 *
 * @param options PostRequestOptions
 * @returns
 */
export async function post<Req, Res>(options: PostRequestOptions): Promise<DiemResponse<Req, Res>> {
  const response: DiemResponse<Req, Res> = await diemRequest<Req, Res>({ ...options, method: "POST" });
  return response;
}
