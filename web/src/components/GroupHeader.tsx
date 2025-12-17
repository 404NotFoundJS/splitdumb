import React, { useState } from "react";

interface GroupHeaderProps {
  groupName: string;
  groupId: number;
  onUpdateGroup: (groupId: number, newName: string) => Promise<void>;
  onDeleteGroup: () => void;
}

const GroupHeader: React.FC<GroupHeaderProps> = ({
  groupName,
  groupId,
  onUpdateGroup,
  onDeleteGroup,
}) => {
  const [isEditing, setIsEditing] = useState(false);
  const [editedName, setEditedName] = useState("");

  const startEdit = () => {
    setEditedName(groupName);
    setIsEditing(true);
  };

  const cancelEdit = () => {
    setIsEditing(false);
    setEditedName("");
  };

  const saveEdit = async () => {
    if (!editedName.trim()) return;

    try {
      await onUpdateGroup(groupId, editedName);
      setIsEditing(false);
      setEditedName("");
    } catch {
      setEditedName(groupName);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") saveEdit();
    if (e.key === "Escape") cancelEdit();
  };

  return (
    <div className="dashboard-group-header">
      {isEditing ? (
        <div className="edit-form-row">
          <input
            type="text"
            className="form-control edit-form-input"
            value={editedName}
            onChange={(e) => setEditedName(e.target.value)}
            onKeyDown={handleKeyDown}
            autoFocus
          />
          <button className="btn btn-sm btn-success" onClick={saveEdit}>
            Save
          </button>
          <button className="btn btn-sm btn-secondary" onClick={cancelEdit}>
            Cancel
          </button>
        </div>
      ) : (
        <div className="flex-between">
          <h3 className="dashboard-group-name">{groupName}</h3>
          <div className="flex-center gap-sm">
            <button
              className="btn btn-sm btn-secondary"
              onClick={startEdit}
              title="Edit group name"
            >
              Edit
            </button>
            <button
              className="btn btn-sm btn-danger"
              onClick={onDeleteGroup}
              title="Delete group"
            >
              Delete
            </button>
          </div>
        </div>
      )}
    </div>
  );
};

export default GroupHeader;
