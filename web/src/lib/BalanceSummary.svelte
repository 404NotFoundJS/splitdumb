<script lang="ts">
  interface Props {
    balances: Record<string, number>;
  }

  let { balances }: Props = $props();

  let sortedBalances = $derived(
    Object.entries(balances).sort(([a], [b]) => a.localeCompare(b))
  );
</script>

<div class="card dashboard-card mt-3">
  <div class="card-body">
    <h5 class="card-title">Balance Summary</h5>
    <div class="list-group">
      {#each sortedBalances as [user, balance] (user)}
        <div class="balance-item list-group-item">
          <span class="balance-name">{user}</span>
          <span class="balance-amount {balance >= 0 ? 'text-success' : 'text-danger'}">
            {balance >= 0 ? "+" : ""}${balance.toFixed(2)}
          </span>
        </div>
      {/each}
    </div>
  </div>
</div>
