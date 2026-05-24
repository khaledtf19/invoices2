import { type RegisterRequest, type LoginRequest, type RefreshRequest, type AuthResponse } from "@/types/bindings/Auth";
import { safeFetch } from "./utils";

export const register = async (request: RegisterRequest) => {
  return safeFetch<AuthResponse>("/api/auth/register", {
    method: "POST",
    body: request as any,
  });
};

export const login = async (request: LoginRequest) => {};

export const refresh = async (request: RefreshRequest) => {};
