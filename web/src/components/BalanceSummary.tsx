import React from "react";

interface BalanceSummaryProps {
  balances: Record<string, number>;
}

const BalanceSummary: React.FC<BalanceSummaryProps> = ({ balances }) => {
  return (
    <div className="card dashboard-card mt-3">
      <div className="card-body">
        <h5 className="card-title">Balance Summary</h5>
        <div className="list-group">
          {Object.entries(balances).map(([user, balance]) => (
            <div key={user} className="balance-item list-group-item">
              <span className="balance-name">{user}</span>
              <span
                className={`balance-amount ${balance >= 0 ? "text-success" : "text-danger"}`}
              >
                {balance >= 0 ? "+" : ""}${balance.toFixed(2)}
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default BalanceSummary;
