import React, { memo } from "react";
import * as Types from "../types";

interface SettlementCardProps {
  settlements: Types.Settlement[];
  onSettle: (from: string, to: string, amount: number) => Promise<void>;
  isSimplified: boolean;
  onToggleSimplify: () => void;
}

const SettlementCard: React.FC<SettlementCardProps> = memo(
  ({ settlements, onSettle, isSimplified, onToggleSimplify }) => {
    return (
      <div className="card-body settlement-card">
        <div className="settlement-header">
          <h5 className="card-title">Who Pays Whom</h5>
          <button
            className={`btn btn-sm ${isSimplified ? "btn-primary" : "btn-outline"}`}
            onClick={onToggleSimplify}
            title={
              isSimplified
                ? "Showing simplified debts (fewer transactions)"
                : "Showing pairwise debts (stable)"
            }
          >
            {isSimplified ? "Simplified" : "Simplify"}
          </button>
        </div>
        {settlements.length > 0 ? (
          <div>
            {settlements.map((settlement, index) => {
              return (
                <div
                  key={index}
                  className={`settlement-item ${settlement.settled ? "settlement-item-settled" : ""}`}
                >
                  <div>
                    <strong>{settlement.from}</strong> pays{" "}
                    <strong>{settlement.to}</strong>
                  </div>
                  <div className="flex-center gap-sm">
                    <span className="settlement-amount">
                      ${settlement.amount.toFixed(2)}
                    </span>
                    {settlement.settled ? (
                      <span className="badge badge-settled">Settled</span>
                    ) : (
                      <button
                        className="btn btn-sm btn-success"
                        onClick={() =>
                          onSettle(
                            settlement.from,
                            settlement.to,
                            settlement.amount,
                          )
                        }
                      >
                        Settle
                      </button>
                    )}
                  </div>
                </div>
              );
            })}
          </div>
        ) : (
          <div className="empty-state">
            <div className="empty-state-icon">✓</div>
            <p>All settled up!</p>
          </div>
        )}
      </div>
    );
  },
);

SettlementCard.displayName = "SettlementCard";

export default SettlementCard;
