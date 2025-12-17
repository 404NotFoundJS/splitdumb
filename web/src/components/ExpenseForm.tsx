import React, { useState, useEffect } from "react";
import { createExpense, getGroup } from "../services/api";
import * as Types from "../types";

interface ExpenseFormProps {
  onExpenseAdded: () => void;
  refresh: boolean;
}

const ExpenseForm: React.FC<ExpenseFormProps> = ({
  onExpenseAdded,
  refresh,
}) => {
  const [description, setDescription] = useState("");
  const [amount, setAmount] = useState<number>(0);
  const [payer, setPayer] = useState("");
  const [participants, setParticipants] = useState<string[]>([]);
  const [category, setCategory] = useState("");
  const [notes, setNotes] = useState("");
  const [users, setUsers] = useState<Types.User[]>([]);
  const [error, setError] = useState<string | null>(null);

  const categories = [
    "Food",
    "Transport",
    "Entertainment",
    "Accommodation",
    "Shopping",
    "Other",
  ];

  useEffect(() => {
    fetchUsers();
  }, [refresh]);

  const fetchUsers = async () => {
    const group = await getGroup();
    setUsers(group.members);
    if (group.members.length > 0) {
      setPayer(group.members[0].name);
    }
  };

  const handleParticipantChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { value, checked } = e.target;
    if (checked) {
      setParticipants([...participants, value]);
    } else {
      setParticipants(participants.filter((p) => p !== value));
    }
  };

  const handleSelectAll = () => {
    setParticipants(users.map((u) => u.name));
  };

  const handleClearAll = () => {
    setParticipants([]);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!description || amount <= 0 || !payer || participants.length === 0)
      return;

    setError(null);
    try {
      await createExpense(
        description,
        amount,
        payer,
        participants,
        category || undefined,
        notes || undefined,
      );
      setDescription("");
      setAmount(0);
      setParticipants([]);
      setCategory("");
      setNotes("");
      onExpenseAdded();
    } catch (err: any) {
      setError(err.response?.data?.error || "Failed to add expense");
    }
  };

  return (
    <div className="card form-card">
      <div className="card-body">
        <h5 className="card-title">💸 Add New Expense</h5>
        {error && (
          <div className="alert alert-danger" style={{ marginBottom: "1rem" }}>
            {error}
          </div>
        )}
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
              {categories.map((cat) => (
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
            <div
              style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
                marginBottom: "0.5rem",
              }}
            >
              <label className="form-label" style={{ margin: 0 }}>
                Split Between
              </label>
              <div>
                <button
                  type="button"
                  className="btn btn-sm"
                  onClick={handleSelectAll}
                  style={{
                    marginRight: "0.5rem",
                    fontSize: "0.75rem",
                    padding: "0.25rem 0.5rem",
                  }}
                >
                  Select All
                </button>
                <button
                  type="button"
                  className="btn btn-sm"
                  onClick={handleClearAll}
                  style={{ fontSize: "0.75rem", padding: "0.25rem 0.5rem" }}
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
                    onChange={handleParticipantChange}
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
          <button type="submit" className="btn btn-primary w-100">
            Add Expense
          </button>
        </form>
      </div>
    </div>
  );
};

export default ExpenseForm;
