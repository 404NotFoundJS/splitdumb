import React from "react";
import * as Types from "../types";

interface MemberListProps {
  members: Types.User[];
  onDeleteUser: (id: number, name: string) => void;
}

const MemberList: React.FC<MemberListProps> = ({ members, onDeleteUser }) => {
  return (
    <div className="card dashboard-card mt-3">
      <div className="card-body">
        <h5 className="card-title">Group Members</h5>
        {members.length > 0 ? (
          <div className="list-group">
            {members.map((user) => (
              <div key={user.id} className="list-group-item member-item">
                <span className="balance-name">{user.name}</span>
                <button
                  className="btn btn-sm btn-danger"
                  onClick={() => onDeleteUser(user.id, user.name)}
                  title="Remove user from group"
                >
                  Remove
                </button>
              </div>
            ))}
          </div>
        ) : (
          <div className="empty-state">
            <div className="empty-state-icon">👤</div>
            <p>No members yet. Add someone to get started!</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default MemberList;
