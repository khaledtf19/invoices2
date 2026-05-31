import { useQuery } from "@tanstack/react-query";
import { me } from "@/api/auth";
import { type ApiResponse, unwrapResult } from "@/api/utils";
import type { UserResponse } from "@/types/bindings/UserResponse";

type MeResponse = ApiResponse<UserResponse>;

export const useMe = () => {
  return useQuery<MeResponse, Error>({
    queryKey: ["me"],
    queryFn: () => me().then(unwrapResult),
    staleTime: 1000 * 60 * 5,
  });
};

export type { MeResponse };
