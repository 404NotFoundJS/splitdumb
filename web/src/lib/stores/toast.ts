import { writable } from "svelte/store";

export type ToastType = "success" | "error" | "info" | "warning";

export interface Toast {
  id: string;
  message: string;
  type: ToastType;
  duration?: number;
  isConfirm?: boolean;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  const callbacks = new Map<string, () => void>();

  function removeToast(id: string) {
    update((toasts) => toasts.filter((t) => t.id !== id));
    callbacks.delete(id);
  }

  function addToast(message: string, type: ToastType = "info", duration = 3000) {
    const id = Math.random().toString(36).substring(2, 9);
    const toast: Toast = { id, message, type, duration };

    update((toasts) => [...toasts, toast]);

    if (duration > 0) {
      setTimeout(() => removeToast(id), duration);
    }

    return id;
  }

  function success(message: string, duration?: number) {
    return addToast(message, "success", duration);
  }

  function error(message: string, duration?: number) {
    return addToast(message, "error", duration ?? 5000);
  }

  function info(message: string, duration?: number) {
    return addToast(message, "info", duration);
  }

  function warning(message: string, duration?: number) {
    return addToast(message, "warning", duration);
  }

  function confirm(message: string, onConfirm: () => void) {
    const id = Math.random().toString(36).substring(2, 9);
    callbacks.set(id, onConfirm);

    const toast: Toast = {
      id,
      message,
      type: "warning",
      duration: 0,
      isConfirm: true,
    };

    update((toasts) => [...toasts, toast]);
    return id;
  }

  function handleConfirm(id: string) {
    const callback = callbacks.get(id);
    if (callback) {
      callback();
    }
    removeToast(id);
  }

  function handleCancel(id: string) {
    removeToast(id);
  }

  return {
    subscribe,
    addToast,
    removeToast,
    success,
    error,
    info,
    warning,
    confirm,
    handleConfirm,
    handleCancel,
  };
}

export const toast = createToastStore();
