<script lang="ts">
  import type { Expense } from "./types";
  import { formatDate } from "./utils/date";

  interface Props {
    expenses: Expense[];
    onDeleteExpense: (id: number, description: string) => void;
    onEditExpense: (expense: Expense) => void;
  }

  let { expenses, onDeleteExpense, onEditExpense }: Props = $props();
</script>

<div class="card dashboard-card mt-3">
  <div class="card-body">
    <h5 class="card-title">All Expenses</h5>
    {#if expenses.length > 0}
      <div class="list-group">
        {#each expenses as expense (expense.id)}
          <div class="expense-item list-group-item">
            <div class="expense-content">
              <div class="expense-title">
                {expense.description}
                {#if expense.category}
                  <span class="expense-category">{expense.category}</span>
                {/if}
              </div>
              <div class="expense-amount">
                ${expense.amount.toFixed(2)}
              </div>
              <div class="expense-details">
                {formatDate(expense.created_at)} • Paid by
                <span class="expense-payer">{expense.payer.name}</span>
                • Split between {expense.participants.map((p) => p.name).join(", ")}
              </div>
              {#if expense.notes}
                <div class="expense-notes">{expense.notes}</div>
              {/if}
            </div>
            <div class="expense-actions">
              <button
                class="btn btn-sm btn-secondary"
                onclick={() => onEditExpense(expense)}
              >
                Edit
              </button>
              <button
                class="btn btn-sm btn-danger"
                onclick={() => onDeleteExpense(expense.id, expense.description)}
              >
                Delete
              </button>
            </div>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-state-icon">📭</div>
        <p>No expenses yet. Add one to get started!</p>
      </div>
    {/if}
  </div>
</div>
