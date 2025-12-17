import { writable } from "svelte/store";
import {
  register as apiRegister,
  login as apiLogin,
  getMe,
  type AuthUser,
} from "../api";
import { logger } from "../utils/logger";

export type { AuthUser };

function createAuthStore() {
  const user = writable<AuthUser | null>(null);
  const isLoading = writable(true);

  async function init() {
    const token = localStorage.getItem("auth_token");
    if (!token) {
      logger.debug("no auth token found");
      isLoading.set(false);
      return;
    }

    try {
      logger.debug("validating existing token");
      const me = await getMe();
      user.set(me);
      logger.info("session restored", { userId: me.id });
    } catch {
      logger.warn("token validation failed, clearing token");
      localStorage.removeItem("auth_token");
    } finally {
      isLoading.set(false);
    }
  }

  async function register(phone: string, name: string) {
    const response = await apiRegister(phone, name);
    localStorage.setItem("auth_token", response.user.token);
    user.set(response.user);
  }

  async function login(phone: string) {
    const response = await apiLogin(phone);
    localStorage.setItem("auth_token", response.user.token);
    user.set(response.user);
  }

  function logout() {
    logger.info("user logged out");
    localStorage.removeItem("auth_token");
    user.set(null);
  }

  return {
    user,
    isLoading,
    init,
    register,
    login,
    logout,
  };
}

export const auth = createAuthStore();
