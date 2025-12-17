import { useState } from "react";
import { useAuth } from "../contexts/AuthContext";
import { useToast } from "../contexts/ToastContext";
import { formatPhoneNumber, isValidPhone } from "../utils/phone";

type Mode = "signin" | "signup";

export default function LoginPage() {
  const { register, login } = useAuth();
  const toast = useToast();
  const [mode, setMode] = useState<Mode>("signin");
  const [phone, setPhone] = useState("");
  const [name, setName] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const validPhone = isValidPhone(phone);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validPhone) return;
    if (mode === "signup" && !name.trim()) return;

    setIsSubmitting(true);
    try {
      if (mode === "signup") {
        await register(phone.trim(), name.trim());
      } else {
        await login(phone.trim());
      }
    } catch (error) {
      toast.error(
        error instanceof Error
          ? error.message
          : mode === "signup"
            ? "Registration failed. Please try again."
            : "Sign in failed. Please try again.",
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  const switchMode = () => {
    setMode(mode === "signin" ? "signup" : "signin");
    setName("");
  };

  return (
    <div className="welcome-page">
      <div className="welcome-card">
        <h1 className="welcome-title">Splitdumb</h1>
        <p className="welcome-subtitle">
          Split expenses with friends, the simple way.
        </p>
        <form onSubmit={handleSubmit} className="welcome-form">
          <input
            type="tel"
            className="form-control welcome-input"
            placeholder="Phone number (XXX-XXXX-XXXX)"
            value={phone}
            onChange={(e) => setPhone(formatPhoneNumber(e.target.value))}
            autoFocus
            disabled={isSubmitting}
          />
          {mode === "signup" && (
            <input
              type="text"
              className="form-control welcome-input"
              placeholder="Your name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              disabled={isSubmitting}
            />
          )}
          <button
            type="submit"
            className="btn btn-primary welcome-button"
            disabled={
              isSubmitting || !validPhone || (mode === "signup" && !name.trim())
            }
          >
            {isSubmitting
              ? mode === "signup"
                ? "Creating account..."
                : "Signing in..."
              : mode === "signup"
                ? "Sign Up"
                : "Sign In"}
          </button>
        </form>
        <p className="auth-switch">
          {mode === "signin" ? (
            <>
              Don't have an account?{" "}
              <button
                type="button"
                className="link-button"
                onClick={switchMode}
              >
                Sign Up
              </button>
            </>
          ) : (
            <>
              Already have an account?{" "}
              <button
                type="button"
                className="link-button"
                onClick={switchMode}
              >
                Sign In
              </button>
            </>
          )}
        </p>
      </div>
    </div>
  );
}
