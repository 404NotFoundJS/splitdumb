import React, { useState, useEffect } from "react";
import { createExpense, updateExpense, getGroup } from "../services/api";
import * as Types from "../types";
import { useToast } from "../contexts/ToastContext";

interface ExpenseFormProps {
  onExpenseAdded: () => void;
  refreshKey: number;
  editingExpense?: Types.Expense | null;
  onCancelEdit?: () => void;
}

const CATEGORIES = [
  "Food",
  "Transport",
  "Entertainment",
  "Accommodation",
  "Shopping",
  "Other",
];

const ExpenseForm: React.FC<ExpenseFormProps> = ({
  onExpenseAdded,
  refreshKey,
  editingExpense,
  onCancelEdit,
}) => {
  const toast = useToast();
  const [description, setDescription] = useState("");
  const [amount, setAmount] = useState<number>(0);
  const [payer, setPayer] = useState("");
  const [participants, setParticipants] = useState<string[]>([]);
  const [category, setCategory] = useState("");
  const [notes, setNotes] = useState("");
  const [users, setUsers] = useState<Types.User[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);

  const isEditing = !!editingExpense;

  useEffect(() => {
    fetchUsers();
  }, [refreshKey]);

  useEffect(() => {
    if (editingExpense) {
      setDescription(editingExpense.description);
      setAmount(editingExpense.amount);
      setPayer(editingExpense.payer.name);
      setParticipants(editingExpense.participants.map((p) => p.name));
      setCategory(editingExpense.category || "");
      setNotes(editingExpense.notes || "");
    } else {
      setDescription("");
      setAmount(0);
      setParticipants([]);
      setCategory("");
      setNotes("");
    }
  }, [editingExpense]);

  const fetchUsers = async () => {
    try {
      const group = await getGroup();
      setUsers(group.members);
      if (group.members.length > 0) {
        setPayer(group.members[0].name);
      }
    } catch {
      // Silently fail - errors will be shown elsewhere
    }
  };

  const handleParticipantToggle = (name: string, checked: boolean) => {
    if (checked) {
      setParticipants([...participants, name]);
    } else {
      setParticipants(participants.filter((p) => p !== name));
    }
  };

  const handleSelectAll = () => setParticipants(users.map((u) => u.name));
  const handleClearAll = () => setParticipants([]);

  const resetForm = () => {
    setDescription("");
    setAmount(0);
    setParticipants([]);
    setCategory("");
    setNotes("");
    if (users.length > 0) {
      setPayer(users[0].name);
    }
  };

  const handleCancel = () => {
    resetForm();
    onCancelEdit?.();
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (
      !description ||
      amount <= 0 ||
      !payer ||
      participants.length === 0 ||
      isSubmitting
    ) {
      return;
    }

    setIsSubmitting(true);
    const expenseData = {
      description,
      amount,
      payer,
      participants,
      category,
      notes,
    };

    try {
      if (isEditing && editingExpense) {
        await updateExpense(editingExpense.id, {
          description: expenseData.description,
          amount: expenseData.amount,
          payer: expenseData.payer,
          participants: expenseData.participants,
          category: expenseData.category || undefined,
          notes: expenseData.notes || undefined,
        });
        toast.success(`Expense "${expenseData.description}" updated`);
        onCancelEdit?.();
      } else {
        await createExpense(
          expenseData.description,
          expenseData.amount,
          expenseData.payer,
          expenseData.participants,
          expenseData.category || undefined,
          expenseData.notes || undefined,
        );
        toast.success(`Expense "${expenseData.description}" added`);
        resetForm();
      }
      onExpenseAdded();
    } catch (err) {
      toast.error(
        err instanceof Error
          ? err.message
          : `Failed to ${isEditing ? "update" : "add"} expense`,
      );
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="card form-card">
      <div className="card-body">
        <div className="form-header">
          <h5 className="card-title">
            {isEditing ? "Edit Expense" : "Add New Expense"}
          </h5>
          {isEditing && (
            <button
              type="button"
              className="btn btn-sm btn-secondary"
              onClick={handleCancel}
            >
              Cancel
            </button>
          )}
        </div>
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="expenseDescription" className="form-label">
              Description
            </label>
            <input
              id="expenseDescription"
              type="text"
              className="form-control"
              placeholder="e.g., Dinner, Movie tickets"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              disabled={isSubmitting}
              required
            />
          </div>

          <div className="form-group">
            <label htmlFor="expenseAmount" className="form-label">
              Amount ($)
            </label>
            <input
              id="expenseAmount"
              type="number"
              className="form-control"
              placeholder="0.00"
              step="0.01"
              min="0.01"
              value={amount || ""}
              onChange={(e) => setAmount(parseFloat(e.target.value))}
              disabled={isSubmitting}
              required
            />
          </div>

          <div className="form-group">
            <label htmlFor="expensePayer" className="form-label">
              Who Paid?
            </label>
            <select
              id="expensePayer"
              className="form-select"
              value={payer}
              onChange={(e) => setPayer(e.target.value)}
              required
            >
              {users.map((user) => (
                <option key={user.id} value={user.name}>
                  {user.name}
                </option>
              ))}
            </select>
          </div>

          <div className="form-group">
            <label htmlFor="expenseCategory" className="form-label">
              Category (Optional)
            </label>
            <select
              id="expenseCategory"
              className="form-select"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
            >
              <option value="">No category</option>
              {CATEGORIES.map((cat) => (
                <option key={cat} value={cat}>
                  {cat}
                </option>
              ))}
            </select>
          </div>

          <div className="form-group">
            <label htmlFor="expenseNotes" className="form-label">
              Notes (Optional)
            </label>
            <textarea
              id="expenseNotes"
              className="form-control"
              placeholder="Add any additional details..."
              rows={3}
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
            />
          </div>

          <div className="form-group">
            <div className="form-actions-row">
              <label className="form-label">Split Between</label>
              <div>
                <button
                  type="button"
                  className="btn btn-sm btn-inline"
                  onClick={handleSelectAll}
                >
                  Select All
                </button>
                <button
                  type="button"
                  className="btn btn-sm btn-inline"
                  onClick={handleClearAll}
                >
                  Clear
                </button>
              </div>
            </div>
            <div className="participants-group">
              {users.map((user) => (
                <div className="form-check" key={user.id}>
                  <input
                    id={`participant-${user.id}`}
                    className="form-check-input"
                    type="checkbox"
                    value={user.name}
                    checked={participants.includes(user.name)}
                    onChange={(e) =>
                      handleParticipantToggle(user.name, e.target.checked)
                    }
                  />
                  <label
                    className="form-check-label"
                    htmlFor={`participant-${user.id}`}
                  >
                    {user.name}
                  </label>
                </div>
              ))}
            </div>
          </div>

          <button
            type="submit"
            className="btn btn-primary w-100"
            disabled={isSubmitting}
          >
            {isSubmitting
              ? isEditing
                ? "Saving..."
                : "Adding..."
              : isEditing
                ? "Save Changes"
                : "Add Expense"}
          </button>
        </form>
      </div>
    </div>
  );
};

export default ExpenseForm;
