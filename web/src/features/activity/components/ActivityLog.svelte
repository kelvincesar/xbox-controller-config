<script lang="ts">
  import { useActivityStore } from '../store/useActivityStore.svelte';

  const activity = useActivityStore();

  const DIRECTION_LABEL = { sent: 'sent', received: 'received', info: '' };
</script>

<section class="activity" aria-labelledby="activity-title">
  <details>
    <summary id="activity-title">Activity ({activity.entries.length})</summary>
    {#if activity.entries.length === 0}
      <p class="empty">
        Connect a controller to see the packets sent to it and what it sends
        back.
      </p>
    {:else}
      <ol>
        {#each activity.entries as entry (entry.id)}
          <li class={entry.direction}>
            <time>{entry.time}</time>
            <span class="direction">{DIRECTION_LABEL[entry.direction]}</span>
            <span class="text">{entry.text}</span>
          </li>
        {/each}
      </ol>
    {/if}
  </details>
</section>

<style>
  summary {
    cursor: pointer;
    font-weight: 700;
  }

  .empty {
    margin: 0.75rem 0 0;
    color: var(--ink-soft);
    font-size: 0.925rem;
  }

  ol {
    list-style: none;
    margin: 1rem 0 0;
    padding: 0.75rem 1rem;
    max-height: 18rem;
    overflow: auto;
    background: var(--well);
    border-radius: 0.5rem;
    font-family: ui-monospace, 'SF Mono', 'Cascadia Mono', Menlo, monospace;
    font-size: 0.8rem;
    line-height: 1.7;
  }

  li {
    display: grid;
    grid-template-columns: auto 4.5rem minmax(0, 1fr);
    gap: 0.75rem;
  }

  time,
  .direction {
    color: var(--ink-soft);
  }

  .sent .direction {
    color: var(--accent);
  }

  .text {
    overflow-wrap: anywhere;
  }
</style>
