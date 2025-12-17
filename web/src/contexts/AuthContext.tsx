import {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  type ReactNode,
} from "react";
import {
  register as apiRegister,
  login as apiLogin,
  getMe,
  type AuthUser,
} from "../services/api";

interface AuthContextType {
  user: AuthUser | null;
  isLoading: boolean;
  register: (phone: string, name: string) => Promise<void>;
  login: (phone: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const checkAuth = async () => {
      const token = localStorage.getItem("auth_token");
      if (!token) {
        setIsLoading(false);
        return;
      }

      try {
        const me = await getMe();
        setUser(me);
      } catch {
        localStorage.removeItem("auth_token");
      } finally {
        setIsLoading(false);
      }
    };

    checkAuth();
  }, []);

  const register = useCallback(async (phone: string, name: string) => {
    const response = await apiRegister(phone, name);
    localStorage.setItem("auth_token", response.user.token);
    setUser(response.user);
  }, []);

  const login = useCallback(async (phone: string) => {
    const response = await apiLogin(phone);
    localStorage.setItem("auth_token", response.user.token);
    setUser(response.user);
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem("auth_token");
    setUser(null);
  }, []);

  return (
    <AuthContext.Provider value={{ user, isLoading, register, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useAuth() {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
