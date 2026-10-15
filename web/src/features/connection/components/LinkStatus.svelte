<script lang="ts">
  import { useConnection } from '../hooks/useConnection.svelte';
  import { useRumble } from '../hooks/useRumble.svelte';
  import { batteryLabel } from '../logic/battery';

  const connection = useConnection();
  const { testRumble } = useRumble();

  let details = $derived(
    [
      connection.link?.kind === 'usb' ? 'on USB' : 'on Bluetooth',
      batteryLabel(connection.battery),
    ]
      .filter(Boolean)
      .join(', '),
  );
</script>

{#if connection.link}
  <div class="linked">
    <p><strong>{connection.link.name}</strong> {details}</p>
    <button
      class="quiet"
      onclick={connection.disconnect}
      disabled={connection.isBusy}
    >
      Disconnect
    </button>
  </div>

  {#if connection.link.kind === 'bluetooth'}
    <p class="hint">
      Over Bluetooth the controller ignores light commands. Plug it in with a
      USB cable to change the light.
    </p>
    <button class="quiet rumble" onclick={testRumble}>Test rumble</button>
  {/if}
{/if}

<style>
  .linked {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
    padding: 0.75rem 0.75rem 0.75rem 1rem;
    border-radius: 0.75rem;
    background: var(--panel);
  }

  .linked p {
    margin: 0;
  }

  .rumble {
    margin-top: 0.75rem;
  }
</style>
