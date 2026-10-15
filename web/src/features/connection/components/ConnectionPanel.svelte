<script lang="ts">
  import { useConnection } from '../hooks/useConnection.svelte';
  import ConnectButtons from './ConnectButtons.svelte';
  import LinkStatus from './LinkStatus.svelte';

  const connection = useConnection();
</script>

<section aria-labelledby="connection-title">
  <h2 id="connection-title">Controller</h2>
  {#if connection.link}
    <LinkStatus />
  {:else}
    <ConnectButtons />
  {/if}
  {#if connection.error}
    <p class="error" role="alert">{connection.error}</p>
  {/if}
</section>
