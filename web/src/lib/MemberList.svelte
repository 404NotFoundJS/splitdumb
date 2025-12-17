<script lang="ts">
  import type { User } from "./types";

  interface Props {
    members: User[];
    onDeleteUser: (id: number, name: string) => void;
  }

  let { members, onDeleteUser }: Props = $props();
</script>

<div class="card dashboard-card mt-3">
  <div class="card-body">
    <h5 class="card-title">Group Members</h5>
    {#if members.length > 0}
      <div class="list-group">
        {#each members as user (user.id)}
          <div class="list-group-item member-item">
            <span class="balance-name">{user.name}</span>
            <button
              class="btn btn-sm btn-danger"
              onclick={() => onDeleteUser(user.id, user.name)}
              title="Remove user from group"
            >
              Remove
            </button>
          </div>
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-state-icon">👤</div>
        <p>No members yet. Add someone to get started!</p>
      </div>
    {/if}
  </div>
</div>
