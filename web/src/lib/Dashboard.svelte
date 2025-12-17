<script lang="ts">
  import {
    getGroup,
    getBalances,
    getSettlements,
    toggleSimplify,
    deleteExpense,
    deleteUser,
    settle,
  } from "./api";
  import type { Group, Expense, Settlement } from "./types";
  import { toast } from "./stores/toast";
  import GroupHeader from "./GroupHeader.svelte";
  import MemberList from "./MemberList.svelte";
  import UserForm from "./UserForm.svelte";
  import ExpenseList from "./ExpenseList.svelte";
  import ExpenseForm from "./ExpenseForm.svelte";
  import BalanceSummary from "./BalanceSummary.svelte";
  import SettlementCard from "./SettlementCard.svelte";

  interface Props {
    refreshKey: number;
    onRefresh: () => void;
    onUpdateGroup: (groupId: number, newName: string) => Promise<void>;
    onDeleteGroup: () => void;
  }

  let { refreshKey, onRefresh, onUpdateGroup, onDeleteGroup }: Props = $props();

  let group = $state<Group | null>(null);
  let balances = $state<Record<string, number>>({});
  let settlements = $state<Settlement[]>([]);
  let error = $state<string | null>(null);
  let editingExpense = $state<Expense | null>(null);

  async function fetchGroupData() {
    error = null;
    try {
      const [groupData, balancesData, settlementsData] = await Promise.all([
        getGroup(),
        getBalances(),
        getSettlements(),
      ]);
      group = groupData;
      balances = balancesData.balances;
      settlements = settlementsData.settlements;
    } catch (err) {
      const errorMsg = err instanceof Error ? err.message : "Unknown error occurred";
      error = errorMsg;
      toast.error(errorMsg);
    }
  }

  async function fetchGroupOnly() {
    try {
      const groupData = await getGroup();
      group = groupData;
    } catch (err) {
      toast.error(err instanceof Error ? err.message : "Failed to refresh group");
    }
  }

  async function handleToggleSimplify() {
    try {
      await toggleSimplify();
      onRefresh();
    } catch (err) {
      toast.error(err instanceof Error ? err.message : "Failed to toggle simplify");
    }
  }

  function handleDeleteExpense(expenseId: number, description: string) {
    toast.confirm("Are you sure you want to delete this expense?", async () => {
      if (!group) return;

      const previousExpenses = [...group.expenses];
      group = {
        ...group,
        expenses: group.expenses.filter((e) => e.id !== expenseId),
      };

      try {
        await deleteExpense(expenseId);
        onRefresh();
        toast.success(`Expense "${description}" deleted successfully`);
      } catch (err) {
        group = { ...group!, expenses: previousExpenses };
        toast.error(err instanceof Error ? err.message : "Failed to delete expense");
      }
    });
  }

  function handleDeleteUser(userId: number, userName: string) {
    toast.confirm(
      `Are you sure you want to remove ${userName} from this group?`,
      async () => {
        if (!group) return;

        const previousMembers = [...group.members];
        group = {
          ...group,
          members: group.members.filter((m) => m.id !== userId),
        };

        try {
          await deleteUser(userId);
          onRefresh();
          toast.success(`${userName} removed from group successfully`);
        } catch (err) {
          group = { ...group!, members: previousMembers };
          toast.error(err instanceof Error ? err.message : "Failed to remove user");
        }
      }
    );
  }

  async function handleSettle(from: string, to: string, amount: number) {
    toast.confirm(
      `Mark ${from} as having paid ${to} $${amount.toFixed(2)}?`,
      async () => {
        settlements = settlements.map((s) =>
          s.from === from && s.to === to ? { ...s, settled: true } : s
        );

        try {
          await settle(from, to, amount);
          await fetchGroupOnly();
          toast.success(`Settlement recorded: ${from} paid ${to}`);
        } catch (err) {
          settlements = settlements.map((s) =>
            s.from === from && s.to === to ? { ...s, settled: false } : s
          );
          toast.error(err instanceof Error ? err.message : "Failed to record settlement");
        }
      }
    );
  }

  function handleEditExpense(expense: Expense) {
    editingExpense = expense;
  }

  function handleCancelEdit() {
    editingExpense = null;
  }

  // Fetch data on refreshKey change
  $effect(() => {
    refreshKey;
    fetchGroupData();
  });
</script>

{#if error}
  <div class="alert alert-danger mt-3">Error loading data: {error}</div>
{:else if !group}
  <div class="loading-state">
    <div class="loading-spinner"></div>
    <p>Loading group data...</p>
  </div>
{:else}
  <div class="dashboard">
    <div class="card dashboard-card">
      <GroupHeader
        groupName={group.name}
        groupId={group.id}
        {onUpdateGroup}
        {onDeleteGroup}
      />
    </div>

    <div class="dashboard-grid">
      <div class="dashboard-column">
        <MemberList members={group.members} onDeleteUser={handleDeleteUser} />
        <UserForm onUserAdded={onRefresh} />
      </div>

      <div class="dashboard-column dashboard-column-wide">
        <ExpenseList
          expenses={group.expenses}
          onDeleteExpense={handleDeleteExpense}
          onEditExpense={handleEditExpense}
        />
        <ExpenseForm
          onExpenseAdded={onRefresh}
          {refreshKey}
          {editingExpense}
          onCancelEdit={handleCancelEdit}
        />
      </div>

      <div class="dashboard-column">
        <BalanceSummary {balances} />
        <SettlementCard
          {settlements}
          onSettle={handleSettle}
          isSimplified={group.simplify_debts}
          onToggleSimplify={handleToggleSimplify}
        />
      </div>
    </div>
  </div>
{/if}
