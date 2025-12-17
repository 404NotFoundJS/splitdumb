import React, { useEffect, useState, useCallback } from "react";
import {
  getGroup,
  getBalances,
  getSettlements,
  toggleSimplify,
  deleteExpense,
  deleteUser,
  settle,
} from "../services/api";
import * as Types from "../types";
import { useToast } from "../contexts/ToastContext";
import GroupHeader from "./GroupHeader";
import MemberList from "./MemberList";
import UserForm from "./UserForm";
import ExpenseList from "./ExpenseList";
import ExpenseForm from "./ExpenseForm";
import BalanceSummary from "./BalanceSummary";
import SettlementCard from "./SettlementCard";

interface DashboardProps {
  refreshKey: number;
  onRefresh: () => void;
  onUpdateGroup: (groupId: number, newName: string) => Promise<void>;
  onDeleteGroup: () => void;
}

const Dashboard: React.FC<DashboardProps> = ({
  refreshKey,
  onRefresh,
  onUpdateGroup,
  onDeleteGroup,
}) => {
  const toast = useToast();
  const [group, setGroup] = useState<Types.Group | null>(null);
  const [balances, setBalances] = useState<Record<string, number>>({});
  const [settlements, setSettlements] = useState<Types.Settlement[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [editingExpense, setEditingExpense] = useState<Types.Expense | null>(
    null,
  );

  const fetchGroupData = useCallback(async () => {
    setError(null);
    try {
      const [groupData, balancesData, settlementsData] = await Promise.all([
        getGroup(),
        getBalances(),
        getSettlements(),
      ]);
      setGroup(groupData);
      setBalances(balancesData.balances);
      setSettlements(settlementsData.settlements);
    } catch (err) {
      const errorMsg =
        err instanceof Error ? err.message : "Unknown error occurred";
      setError(errorMsg);
      toast.error(errorMsg);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const fetchGroupOnly = useCallback(async () => {
    try {
      const groupData = await getGroup();
      setGroup(groupData);
    } catch (err) {
      toast.error(
        err instanceof Error ? err.message : "Failed to refresh group",
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    fetchGroupData();
  }, [refreshKey, fetchGroupData]);

  const handleToggleSimplify = useCallback(async () => {
    try {
      await toggleSimplify();
      onRefresh(); // Refresh to get updated settlements
    } catch (err) {
      toast.error(
        err instanceof Error ? err.message : "Failed to toggle simplify",
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [onRefresh]);

  const handleDeleteExpense = async (
    expenseId: number,
    description: string,
  ) => {
    toast.confirm("Are you sure you want to delete this expense?", async () => {
      if (!group) return;

      const previousExpenses = [...group.expenses];
      setGroup({
        ...group,
        expenses: group.expenses.filter((e) => e.id !== expenseId),
      });

      try {
        await deleteExpense(expenseId);
        onRefresh();
        toast.success(`Expense "${description}" deleted successfully`);
      } catch (err) {
        setGroup({ ...group, expenses: previousExpenses });
        toast.error(
          err instanceof Error ? err.message : "Failed to delete expense",
        );
      }
    });
  };

  const handleDeleteUser = async (userId: number, userName: string) => {
    toast.confirm(
      `Are you sure you want to remove ${userName} from this group?`,
      async () => {
        if (!group) return;

        const previousMembers = [...group.members];
        setGroup({
          ...group,
          members: group.members.filter((m) => m.id !== userId),
        });

        try {
          await deleteUser(userId);
          onRefresh();
          toast.success(`${userName} removed from group successfully`);
        } catch (err) {
          setGroup({ ...group, members: previousMembers });
          toast.error(
            err instanceof Error ? err.message : "Failed to remove user",
          );
        }
      },
    );
  };

  const handleSettle = async (from: string, to: string, amount: number) => {
    toast.confirm(
      `Mark ${from} as having paid ${to} $${amount.toFixed(2)}?`,
      async () => {
        setSettlements((prev) =>
          prev.map((s) =>
            s.from === from && s.to === to ? { ...s, settled: true } : s,
          ),
        );

        try {
          await settle(from, to, amount);
          await fetchGroupOnly();
          toast.success(`Settlement recorded: ${from} paid ${to}`);
        } catch (err) {
          setSettlements((prev) =>
            prev.map((s) =>
              s.from === from && s.to === to ? { ...s, settled: false } : s,
            ),
          );
          toast.error(
            err instanceof Error ? err.message : "Failed to record settlement",
          );
        }
      },
    );
  };

  if (error) {
    return (
      <div className="alert alert-danger mt-3">Error loading data: {error}</div>
    );
  }

  if (!group) {
    return (
      <div className="loading-state">
        <div className="loading-spinner"></div>
        <p>Loading group data...</p>
      </div>
    );
  }

  return (
    <div className="dashboard">
      <div className="card dashboard-card">
        <GroupHeader
          groupName={group.name}
          groupId={group.id}
          onUpdateGroup={onUpdateGroup}
          onDeleteGroup={onDeleteGroup}
        />
      </div>

      <div className="dashboard-grid">
        <div className="dashboard-column">
          <MemberList members={group.members} onDeleteUser={handleDeleteUser} />
          <UserForm onUserAdded={onRefresh} />
        </div>

        <div className="dashboard-column dashboard-column-wide">
          <ExpenseList
            expenses={group.expenses}
            onDeleteExpense={handleDeleteExpense}
            onEditExpense={setEditingExpense}
          />
          <ExpenseForm
            onExpenseAdded={onRefresh}
            refreshKey={refreshKey}
            editingExpense={editingExpense}
            onCancelEdit={() => setEditingExpense(null)}
          />
        </div>

        <div className="dashboard-column">
          <BalanceSummary balances={balances} />
          <SettlementCard
            settlements={settlements}
            onSettle={handleSettle}
            isSimplified={group.simplify_debts}
            onToggleSimplify={handleToggleSimplify}
          />
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
