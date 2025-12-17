import { useState, useEffect } from "react";
import Dashboard from "./components/Dashboard";
import UserForm from "./components/UserForm";
import ExpenseForm from "./components/ExpenseForm";
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
  const [refreshDashboard, setRefreshDashboard] = useState(false);
  const [groups, setGroups] = useState<Types.Group[]>([]);
  const [currentGroupId, setCurrentGroupId] = useState<number>(1);
  const [showCreateGroup, setShowCreateGroup] = useState(false);
  const [newGroupName, setNewGroupName] = useState("");

  useEffect(() => {
    fetchGroups();
  }, []);

  const fetchGroups = async () => {
    try {
      const fetchedGroups = await listGroups();
      setGroups(fetchedGroups);
    } catch (error: any) {
      toast.error(error.response?.data?.error || "Failed to fetch groups");
    }
  };

  const handleUpdate = () => {
    setRefreshDashboard(!refreshDashboard);
  };

  const handleSwitchGroup = async (groupId: number) => {
    const previousGroupId = currentGroupId;
    setCurrentGroupId(groupId);
    handleUpdate();

    try {
      await switchGroup(groupId);
    } catch (error: any) {
      setCurrentGroupId(previousGroupId);
      handleUpdate();
      toast.error(error.response?.data?.error || "Failed to switch group");
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
    } catch (error: any) {
      setGroups(groups.filter((g) => g.id !== tempGroup.id));
      setShowCreateGroup(true);
      setNewGroupName(tempGroup.name);
      toast.error(error.response?.data?.error || "Failed to create group");
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
    } catch (error: any) {
      setGroups(previousGroups);
      toast.error(error.response?.data?.error || "Failed to update group");
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
        } catch (error: any) {
          setGroups(previousGroups);
          setCurrentGroupId(previousGroupId);
          toast.error(error.response?.data?.error || "Failed to delete group");
        }
      },
    );
  };

  return (
    <>
      <header className="app-header">
        <div className="container">
          <div
            style={{
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
            }}
          >
            <div>
              <h1 className="app-title">Splitdumb</h1>
            </div>
            <div
              style={{
                display: "flex",
                gap: "0.5rem",
                alignItems: "center",
                flexWrap: "wrap",
              }}
            >
              {showCreateGroup ? (
                <>
                  <input
                    type="text"
                    className="form-control"
                    placeholder="New group name..."
                    value={newGroupName}
                    onChange={(e) => setNewGroupName(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === "Enter") {
                        e.preventDefault();
                        handleCreateGroup(e);
                      }
                      if (e.key === "Escape") {
                        setShowCreateGroup(false);
                        setNewGroupName("");
                      }
                    }}
                    autoFocus
                    style={{ width: "200px" }}
                  />
                  <button
                    className="btn btn-sm btn-success"
                    onClick={handleCreateGroup}
                  >
                    💾 Create
                  </button>
                  <button
                    className="btn btn-sm btn-secondary"
                    onClick={() => {
                      setShowCreateGroup(false);
                      setNewGroupName("");
                    }}
                  >
                    ✖️ Cancel
                  </button>
                </>
              ) : (
                <>
                  <select
                    className="form-select"
                    style={{ width: "200px" }}
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
        <div className="row">
          <div className="col-md-6">
            <UserForm onUserAdded={handleUpdate} />
            <ExpenseForm
              onExpenseAdded={handleUpdate}
              refresh={refreshDashboard}
            />
          </div>
          <div className="col-md-6">
            <Dashboard
              refresh={refreshDashboard}
              onRefresh={handleUpdate}
              onUpdateGroup={handleUpdateGroup}
              onDeleteGroup={() => handleDeleteGroup(currentGroupId)}
            />
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
