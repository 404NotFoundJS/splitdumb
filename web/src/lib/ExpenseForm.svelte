<script lang="ts">
  import { createExpense, updateExpense, getGroup } from "./api";
  import type { Expense, User } from "./types";
  import { toast } from "./stores/toast";

  interface Props {
    onExpenseAdded: () => void;
    refreshKey: number;
    editingExpense?: Expense | null;
    onCancelEdit?: () => void;
  }

  let { onExpenseAdded, refreshKey, editingExpense = null, onCancelEdit }: Props = $props();

  const CATEGORIES = [
    "Food",
    "Transport",
    "Entertainment",
    "Accommodation",
    "Shopping",
    "Other",
  ];

  let description = $state("");
  let amount = $state(0);
  let payer = $state("");
  let participants = $state<string[]>([]);
  let category = $state("");
  let notes = $state("");
  let users = $state<User[]>([]);
  let isSubmitting = $state(false);

  let isEditing = $derived(!!editingExpense);

  async function fetchUsers() {
    try {
      const group = await getGroup();
      users = group.members;
      if (group.members.length > 0 && !payer) {
        payer = group.members[0].name;
      }
    } catch {
      // Silently fail
    }
  }

  function resetForm() {
    description = "";
    amount = 0;
    participants = [];
    category = "";
    notes = "";
    if (users.length > 0) {
      payer = users[0].name;
    }
  }

  function handleCancel() {
    resetForm();
    onCancelEdit?.();
  }

  function handleParticipantToggle(name: string, checked: boolean) {
    if (checked) {
      participants = [...participants, name];
    } else {
      participants = participants.filter((p) => p !== name);
    }
  }

  function handleSelectAll() {
    participants = users.map((u) => u.name);
  }

  function handleClearAll() {
    participants = [];
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!description || amount <= 0 || !payer || participants.length === 0 || isSubmitting) {
      return;
    }

    isSubmitting = true;
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
          expenseData.notes || undefined
        );
        toast.success(`Expense "${expenseData.description}" added`);
        resetForm();
      }
      onExpenseAdded();
    } catch (err) {
      toast.error(
        err instanceof Error
          ? err.message
          : `Failed to ${isEditing ? "update" : "add"} expense`
      );
    } finally {
      isSubmitting = false;
    }
  }

  // Fetch users on refreshKey change
  $effect(() => {
    refreshKey;
    fetchUsers();
  });

  // Load editing expense data
  $effect(() => {
    if (editingExpense) {
      description = editingExpense.description;
      amount = editingExpense.amount;
      payer = editingExpense.payer.name;
      participants = editingExpense.participants.map((p) => p.name);
      category = editingExpense.category || "";
      notes = editingExpense.notes || "";
    } else {
      description = "";
      amount = 0;
      participants = [];
      category = "";
      notes = "";
    }
  });
</script>

<div class="card form-card">
  <div class="card-body">
    <div class="form-header">
      <h5 class="card-title">
        {isEditing ? "Edit Expense" : "Add New Expense"}
      </h5>
      {#if isEditing}
        <button
          type="button"
          class="btn btn-sm btn-secondary"
          onclick={handleCancel}
        >
          Cancel
        </button>
      {/if}
    </div>
    <form onsubmit={handleSubmit}>
      <div class="form-group">
        <label for="expenseDescription" class="form-label">Description</label>
        <input
          id="expenseDescription"
          type="text"
          class="form-control"
          placeholder="e.g., Dinner, Movie tickets"
          bind:value={description}
          disabled={isSubmitting}
          required
        />
      </div>

      <div class="form-group">
        <label for="expenseAmount" class="form-label">Amount ($)</label>
        <input
          id="expenseAmount"
          type="number"
          class="form-control"
          placeholder="0.00"
          step="0.01"
          min="0.01"
          bind:value={amount}
          disabled={isSubmitting}
          required
        />
      </div>

      <div class="form-group">
        <label for="expensePayer" class="form-label">Who Paid?</label>
        <select
          id="expensePayer"
          class="form-select"
          bind:value={payer}
          required
        >
          {#each users as user (user.id)}
            <option value={user.name}>{user.name}</option>
          {/each}
        </select>
      </div>

      <div class="form-group">
        <label for="expenseCategory" class="form-label">Category (Optional)</label>
        <select
          id="expenseCategory"
          class="form-select"
          bind:value={category}
        >
          <option value="">No category</option>
          {#each CATEGORIES as cat}
            <option value={cat}>{cat}</option>
          {/each}
        </select>
      </div>

      <div class="form-group">
        <label for="expenseNotes" class="form-label">Notes (Optional)</label>
        <textarea
          id="expenseNotes"
          class="form-control"
          placeholder="Add any additional details..."
          rows="3"
          bind:value={notes}
        ></textarea>
      </div>

      <div class="form-group" role="group" aria-labelledby="participants-label">
        <div class="form-actions-row">
          <span id="participants-label" class="form-label">Split Between</span>
          <div>
            <button
              type="button"
              class="btn btn-sm btn-inline"
              onclick={handleSelectAll}
            >
              Select All
            </button>
            <button
              type="button"
              class="btn btn-sm btn-inline"
              onclick={handleClearAll}
            >
              Clear
            </button>
          </div>
        </div>
        <div class="participants-group">
          {#each users as user (user.id)}
            <div class="form-check">
              <input
                id="participant-{user.id}"
                class="form-check-input"
                type="checkbox"
                value={user.name}
                checked={participants.includes(user.name)}
                onchange={(e) => handleParticipantToggle(user.name, (e.target as HTMLInputElement).checked)}
              />
              <label class="form-check-label" for="participant-{user.id}">
                {user.name}
              </label>
            </div>
          {/each}
        </div>
      </div>

      <button
        type="submit"
        class="btn btn-primary w-100"
        disabled={isSubmitting}
      >
        {#if isSubmitting}
          {isEditing ? "Saving..." : "Adding..."}
        {:else}
          {isEditing ? "Save Changes" : "Add Expense"}
        {/if}
      </button>
    </form>
  </div>
</div>
