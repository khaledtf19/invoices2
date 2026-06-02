import { useMutation } from "@tanstack/react-query";
import { refresh } from "@/api/auth";
import { type ApiResponse, unwrapResult } from "@/api/utils";
import type { AuthResponse, RefreshRequest } from "@/types/bindings/Auth";

type RefreshResponse = ApiResponse<AuthResponse>;

export const useRefresh = () => {
  return useMutation<RefreshResponse, Error, RefreshRequest>({
    mutationFn: (request: RefreshRequest) => refresh(request).then(unwrapResult),
  });
};

export type { RefreshResponse };
