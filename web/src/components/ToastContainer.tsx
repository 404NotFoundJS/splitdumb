import React from "react";
import { useToast } from "../contexts/ToastContext";
import "./ToastContainer.css";

const ToastContainer: React.FC = () => {
  const { toasts, removeToast, handleConfirm, handleCancel } = useToast();

  if (toasts.length === 0) {
    return null;
  }

  return (
    <div
      style={{
        position: "fixed",
        top: "80px",
        right: "20px",
        left: window.innerWidth < 768 ? "20px" : "auto",
        zIndex: 9999,
        display: "flex",
        flexDirection: "column",
        gap: "12px",
        maxWidth: "420px",
        pointerEvents: "all",
      }}
    >
      {toasts.map((toast) => {
        return (
          <div
            key={toast.id}
            style={{
              background: "white",
              padding: "16px",
              borderRadius: "12px",
              boxShadow: "0 4px 12px rgba(0,0,0,0.15)",
              minWidth: "320px",
              borderLeft: toast.isConfirm
                ? "4px solid #f59e0b"
                : toast.type === "success"
                  ? "4px solid #10b981"
                  : toast.type === "error"
                    ? "4px solid #ef4444"
                    : "4px solid #3b82f6",
              animation: "slideIn 0.3s ease-out",
            }}
          >
            <div
              style={{
                marginBottom: toast.isConfirm ? "12px" : "0",
                fontSize: "14px",
                lineHeight: "1.5",
                color: "#1e293b",
              }}
            >
              {toast.message}
            </div>
            {toast.isConfirm ? (
              <div style={{ display: "flex", gap: "8px" }}>
                <button
                  style={{
                    flex: 1,
                    padding: "10px",
                    background: "#ef4444",
                    color: "white",
                    border: "none",
                    borderRadius: "8px",
                    cursor: "pointer",
                    fontSize: "14px",
                  }}
                  onClick={() => handleConfirm(toast.id)}
                >
                  Confirm
                </button>
                <button
                  style={{
                    flex: 1,
                    padding: "10px",
                    background: "#6b7280",
                    color: "white",
                    border: "none",
                    borderRadius: "8px",
                    cursor: "pointer",
                    fontSize: "14px",
                  }}
                  onClick={() => handleCancel(toast.id)}
                >
                  Cancel
                </button>
              </div>
            ) : (
              <button
                style={{
                  background: "none",
                  border: "none",
                  fontSize: "24px",
                  cursor: "pointer",
                  padding: "0",
                }}
                onClick={() => removeToast(toast.id)}
                aria-label="Close"
              >
                ×
              </button>
            )}
          </div>
        );
      })}
    </div>
  );
};

export default ToastContainer;
