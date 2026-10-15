<script lang="ts">
  import { rise } from '@/lib/motion';

  import { useConnection } from '../hooks/useConnection.svelte';
  import ConnectButtons from './ConnectButtons.svelte';
  import LinkStatus from './LinkStatus.svelte';

  const connection = useConnection();
</script>

<section aria-labelledby="connection-title">
  <h2 id="connection-title">Controller</h2>
  {#if connection.link}
    <div in:rise>
      <LinkStatus />
    </div>
  {:else}
    <div in:rise>
      <ConnectButtons />
    </div>
  {/if}
  {#if connection.error}
    <p class="error" role="alert" in:rise>{connection.error}</p>
  {/if}
</section>
