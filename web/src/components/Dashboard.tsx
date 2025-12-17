import React, { useEffect, useState, useCallback } from "react";
import {
  getGroup,
  getBalances,
  getSettlements,
  getSimplifiedSettlements,
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
  const [isSimplified, setIsSimplified] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchGroupData = useCallback(async () => {
    setError(null);
    try {
      const settlementsFunc = isSimplified
        ? getSimplifiedSettlements
        : getSettlements;
      const [groupData, balancesData, settlementsData] = await Promise.all([
        getGroup(),
        getBalances(),
        settlementsFunc(),
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
  }, [toast, isSimplified]);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    fetchGroupData();
  }, [refreshKey, fetchGroupData]);

  const handleToggleSimplify = useCallback(() => {
    setIsSimplified((prev) => !prev);
  }, []);

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
        try {
          await settle(from, to, amount);
          onRefresh();
          toast.success(`Settlement recorded: ${from} paid ${to}`);
        } catch (err) {
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
      {/* Group Header */}
      <div className="card dashboard-card">
        <GroupHeader
          groupName={group.name}
          groupId={group.id}
          onUpdateGroup={onUpdateGroup}
          onDeleteGroup={onDeleteGroup}
        />
      </div>

      {/* Three-column layout for main content */}
      <div className="dashboard-grid">
        {/* Column 1: Members */}
        <div className="dashboard-column">
          <MemberList members={group.members} onDeleteUser={handleDeleteUser} />
          <UserForm onUserAdded={onRefresh} />
        </div>

        {/* Column 2: Expenses */}
        <div className="dashboard-column dashboard-column-wide">
          <ExpenseList
            expenses={group.expenses}
            onDeleteExpense={handleDeleteExpense}
          />
          <ExpenseForm onExpenseAdded={onRefresh} refreshKey={refreshKey} />
        </div>

        {/* Column 3: Summary */}
        <div className="dashboard-column">
          <BalanceSummary balances={balances} />
          <SettlementCard
            settlements={settlements}
            onSettle={handleSettle}
            isSimplified={isSimplified}
            onToggleSimplify={handleToggleSimplify}
          />
        </div>
      </div>
    </div>
  );
};

export default Dashboard;
