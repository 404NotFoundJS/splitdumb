import { useState, useCallback } from "react";

interface UseFormOptions<T> {
  initialValues: T;
  onSubmit: (values: T) => Promise<void>;
  validate?: (values: T) => boolean;
  onSuccess?: () => void;
  onError?: (error: Error) => void;
}

interface UseFormReturn<T> {
  values: T;
  isSubmitting: boolean;
  handleChange: <K extends keyof T>(field: K, value: T[K]) => void;
  handleSubmit: (e: React.FormEvent) => Promise<void>;
  reset: () => void;
  setValues: React.Dispatch<React.SetStateAction<T>>;
}

export function useForm<T extends object>({
  initialValues,
  onSubmit,
  validate,
  onSuccess,
  onError,
}: UseFormOptions<T>): UseFormReturn<T> {
  const [values, setValues] = useState<T>(initialValues);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleChange = useCallback(
    <K extends keyof T>(field: K, value: T[K]) => {
      setValues((prev) => ({ ...prev, [field]: value }));
    },
    [],
  );

  const reset = useCallback(() => {
    setValues(initialValues);
  }, [initialValues]);

  const handleSubmit = useCallback(
    async (e: React.FormEvent) => {
      e.preventDefault();

      if (isSubmitting) return;
      if (validate && !validate(values)) return;

      setIsSubmitting(true);
      const savedValues = { ...values };
      reset();

      try {
        await onSubmit(savedValues);
        onSuccess?.();
      } catch (err) {
        setValues(savedValues);
        onError?.(err instanceof Error ? err : new Error("An error occurred"));
      } finally {
        setIsSubmitting(false);
      }
    },
    [values, isSubmitting, validate, onSubmit, onSuccess, onError, reset],
  );

  return {
    values,
    isSubmitting,
    handleChange,
    handleSubmit,
    reset,
    setValues,
  };
}
