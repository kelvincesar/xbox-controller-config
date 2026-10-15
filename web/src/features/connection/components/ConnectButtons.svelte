<script lang="ts">
  import { useBrowserSupport } from '../hooks/useBrowserSupport';
  import { useConnection } from '../hooks/useConnection.svelte';

  const support = useBrowserSupport();
  const connection = useConnection();
</script>

<div class="buttons">
  <button
    onclick={connection.connectUsb}
    disabled={!support.canUseUsb || connection.isBusy}
  >
    Connect with USB cable
  </button>
  <button
    class="quiet"
    onclick={connection.connectBluetooth}
    disabled={!support.canUseBluetooth || connection.isBusy}
  >
    Connect over Bluetooth
  </button>
</div>
<p class="hint">
  Light changes need the cable. Bluetooth shows battery level and can test
  rumble.
</p>

<style>
  .buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }
</style>
