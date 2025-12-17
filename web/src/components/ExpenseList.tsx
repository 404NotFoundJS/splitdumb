import React from "react";
import * as Types from "../types";
import { formatDate } from "../utils/date";

interface ExpenseListProps {
  expenses: Types.Expense[];
  onDeleteExpense: (id: number, description: string) => void;
  onEditExpense: (expense: Types.Expense) => void;
}

const ExpenseList: React.FC<ExpenseListProps> = ({
  expenses,
  onDeleteExpense,
  onEditExpense,
}) => {
  return (
    <div className="card dashboard-card mt-3">
      <div className="card-body">
        <h5 className="card-title">All Expenses</h5>
        {expenses.length > 0 ? (
          <div className="list-group">
            {expenses.map((expense) => (
              <div key={expense.id} className="expense-item list-group-item">
                <div className="expense-content">
                  <div className="expense-title">
                    {expense.description}
                    {expense.category && (
                      <span className="expense-category">
                        {expense.category}
                      </span>
                    )}
                  </div>
                  <div className="expense-amount">
                    ${expense.amount.toFixed(2)}
                  </div>
                  <div className="expense-details">
                    {formatDate(expense.created_at)} • Paid by{" "}
                    <span className="expense-payer">{expense.payer.name}</span>{" "}
                    • Split between{" "}
                    {expense.participants.map((p) => p.name).join(", ")}
                  </div>
                  {expense.notes && (
                    <div className="expense-notes">{expense.notes}</div>
                  )}
                </div>
                <div className="expense-actions">
                  <button
                    className="btn btn-sm btn-secondary"
                    onClick={() => onEditExpense(expense)}
                  >
                    Edit
                  </button>
                  <button
                    className="btn btn-sm btn-danger"
                    onClick={() =>
                      onDeleteExpense(expense.id, expense.description)
                    }
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="empty-state">
            <div className="empty-state-icon">📭</div>
            <p>No expenses yet. Add one to get started!</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default ExpenseList;
