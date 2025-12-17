import { useState, useEffect, useCallback } from "react";
import Dashboard from "./components/Dashboard";
import {
  listGroups,
  createGroup,
  switchGroup,
  updateGroup,
  deleteGroup,
} from "./services/api";
import * as Types from "./types";
import { useToast } from "./contexts/ToastContext";
import "./App.css";

function App() {
  const toast = useToast();
  const [refreshKey, setRefreshKey] = useState(0);
  const [groups, setGroups] = useState<Types.Group[]>([]);
  const [currentGroupId, setCurrentGroupId] = useState<number>(1);
  const [showCreateGroup, setShowCreateGroup] = useState(false);
  const [newGroupName, setNewGroupName] = useState("");

  const fetchGroups = useCallback(async () => {
    try {
      const fetchedGroups = await listGroups();
      setGroups(fetchedGroups);
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to fetch groups",
      );
    }
  }, [toast]);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    fetchGroups();
  }, [fetchGroups]);

  const handleUpdate = useCallback(() => {
    setRefreshKey((k) => k + 1);
  }, []);

  const handleSwitchGroup = async (groupId: number) => {
    const previousGroupId = currentGroupId;
    setCurrentGroupId(groupId);
    handleUpdate();

    try {
      await switchGroup(groupId);
    } catch (error) {
      setCurrentGroupId(previousGroupId);
      handleUpdate();
      toast.error(
        error instanceof Error ? error.message : "Failed to switch group",
      );
    }
  };

  const handleCreateGroup = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newGroupName.trim()) return;

    const tempGroup: Types.Group = {
      id: Date.now(),
      name: newGroupName,
      members: [],
      expenses: [],
    };

    setGroups([...groups, tempGroup]);
    setNewGroupName("");
    setShowCreateGroup(false);

    try {
      const newGroup = await createGroup(tempGroup.name);
      await fetchGroups();
      await handleSwitchGroup(newGroup.id);
      toast.success(`Group "${newGroup.name}" created successfully`);
    } catch (error) {
      setGroups(groups.filter((g) => g.id !== tempGroup.id));
      setShowCreateGroup(true);
      setNewGroupName(tempGroup.name);
      toast.error(
        error instanceof Error ? error.message : "Failed to create group",
      );
    }
  };

  const handleUpdateGroup = async (groupId: number, newName: string) => {
    const previousGroups = [...groups];
    setGroups(
      groups.map((g) => (g.id === groupId ? { ...g, name: newName } : g)),
    );

    try {
      await updateGroup(groupId, newName);
      await fetchGroups();
      handleUpdate();
      toast.success("Group name updated successfully");
    } catch (error) {
      setGroups(previousGroups);
      toast.error(
        error instanceof Error ? error.message : "Failed to update group",
      );
      throw error;
    }
  };

  const handleDeleteGroup = async (groupId: number) => {
    const group = groups.find((g) => g.id === groupId);
    if (!group) return;

    toast.confirm(
      `Are you sure you want to delete "${group.name}"? This will permanently delete all users and expenses in this group.`,
      async () => {
        const previousGroups = [...groups];
        const previousGroupId = currentGroupId;
        setGroups(groups.filter((g) => g.id !== groupId));

        try {
          const response = await deleteGroup(groupId);

          if (response.switched_group) {
            setCurrentGroupId(response.switched_group);
          }

          await fetchGroups();
          handleUpdate();
          toast.success(`Group "${group.name}" deleted successfully`);
        } catch (error) {
          setGroups(previousGroups);
          setCurrentGroupId(previousGroupId);
          toast.error(
            error instanceof Error ? error.message : "Failed to delete group",
          );
        }
      },
    );
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      e.preventDefault();
      handleCreateGroup(e);
    }
    if (e.key === "Escape") {
      setShowCreateGroup(false);
      setNewGroupName("");
    }
  };

  const cancelCreateGroup = () => {
    setShowCreateGroup(false);
    setNewGroupName("");
  };

  return (
    <>
      <header className="app-header">
        <div className="container">
          <div className="header-content">
            <div>
              <h1 className="app-title">Splitdumb</h1>
            </div>
            <div className="header-actions">
              {showCreateGroup ? (
                <>
                  <input
                    type="text"
                    className="form-control header-input"
                    placeholder="New group name..."
                    value={newGroupName}
                    onChange={(e) => setNewGroupName(e.target.value)}
                    onKeyDown={handleKeyDown}
                    autoFocus
                  />
                  <button
                    className="btn btn-sm btn-success"
                    onClick={handleCreateGroup}
                  >
                    Create
                  </button>
                  <button
                    className="btn btn-sm btn-secondary"
                    onClick={cancelCreateGroup}
                  >
                    Cancel
                  </button>
                </>
              ) : (
                <>
                  <select
                    className="form-select header-select"
                    value={currentGroupId}
                    onChange={(e) => handleSwitchGroup(Number(e.target.value))}
                  >
                    {groups.map((group) => (
                      <option key={group.id} value={group.id}>
                        {group.name}
                      </option>
                    ))}
                  </select>
                  <button
                    className="btn btn-sm btn-primary"
                    onClick={() => setShowCreateGroup(true)}
                  >
                    + New
                  </button>
                </>
              )}
            </div>
          </div>
        </div>
      </header>

      <div className="app-container">
        <Dashboard
          refreshKey={refreshKey}
          onRefresh={handleUpdate}
          onUpdateGroup={handleUpdateGroup}
          onDeleteGroup={() => handleDeleteGroup(currentGroupId)}
        />
      </div>
    </>
  );
}

export default App;
