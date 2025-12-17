import React from "react";
import { createUser } from "../services/api";
import { useToast } from "../contexts/ToastContext";
import { useForm } from "../hooks/useForm";
import { formatPhoneNumber, isValidPhone } from "../utils/phone";

interface UserFormProps {
  onUserAdded: () => void;
}

interface UserFormValues {
  phone: string;
}

const UserForm: React.FC<UserFormProps> = ({ onUserAdded }) => {
  const toast = useToast();

  const { values, isSubmitting, handleChange, handleSubmit } =
    useForm<UserFormValues>({
      initialValues: { phone: "" },
      validate: (values) => isValidPhone(values.phone),
      onSubmit: async (values) => {
        const user = await createUser(values.phone);
        onUserAdded();
        toast.success(`User "${user.name}" added successfully`);
      },
      onError: (error) => {
        toast.error(error.message);
      },
    });

  return (
    <div className="card form-card">
      <div className="card-body">
        <h5 className="card-title">Add Member</h5>
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="userPhone" className="form-label">
              Phone Number
            </label>
            <input
              id="userPhone"
              type="tel"
              className="form-control"
              placeholder="XXX-XXXX-XXXX"
              value={values.phone}
              onChange={(e) =>
                handleChange("phone", formatPhoneNumber(e.target.value))
              }
              disabled={isSubmitting}
              required
            />
          </div>
          <button
            type="submit"
            className="btn btn-primary w-100"
            disabled={isSubmitting || !isValidPhone(values.phone)}
          >
            {isSubmitting ? "Adding..." : "Add Member"}
          </button>
        </form>
      </div>
    </div>
  );
};

export default UserForm;
