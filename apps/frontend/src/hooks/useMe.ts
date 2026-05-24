import { useQuery } from "@tanstack/react-query";
import { me, MeResponse, APIError } from "../api/auth";

export const useMe = () => {
  return useQuery<MeResponse, APIError>({
    queryKey: ["me"],
    queryFn: () => me(),
    staleTime: 1000 * 60 * 5,
  });
};

export type { MeResponse };
