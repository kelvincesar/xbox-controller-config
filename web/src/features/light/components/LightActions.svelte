<script lang="ts">
  import { TextMorph } from 'torph/svelte';

  import { rise } from '@/lib/motion';

  import { useLight } from '../hooks/useLight.svelte';

  const light = useLight();

  let isSent = $derived(light.canSend && !light.hasUnsentChanges);
</script>

<div class="actions">
  <button
    class="primary"
    onclick={light.handleSend}
    disabled={!light.canSend || !light.hasUnsentChanges}
    aria-label={isSent ? 'Sent' : 'Send to controller'}
  >
    <TextMorph text={isSent ? 'Sent' : 'Send to controller'} duration={350} />
  </button>
  <button onclick={light.handleTurnOff} disabled={!light.canSend}>
    Turn light off
  </button>
</div>
{#if light.error}
  <p class="error" role="alert" in:rise>{light.error}</p>
{/if}

<style>
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }
</style>
