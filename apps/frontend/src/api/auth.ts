import { type RefreshRequest, type AuthResponse } from "@/types/bindings/Auth";
import type { UserResponse } from "@/types/bindings/UserResponse";
import { safeFetch } from "./utils";

export const refresh = async (request: RefreshRequest) => {
  return safeFetch<AuthResponse>("/auth/refresh", {
    method: "POST",
    body: request as any,
  });
};

export const me = async () => {
  return safeFetch<UserResponse>("/auth/me");
};
