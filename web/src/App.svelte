<script lang="ts">
  import { onMount } from "svelte";
  import Dashboard from "./lib/Dashboard.svelte";
  import LoginPage from "./lib/LoginPage.svelte";
  import ToastContainer from "./lib/ToastContainer.svelte";
  import {
    listGroups,
    createGroup,
    switchGroup,
    updateGroup,
    deleteGroup,
  } from "./lib/api";
  import type { Group } from "./lib/types";
  import { auth, type AuthUser } from "./lib/stores/auth";
  import { toast } from "./lib/stores/toast";

  let user = $state<AuthUser | null>(null);
  let authLoading = $state(true);

  let refreshKey = $state(0);
  let groups = $state<Group[]>([]);
  let currentGroupId = $state(0);
  let showCreateGroup = $state(false);
  let newGroupName = $state("");
  let isLoading = $state(true);

  // Subscribe to auth stores
  $effect(() => {
    const unsubUser = auth.user.subscribe((v) => (user = v));
    const unsubLoading = auth.isLoading.subscribe((v) => (authLoading = v));
    return () => {
      unsubUser();
      unsubLoading();
    };
  });

  async function fetchGroups() {
    try {
      const fetchedGroups = await listGroups();
      groups = fetchedGroups;
      if (fetchedGroups.length > 0 && currentGroupId === 0) {
        currentGroupId = fetchedGroups[0].id;
        // Sync with backend to ensure current_group_id is valid
        await switchGroup(fetchedGroups[0].id);
      }
    } catch (error) {
      toast.error(error instanceof Error ? error.message : "Failed to fetch groups");
    } finally {
      isLoading = false;
    }
  }

  function handleUpdate() {
    refreshKey += 1;
  }

  async function handleSwitchGroup(groupId: number) {
    const previousGroupId = currentGroupId;
    currentGroupId = groupId;
    handleUpdate();

    try {
      await switchGroup(groupId);
    } catch (error) {
      currentGroupId = previousGroupId;
      handleUpdate();
      toast.error(error instanceof Error ? error.message : "Failed to switch group");
    }
  }

  async function handleCreateGroup(e: SubmitEvent) {
    e.preventDefault();
    if (!newGroupName.trim()) return;

    const savedName = newGroupName;
    const wasFirstGroup = groups.length === 0;

    // Only do optimistic update if not the first group
    // (to avoid rendering Dashboard before backend state is ready)
    let tempGroup: Group | null = null;
    if (!wasFirstGroup) {
      tempGroup = {
        id: Date.now(),
        name: savedName,
        members: [],
        expenses: [],
        simplify_debts: false,
      };
      groups = [...groups, tempGroup];
    }

    newGroupName = "";
    showCreateGroup = false;

    try {
      const newGroup = await createGroup(savedName);
      // Switch group FIRST to ensure backend current_group_id is set
      await switchGroup(newGroup.id);
      currentGroupId = newGroup.id;
      // Now fetch groups - Dashboard will have correct state
      await fetchGroups();
      toast.success(`Group "${newGroup.name}" created successfully`);
    } catch (error) {
      if (tempGroup) {
        groups = groups.filter((g) => g.id !== tempGroup!.id);
      }
      showCreateGroup = true;
      newGroupName = savedName;
      toast.error(error instanceof Error ? error.message : "Failed to create group");
    }
  }

  async function handleUpdateGroup(groupId: number, newName: string) {
    const previousGroups = [...groups];
    groups = groups.map((g) => (g.id === groupId ? { ...g, name: newName } : g));

    try {
      await updateGroup(groupId, newName);
      await fetchGroups();
      handleUpdate();
      toast.success("Group name updated successfully");
    } catch (error) {
      groups = previousGroups;
      toast.error(error instanceof Error ? error.message : "Failed to update group");
      throw error;
    }
  }

  async function handleDeleteGroup(groupId: number) {
    const group = groups.find((g) => g.id === groupId);
    if (!group) return;

    toast.confirm(
      `Are you sure you want to delete "${group.name}"? This will permanently delete all users and expenses in this group.`,
      async () => {
        const previousGroups = [...groups];
        const previousGroupId = currentGroupId;
        groups = groups.filter((g) => g.id !== groupId);

        try {
          const response = await deleteGroup(groupId);

          if (response.switched_group !== undefined) {
            currentGroupId = response.switched_group;
          }

          await fetchGroups();
          handleUpdate();
          toast.success(`Group "${group.name}" deleted successfully`);
        } catch (error) {
          groups = previousGroups;
          currentGroupId = previousGroupId;
          toast.error(error instanceof Error ? error.message : "Failed to delete group");
        }
      }
    );
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      e.preventDefault();
      handleCreateGroup(e as unknown as SubmitEvent);
    }
    if (e.key === "Escape") {
      showCreateGroup = false;
      newGroupName = "";
    }
  }

  function cancelCreateGroup() {
    showCreateGroup = false;
    newGroupName = "";
  }

  function handleGroupChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    handleSwitchGroup(Number(target.value));
  }

  onMount(() => {
    auth.init();
  });

  // Fetch groups when user changes
  $effect(() => {
    if (user) {
      fetchGroups();
    }
  });
</script>

<ToastContainer />

{#if authLoading || (user && isLoading)}
  <div class="welcome-page">
    <div class="loading-state">
      <div class="loading-spinner"></div>
      <p>Loading...</p>
    </div>
  </div>
{:else if !user}
  <LoginPage />
{:else}
  <header class="app-header">
    <div class="container">
      <div class="header-content">
        <div>
          <h1 class="app-title">Splitdumb</h1>
        </div>
        <div class="header-actions">
          {#if showCreateGroup}
            <!-- svelte-ignore a11y_autofocus -->
            <input
              type="text"
              class="form-control header-input"
              placeholder="New group name..."
              bind:value={newGroupName}
              onkeydown={handleKeyDown}
              autofocus
            />
            <button class="btn btn-sm btn-success" onclick={handleCreateGroup}>
              Create
            </button>
            <button class="btn btn-sm btn-secondary" onclick={cancelCreateGroup}>
              Cancel
            </button>
          {:else if groups.length > 0}
            <select
              class="form-select header-select"
              value={currentGroupId}
              onchange={handleGroupChange}
            >
              {#each groups as group (group.id)}
                <option value={group.id}>{group.name}</option>
              {/each}
            </select>
            <button class="btn btn-sm btn-primary" onclick={() => (showCreateGroup = true)}>
              + New
            </button>
          {/if}
          <span class="user-name">{user.name}</span>
          <button class="btn btn-sm btn-secondary" onclick={auth.logout} title="Logout">
            Logout
          </button>
        </div>
      </div>
    </div>
  </header>

  <div class="app-container">
    {#if groups.length === 0}
      <div class="empty-state-card">
        <h2>Welcome to Splitdumb</h2>
        <p>Create your first group to start splitting expenses with friends.</p>
        <button class="btn btn-primary" onclick={() => (showCreateGroup = true)}>
          Create Your First Group
        </button>
      </div>
    {:else}
      <Dashboard
        {refreshKey}
        onRefresh={handleUpdate}
        onUpdateGroup={handleUpdateGroup}
        onDeleteGroup={() => handleDeleteGroup(currentGroupId)}
      />
    {/if}
  </div>
{/if}
