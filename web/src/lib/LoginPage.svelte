<script lang="ts">
  import { auth } from "./stores/auth";
  import { toast } from "./stores/toast";
  import { formatPhoneNumber, isValidPhone } from "./utils/phone";

  type Mode = "signin" | "signup";

  let mode = $state<Mode>("signin");
  let phone = $state("");
  let name = $state("");
  let isSubmitting = $state(false);

  let validPhone = $derived(isValidPhone(phone));

  function handlePhoneInput(e: Event) {
    const target = e.target as HTMLInputElement;
    phone = formatPhoneNumber(target.value);
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!validPhone) return;
    if (mode === "signup" && !name.trim()) return;

    isSubmitting = true;
    try {
      if (mode === "signup") {
        await auth.register(phone.trim(), name.trim());
      } else {
        await auth.login(phone.trim());
      }
    } catch (error) {
      toast.error(
        error instanceof Error
          ? error.message
          : mode === "signup"
            ? "Registration failed. Please try again."
            : "Sign in failed. Please try again."
      );
    } finally {
      isSubmitting = false;
    }
  }

  function switchMode() {
    mode = mode === "signin" ? "signup" : "signin";
    name = "";
  }
</script>

<div class="welcome-page">
  <div class="welcome-card">
    <h1 class="welcome-title">Splitdumb</h1>
    <p class="welcome-subtitle">Split expenses with friends, the simple way.</p>
    <form onsubmit={handleSubmit} class="welcome-form">
      <!-- svelte-ignore a11y_autofocus -->
      <input
        type="tel"
        class="form-control welcome-input"
        placeholder="Phone number (XXX-XXX-XXXX)"
        value={phone}
        oninput={handlePhoneInput}
        autofocus
        disabled={isSubmitting}
      />
      {#if mode === "signup"}
        <input
          type="text"
          class="form-control welcome-input"
          placeholder="Your name"
          bind:value={name}
          disabled={isSubmitting}
        />
      {/if}
      <button
        type="submit"
        class="btn btn-primary welcome-button"
        disabled={isSubmitting || !validPhone || (mode === "signup" && !name.trim())}
      >
        {#if isSubmitting}
          {mode === "signup" ? "Creating account..." : "Signing in..."}
        {:else}
          {mode === "signup" ? "Sign Up" : "Sign In"}
        {/if}
      </button>
    </form>
    <p class="auth-switch">
      {#if mode === "signin"}
        Don't have an account?
        <button type="button" class="link-button" onclick={switchMode}>
          Sign Up
        </button>
      {:else}
        Already have an account?
        <button type="button" class="link-button" onclick={switchMode}>
          Sign In
        </button>
      {/if}
    </p>
  </div>
</div>
