import React, { useEffect, useState } from 'react';
import { getGroup, getBalances, getSettlements, deleteExpense } from '../services/api';
import * as Types from '../types';

interface DashboardProps {
    refresh: boolean;
    onRefresh: () => void;
    onEditGroup: () => void;
    onDeleteGroup: () => void;
}

const Dashboard: React.FC<DashboardProps> = ({ refresh, onRefresh, onEditGroup, onDeleteGroup }) => {
    const [group, setGroup] = useState<Types.Group | null>(null);
    const [balances, setBalances] = useState<Record<string, number>>({});
    const [settlements, setSettlements] = useState<Types.Settlement[]>([]);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetchGroupData();
    }, [refresh]);

    const fetchGroupData = async () => {
        setError(null);
        try {
            const [groupData, balancesData, settlementsData] = await Promise.all([
                getGroup(),
                getBalances(),
                getSettlements()
            ]);
            setGroup(groupData);
            setBalances(balancesData.balances);
            setSettlements(settlementsData.settlements);
        } catch (err: any) {
            console.error("Failed to fetch group data:", err);
            setError(err.response?.data?.error || err.message || "Unknown error occurred");
        }
    };

    const handleDeleteExpense = async (expenseId: number) => {
        if (!confirm('Are you sure you want to delete this expense?')) {
            return;
        }
        try {
            await deleteExpense(expenseId);
            onRefresh();
        } catch (err: any) {
            console.error("Failed to delete expense:", err);
            alert(err.response?.data?.error || "Failed to delete expense");
        }
    };

    if (error) {
        return <div className="alert alert-danger mt-3">Error loading data: {error}</div>;
    }

    if (!group) {
        return (
            <div className="loading-state">
                <div className="loading-spinner"></div>
                <p style={{marginTop: '1rem'}}>Loading group data...</p>
            </div>
        );
    }

    return (
        <div>
            <div className="card dashboard-card">
                <div className="dashboard-group-header">
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                        <h3 className="dashboard-group-name">{group.name}</h3>
                        <div style={{ display: 'flex', gap: '0.5rem' }}>
                            <button
                                className="btn btn-sm btn-secondary"
                                onClick={onEditGroup}
                                title="Edit group name"
                            >
                                ✏️ Edit
                            </button>
                            <button
                                className="btn btn-sm btn-danger"
                                onClick={onDeleteGroup}
                                title="Delete group"
                            >
                                🗑️ Delete
                            </button>
                        </div>
                    </div>
                </div>

                <div className="card-body settlement-card">
                    <h5 className="card-title">💰 Who Pays Whom</h5>
                    {settlements.length > 0 ? (
                        <div>
                            {settlements.map((settlement, index) => (
                                <div key={index} className="settlement-item">
                                    <div><strong>{settlement.from}</strong> pays <strong>{settlement.to}</strong></div>
                                    <div className="settlement-amount">${settlement.amount.toFixed(2)}</div>
                                </div>
                            ))}
                        </div>
                    ) : (
                        <div className="empty-state">
                            <div className="empty-state-icon">✓</div>
                            <p>All settled up!</p>
                        </div>
                    )}
                </div>
            </div>

            <div className="card dashboard-card mt-3">
                <div className="card-body">
                    <h5 className="card-title">📊 Balance Summary</h5>
                    <div className="list-group">
                        {Object.entries(balances).map(([user, balance]) => (
                            <div key={user} className="balance-item list-group-item">
                                <span className="balance-name">{user}</span>
                                <span className={`balance-amount ${balance >= 0 ? 'text-success' : 'text-danger'}`}>
                                    {balance >= 0 ? '+' : ''}${balance.toFixed(2)}
                                </span>
                            </div>
                        ))}
                    </div>
                </div>
            </div>

            <div className="card dashboard-card mt-3">
                <div className="card-body">
                    <h5 className="card-title">📝 All Expenses</h5>
                    {group.expenses.length > 0 ? (
                        <div className="list-group">
                            {group.expenses.map((expense) => {
                                const date = new Date(expense.created_at);
                                const dateStr = date.toLocaleDateString('en-US', {
                                    month: 'short',
                                    day: 'numeric',
                                    year: date.getFullYear() !== new Date().getFullYear() ? 'numeric' : undefined
                                });

                                return (
                                    <div key={expense.id} className="expense-item list-group-item">
                                        <div className="expense-content">
                                            <div className="expense-title">
                                                {expense.description}
                                                {expense.category && (
                                                    <span className="expense-category">{expense.category}</span>
                                                )}
                                            </div>
                                            <div className="expense-amount">${expense.amount.toFixed(2)}</div>
                                            <div className="expense-details">
                                                {dateStr} • Paid by <span className="expense-payer">{expense.payer.name}</span> •
                                                Split between {expense.participants.map(p => p.name).join(', ')}
                                            </div>
                                            {expense.notes && (
                                                <div className="expense-notes">
                                                    📝 {expense.notes}
                                                </div>
                                            )}
                                        </div>
                                        <div className="expense-actions">
                                            <button
                                                className="btn btn-sm btn-danger"
                                                onClick={() => handleDeleteExpense(expense.id)}
                                            >
                                                Delete
                                            </button>
                                        </div>
                                    </div>
                                );
                            })}
                        </div>
                    ) : (
                        <div className="empty-state">
                            <div className="empty-state-icon">📭</div>
                            <p>No expenses yet. Add one to get started!</p>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default Dashboard;
