import React from "react";
import { createUser } from "../services/api";
import { useToast } from "../contexts/ToastContext";
import { useForm } from "../hooks/useForm";

interface UserFormProps {
  onUserAdded: () => void;
}

interface UserFormValues {
  name: string;
}

const UserForm: React.FC<UserFormProps> = ({ onUserAdded }) => {
  const toast = useToast();

  const { values, isSubmitting, handleChange, handleSubmit } =
    useForm<UserFormValues>({
      initialValues: { name: "" },
      validate: (values) => values.name.trim().length > 0,
      onSubmit: async (values) => {
        await createUser(values.name);
        onUserAdded();
        toast.success(`User "${values.name}" added successfully`);
      },
      onError: (error) => {
        toast.error(error.message);
      },
    });

  return (
    <div className="card form-card">
      <div className="card-body">
        <h5 className="card-title">Add New User</h5>
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="userName" className="form-label">
              Name
            </label>
            <input
              id="userName"
              type="text"
              className="form-control"
              placeholder="Enter name"
              value={values.name}
              onChange={(e) => handleChange("name", e.target.value)}
              disabled={isSubmitting}
              required
            />
          </div>
          <button
            type="submit"
            className="btn btn-primary w-100"
            disabled={isSubmitting}
          >
            {isSubmitting ? "Adding..." : "Add User"}
          </button>
        </form>
      </div>
    </div>
  );
};

export default UserForm;
