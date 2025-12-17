import axios, { AxiosError } from "axios";

// Use same hostname as the page, with backend port 3000
const getApiUrl = () => {
  if (import.meta.env.VITE_API_URL) {
    return import.meta.env.VITE_API_URL;
  }
  const { protocol, hostname } = window.location;
  return `${protocol}//${hostname}:3000/api`;
};

const API_URL = getApiUrl();

const api = axios.create({
  baseURL: API_URL,
  timeout: 10000,
});

// Response interceptor for consistent error handling
api.interceptors.response.use(
  (response) => response,
  (error: AxiosError<{ error: string }>) => {
    const message =
      error.response?.data?.error || error.message || "An error occurred";
    return Promise.reject(new Error(message));
  },
);

export const getGroup = async () => {
  const response = await api.get("/group");
  return response.data;
};

export const getBalances = async () => {
  const response = await api.get("/balances");
  return response.data;
};

export const getSettlements = async () => {
  const response = await api.get("/settlements");
  return response.data;
};

export const getSimplifiedSettlements = async () => {
  const response = await api.get("/settlements/simplified");
  return response.data;
};

export const createUser = async (name: string) => {
  const response = await api.post("/users", { name });
  return response.data;
};

export const deleteUser = async (id: number) => {
  const response = await api.delete(`/users/${id}`);
  return response.data;
};

export const createExpense = async (
  description: string,
  amount: number,
  payer: string,
  participants: string[],
  category?: string,
  notes?: string,
) => {
  const response = await api.post("/expenses", {
    description,
    amount,
    payer,
    participants,
    category,
    notes,
  });
  return response.data;
};

export const deleteExpense = async (id: number) => {
  const response = await api.delete(`/expenses/${id}`);
  return response.data;
};

export const settle = async (from: string, to: string, amount: number) => {
  const response = await api.post("/settle", { from, to, amount });
  return response.data;
};

export const listGroups = async () => {
  const response = await api.get("/groups");
  return response.data;
};

export const createGroup = async (name: string) => {
  const response = await api.post("/groups", { name });
  return response.data;
};

export const switchGroup = async (groupId: number) => {
  const response = await api.put("/groups/current", {
    group_id: groupId,
  });
  return response.data;
};

export const updateGroup = async (groupId: number, name: string) => {
  const response = await api.put(`/groups/${groupId}`, { name });
  return response.data;
};

export const deleteGroup = async (groupId: number) => {
  const response = await api.delete(`/groups/${groupId}`);
  return response.data;
};

export default api;
