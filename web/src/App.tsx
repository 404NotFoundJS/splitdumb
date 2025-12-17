import { useState, useEffect } from "react";
import Dashboard from "./components/Dashboard";
import UserForm from "./components/UserForm";
import ExpenseForm from "./components/ExpenseForm";
import { listGroups, createGroup, switchGroup } from "./services/api";
import * as Types from "./types";
import "./App.css";

function App() {
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
    } catch (error) {
      console.error("Failed to fetch groups:", error);
    }
  };

  const handleUpdate = () => {
    setRefreshDashboard(!refreshDashboard);
  };

  const handleSwitchGroup = async (groupId: number) => {
    try {
      await switchGroup(groupId);
      setCurrentGroupId(groupId);
      handleUpdate();
    } catch (error) {
      console.error("Failed to switch group:", error);
    }
  };

  const handleCreateGroup = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newGroupName.trim()) return;

    try {
      const newGroup = await createGroup(newGroupName);
      await fetchGroups();
      setNewGroupName("");
      setShowCreateGroup(false);
      await handleSwitchGroup(newGroup.id);
    } catch (error) {
      console.error("Failed to create group:", error);
    }
  };

  return (
    <>
      <header className="app-header">
        <div className="container">
          <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
            <div>
              <h1 className="app-title">Splitdumb</h1>
            </div>
            <div style={{ display: "flex", gap: "1rem", alignItems: "center" }}>
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
                className="btn btn-primary"
                onClick={() => setShowCreateGroup(!showCreateGroup)}
              >
                + New Group
              </button>
            </div>
          </div>
          {showCreateGroup && (
            <div style={{ marginTop: "1rem" }}>
              <form onSubmit={handleCreateGroup} style={{ display: "flex", gap: "0.5rem" }}>
                <input
                  type="text"
                  className="form-control"
                  placeholder="Group name"
                  value={newGroupName}
                  onChange={(e) => setNewGroupName(e.target.value)}
                  style={{ maxWidth: "250px" }}
                />
                <button type="submit" className="btn btn-success">
                  Create
                </button>
                <button
                  type="button"
                  className="btn btn-secondary"
                  onClick={() => {
                    setShowCreateGroup(false);
                    setNewGroupName("");
                  }}
                >
                  Cancel
                </button>
              </form>
            </div>
          )}
        </div>
      </header>

      <div className="app-container">
        <div className="row">
          <div className="col-md-6">
            <UserForm onUserAdded={handleUpdate} />
            <ExpenseForm onExpenseAdded={handleUpdate} refresh={refreshDashboard} />
          </div>
          <div className="col-md-6">
            <Dashboard refresh={refreshDashboard} onRefresh={handleUpdate} />
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
