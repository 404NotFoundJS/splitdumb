import React, { useState } from "react";
import { createUser } from "../services/api";
import { useToast } from "../contexts/ToastContext";

interface UserFormProps {
  onUserAdded: () => void;
}

const UserForm: React.FC<UserFormProps> = ({ onUserAdded }) => {
  const toast = useToast();
  const [name, setName] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name || isSubmitting) return;

    setIsSubmitting(true);
    const userName = name;
    setName("");

    try {
      await createUser(userName);
      onUserAdded();
      toast.success(`User "${userName}" added successfully`);
    } catch (err: any) {
      setName(userName);
      toast.error(err.response?.data?.error || "Failed to add user");
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="card form-card">
      <div className="card-body">
        <h5 className="card-title">👤 Add New User</h5>
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
              value={name}
              onChange={(e) => setName(e.target.value)}
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
