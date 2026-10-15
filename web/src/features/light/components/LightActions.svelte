<script lang="ts">
  import { useLight } from '../hooks/useLight.svelte';

  const light = useLight();
</script>

<div class="actions">
  <button
    class="primary"
    onclick={light.handleSend}
    disabled={!light.canSend || !light.hasUnsentChanges}
  >
    {light.canSend && !light.hasUnsentChanges ? 'Sent' : 'Send to controller'}
  </button>
  <button onclick={light.handleTurnOff} disabled={!light.canSend}>
    Turn light off
  </button>
</div>
{#if light.error}
  <p class="error" role="alert">{light.error}</p>
{/if}

<style>
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }
</style>
