<script lang="ts">
  import { useBrowserSupport } from '../hooks/useBrowserSupport';

  const support = useBrowserSupport();
</script>

{#if support.problem === 'insecure'}
  <p class="banner" role="alert">
    Open this page over HTTPS (or on localhost). Browsers only allow USB and
    Bluetooth access from secure pages.
  </p>
{:else if support.problem === 'unsupported'}
  <p class="banner" role="alert">
    This browser can’t talk to USB or Bluetooth devices. Open the page in
    Chrome, Edge or Opera on a computer.
  </p>
{/if}

<style>
  .banner {
    margin: 0;
    padding: 1rem 1.25rem;
    border-left: 4px solid var(--danger);
    background: var(--panel);
  }
</style>
