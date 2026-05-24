import { useMutation } from "@tanstack/react-query";
import { refresh, RefreshResponse, APIError } from "../api/auth";

export const useRefresh = () => {
  return useMutation<RefreshResponse, APIError>(() => refresh());
};

export type { RefreshResponse };
