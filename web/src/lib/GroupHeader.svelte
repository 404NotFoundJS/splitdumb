<script lang="ts">
  interface Props {
    groupName: string;
    groupId: number;
    onUpdateGroup: (groupId: number, newName: string) => Promise<void>;
    onDeleteGroup: () => void;
  }

  let { groupName, groupId, onUpdateGroup, onDeleteGroup }: Props = $props();

  let isEditing = $state(false);
  let editedName = $state("");

  function startEdit() {
    editedName = groupName;
    isEditing = true;
  }

  function cancelEdit() {
    isEditing = false;
    editedName = "";
  }

  async function saveEdit() {
    if (!editedName.trim()) return;

    try {
      await onUpdateGroup(groupId, editedName);
      isEditing = false;
      editedName = "";
    } catch {
      editedName = groupName;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter") saveEdit();
    if (e.key === "Escape") cancelEdit();
  }
</script>

<div class="dashboard-group-header">
  {#if isEditing}
    <div class="edit-form-row">
      <!-- svelte-ignore a11y_autofocus -->
      <input
        type="text"
        class="form-control edit-form-input"
        bind:value={editedName}
        onkeydown={handleKeyDown}
        autofocus
      />
      <button class="btn btn-sm btn-success" onclick={saveEdit}>Save</button>
      <button class="btn btn-sm btn-secondary" onclick={cancelEdit}>Cancel</button>
    </div>
  {:else}
    <div class="flex-between">
      <h3 class="dashboard-group-name">{groupName}</h3>
      <div class="flex-center gap-sm">
        <button
          class="btn btn-sm btn-secondary"
          onclick={startEdit}
          title="Edit group name"
        >
          Edit
        </button>
        <button
          class="btn btn-sm btn-danger"
          onclick={onDeleteGroup}
          title="Delete group"
        >
          Delete
        </button>
      </div>
    </div>
  {/if}
</div>
