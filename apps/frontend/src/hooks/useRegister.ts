import { register } from "@/api/auth";
import { type ApiResponse, unwrapResult } from "@/api/utils";
import type { AuthResponse, RegisterRequest } from "@/types/bindings/Auth";
import { useMutation } from "@tanstack/react-query";

export const useRegister = () => {
  return useMutation<ApiResponse<AuthResponse>, Error, RegisterRequest>({
    mutationFn: (request: RegisterRequest) => register(request).then(unwrapResult),
  });
};
