<script lang="ts">
  import type { Settlement } from "./types";

  interface Props {
    settlements: Settlement[];
    onSettle: (from: string, to: string, amount: number) => Promise<void>;
    isSimplified: boolean;
    onToggleSimplify: () => void;
  }

  let { settlements, onSettle, isSimplified, onToggleSimplify }: Props = $props();
</script>

<div class="card-body settlement-card">
  <div class="settlement-header">
    <h5 class="card-title">Who Pays Whom</h5>
    <button
      class="btn btn-sm {isSimplified ? 'btn-primary' : 'btn-outline'}"
      onclick={onToggleSimplify}
      title={isSimplified
        ? "Showing simplified debts (fewer transactions)"
        : "Showing pairwise debts (stable)"}
    >
      {isSimplified ? "Simplified" : "Simplify"}
    </button>
  </div>
  {#if settlements.length > 0}
    <div>
      {#each settlements as settlement, index (index)}
        <div class="settlement-item {settlement.settled ? 'settlement-item-settled' : ''}">
          <div>
            <strong>{settlement.from}</strong> pays <strong>{settlement.to}</strong>
          </div>
          <div class="flex-center gap-sm">
            <span class="settlement-amount">
              ${settlement.amount.toFixed(2)}
            </span>
            {#if settlement.settled}
              <span class="badge badge-settled">Settled</span>
            {:else}
              <button
                class="btn btn-sm btn-success"
                onclick={() => onSettle(settlement.from, settlement.to, settlement.amount)}
              >
                Settle
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <div class="empty-state-icon">✓</div>
      <p>All settled up!</p>
    </div>
  {/if}
</div>

<style>
  .btn-outline {
    background: transparent;
    border: 1px solid var(--color-border);
    color: var(--color-text);
  }
</style>
