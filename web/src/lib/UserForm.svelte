<script lang="ts">
  import { createUser } from "./api";
  import { toast } from "./stores/toast";
  import { formatPhoneNumber, isValidPhone } from "./utils/phone";

  interface Props {
    onUserAdded: () => void;
  }

  let { onUserAdded }: Props = $props();

  let phone = $state("");
  let isSubmitting = $state(false);

  let isValid = $derived(isValidPhone(phone));

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    phone = formatPhoneNumber(target.value);
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!isValid || isSubmitting) return;

    isSubmitting = true;
    try {
      const user = await createUser(phone);
      onUserAdded();
      toast.success(`User "${user.name}" added successfully`);
      phone = "";
    } catch (err) {
      toast.error(err instanceof Error ? err.message : "Failed to add user");
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="card form-card">
  <div class="card-body">
    <h5 class="card-title">Add Member</h5>
    <form onsubmit={handleSubmit}>
      <div class="form-group">
        <label for="userPhone" class="form-label">Phone Number</label>
        <input
          id="userPhone"
          type="tel"
          class="form-control"
          placeholder="XXX-XXX-XXXX"
          value={phone}
          oninput={handleInput}
          disabled={isSubmitting}
          required
        />
      </div>
      <button
        type="submit"
        class="btn btn-primary w-100"
        disabled={isSubmitting || !isValid}
      >
        {isSubmitting ? "Adding..." : "Add Member"}
      </button>
    </form>
  </div>
</div>
