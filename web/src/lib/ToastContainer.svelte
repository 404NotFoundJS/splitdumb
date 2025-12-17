<script lang="ts">
  import { toast, type Toast } from "./stores/toast";

  let toasts = $state<Toast[]>([]);

  $effect(() => {
    const unsub = toast.subscribe((v) => (toasts = v));
    return unsub;
  });
</script>

{#if toasts.length > 0}
  <div class="toast-wrapper">
    {#each toasts as t (t.id)}
      <div
        class="toast-item"
        class:toast-confirm={t.isConfirm}
        class:toast-success={!t.isConfirm && t.type === "success"}
        class:toast-error={!t.isConfirm && t.type === "error"}
        class:toast-info={!t.isConfirm && t.type === "info"}
        class:toast-warning={!t.isConfirm && t.type === "warning"}
      >
        <div class="toast-message">{t.message}</div>
        {#if t.isConfirm}
          <div class="toast-buttons">
            <button class="btn-confirm" onclick={() => toast.handleConfirm(t.id)}>
              Confirm
            </button>
            <button class="btn-cancel" onclick={() => toast.handleCancel(t.id)}>
              Cancel
            </button>
          </div>
        {:else}
          <button class="btn-close" onclick={() => toast.removeToast(t.id)} aria-label="Close">
            ×
          </button>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<style>
  .toast-wrapper {
    position: fixed;
    top: 80px;
    right: 20px;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    gap: 12px;
    max-width: 420px;
    pointer-events: all;
  }

  @media (max-width: 768px) {
    .toast-wrapper {
      left: 20px;
    }
  }

  .toast-item {
    background: white;
    padding: 16px;
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    min-width: 320px;
    animation: slideIn 0.3s ease-out;
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 12px;
  }

  .toast-confirm {
    border-left: 4px solid #f59e0b;
    flex-direction: column;
  }

  .toast-success {
    border-left: 4px solid #10b981;
  }

  .toast-error {
    border-left: 4px solid #ef4444;
  }

  .toast-info {
    border-left: 4px solid #3b82f6;
  }

  .toast-warning {
    border-left: 4px solid #f59e0b;
  }

  .toast-message {
    font-size: 14px;
    line-height: 1.5;
    color: #1e293b;
    flex: 1;
  }

  .toast-buttons {
    display: flex;
    gap: 8px;
    width: 100%;
  }

  .btn-confirm,
  .btn-cancel {
    flex: 1;
    padding: 10px;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 14px;
  }

  .btn-confirm {
    background: #ef4444;
  }

  .btn-cancel {
    background: #6b7280;
  }

  .btn-close {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    color: #6b7280;
  }

  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
</style>
