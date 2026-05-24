import { API_BASE, safeFetch } from "./utils";

const getHealth = async () => {
  const result = await safeFetch<string>(`${API_BASE}/health`);
  return result;
};

export default getHealth;
