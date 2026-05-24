import { TaggedError, Result } from "better-result";
import { type ApiError } from "@/types/bindings/ApiError";
export const API_BASE = (import.meta && import.meta.env && (import.meta.env as any).VITE_API_BASE_URL) || ("" as const);

export class NetworkError extends TaggedError("NetworkError")<{
  message: string;
  url: string;
  apiError: ApiError | null;
}>() {}

export class HttpError extends TaggedError("HttpError")<{
  status: number;
  url: string;
  apiError: ApiError | null;
  message: string;
}>() {}

export class ParseError extends TaggedError("ParseError")<{
  message: string;
  apiError: ApiError | null;
}>() {}

export type FetchError = NetworkError | HttpError | ParseError;

export type ApiResponse<T> = {
  status: "success" | "error";
  data: T | null;
  error: { code: ApiError; message: string } | null;
};

export async function safeFetch<T>(url: string, options?: RequestInit): Promise<Result<ApiResponse<T>, FetchError>> {
  // 1. Network
  const responseResult = await Result.tryPromise({
    try: () => fetch(`${API_BASE}${url}`, options),
    catch: (e): NetworkError => new NetworkError({ message: String(e), url, apiError: null }),
  });

  if (Result.isError(responseResult)) return Result.err(responseResult.error); // ✅

  const res = responseResult.value;

  // 2. Parse JSON regardless of status (backend always returns structured body)
  const bodyResult = await Result.tryPromise({
    try: () => res.json() as Promise<ApiResponse<T>>,
    catch: (e): ParseError => new ParseError({ message: String(e), apiError: null }),
  });

  if (Result.isError(bodyResult)) return Result.err(bodyResult.error);

  // 3. HTTP error — return the parsed body so caller gets the ApiError.code too
  if (!res.ok) {
    return Result.err(
      new HttpError({
        status: res.status,
        url,
        apiError: bodyResult.value.error?.code ?? null,
        message: bodyResult.value.error?.message ?? "",
      }),
    );
  }

  if (bodyResult.value.status === "error") {
    return Result.err(
      new HttpError({
        status: 500,
        url,
        apiError: bodyResult.value.error?.code ?? null,
        message: bodyResult.value.error?.message ?? "",
      }),
    );
  }

  return bodyResult;
}
export function unwrapResult<T>(result: Result<T, FetchError>): T {
  if (Result.isError(result)) throw result.error;
  return result.value;
}
