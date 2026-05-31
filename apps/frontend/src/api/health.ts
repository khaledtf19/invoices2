import { safeFetch } from "./utils";

const getHealth = async () => {
  const result = await safeFetch<string>(`/health`);
  return result;
};

export default getHealth;
