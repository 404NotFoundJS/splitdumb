import axios, { AxiosError } from "axios";
import type {
  User,
  Group,
  Expense,
  BalanceResponse,
  SettlementsResponse,
} from "./types";
import { logger } from "./utils/logger";

// Use VITE_API_URL if set, otherwise use relative /api path
const getApiUrl = () => {
  if (import.meta.env.VITE_API_URL) {
    return import.meta.env.VITE_API_URL;
  }
  return "/api";
};

const API_URL = getApiUrl();

const api = axios.create({
  baseURL: API_URL,
  timeout: 10000,
});

// Request interceptor to add auth token
api.interceptors.request.use((config) => {
  const token = localStorage.getItem("auth_token");
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Response interceptor for consistent error handling
api.interceptors.response.use(
  (response) => {
    logger.debug("API response", {
      url: response.config.url,
      status: response.status,
    });
    return response;
  },
  (error: AxiosError<{ error: string }>) => {
    const message =
      error.response?.data?.error || error.message || "An error occurred";
    logger.error("API error", {
      url: error.config?.url,
      status: error.response?.status,
      message,
    });
    return Promise.reject(new Error(message));
  }
);

export interface AuthUser {
  id: number;
  phone: string;
  name: string;
  token: string;
  current_group_id: number;
}

export const register = async (
  phone: string,
  name: string
): Promise<{ user: AuthUser }> => {
  logger.info("registering user", { phone });
  const response = await api.post("/auth/register", { phone, name });
  logger.info("user registered", { userId: response.data.user.id });
  return response.data;
};

export const login = async (phone: string): Promise<{ user: AuthUser }> => {
  logger.info("logging in", { phone });
  const response = await api.post("/auth/login", { phone });
  logger.info("user logged in", { userId: response.data.user.id });
  return response.data;
};

export const getMe = async (): Promise<AuthUser> => {
  const response = await api.get("/auth/me");
  return response.data;
};

export const getGroup = async (): Promise<Group> => {
  const response = await api.get("/group");
  return response.data;
};

export const getBalances = async (): Promise<BalanceResponse> => {
  const response = await api.get("/balances");
  return response.data;
};

export const getSettlements = async (): Promise<SettlementsResponse> => {
  const response = await api.get("/settlements");
  return response.data;
};

export const toggleSimplify = async (): Promise<{
  simplify_debts: boolean;
}> => {
  const response = await api.post("/simplify");
  return response.data;
};

export const createUser = async (phone: string): Promise<User> => {
  const response = await api.post("/users", { phone });
  return response.data;
};

export const deleteUser = async (id: number): Promise<{ success: boolean }> => {
  const response = await api.delete(`/users/${id}`);
  return response.data;
};

export const createExpense = async (
  description: string,
  amount: number,
  payer: string,
  participants: string[],
  category?: string,
  notes?: string
): Promise<Expense> => {
  logger.info("creating expense", { description, amount, payer });
  const response = await api.post("/expenses", {
    description,
    amount,
    payer,
    participants,
    category,
    notes,
  });
  logger.info("expense created", { expenseId: response.data.id });
  return response.data;
};

export const updateExpense = async (
  id: number,
  data: {
    description?: string;
    amount?: number;
    payer?: string;
    participants?: string[];
    category?: string;
    notes?: string;
  }
): Promise<Expense> => {
  logger.info("updating expense", { expenseId: id });
  const response = await api.put(`/expenses/${id}`, data);
  logger.info("expense updated", { expenseId: id });
  return response.data;
};

export const deleteExpense = async (
  id: number
): Promise<{ success: boolean }> => {
  logger.info("deleting expense", { expenseId: id });
  const response = await api.delete(`/expenses/${id}`);
  logger.info("expense deleted", { expenseId: id });
  return response.data;
};

export const settle = async (
  from: string,
  to: string,
  amount: number
): Promise<Expense> => {
  logger.info("recording settlement", { from, to, amount });
  const response = await api.post("/settle", { from, to, amount });
  logger.info("settlement recorded", { expenseId: response.data.id });
  return response.data;
};

export const listGroups = async (): Promise<Group[]> => {
  const response = await api.get("/groups");
  return response.data;
};

export const createGroup = async (name: string): Promise<Group> => {
  logger.info("creating group", { name });
  const response = await api.post("/groups", { name });
  logger.info("group created", { groupId: response.data.id });
  return response.data;
};

export const switchGroup = async (
  groupId: number
): Promise<{ success: boolean; current_group_id: number }> => {
  logger.info("switching group", { groupId });
  const response = await api.put("/groups/current", {
    group_id: groupId,
  });
  logger.info("group switched", { groupId });
  return response.data;
};

export const updateGroup = async (
  groupId: number,
  name: string
): Promise<Group> => {
  const response = await api.put(`/groups/${groupId}`, { name });
  return response.data;
};

export const deleteGroup = async (
  groupId: number
): Promise<{ success: boolean; switched_group?: number }> => {
  logger.info("deleting group", { groupId });
  const response = await api.delete(`/groups/${groupId}`);
  logger.info("group deleted", { groupId });
  return response.data;
};

export default api;
