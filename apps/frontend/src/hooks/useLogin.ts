import { useMutation } from "@tanstack/react-query";
import type { LoginRequest, LoginResponse, APIError } from "../api/auth";
import { login } from "../api/auth";

export const useLogin = () => {
  return useMutation<LoginResponse, APIError, LoginRequest>({
    mutationFn: login,
  });
};

export type { LoginRequest, LoginResponse };
