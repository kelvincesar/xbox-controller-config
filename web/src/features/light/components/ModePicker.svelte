<script lang="ts">
  import { useLightOptions } from '../hooks/useLightOptions';
  import { useLightStore } from '../store/useLightStore.svelte';

  const light = useLightStore();
  const { modes } = useLightOptions();
</script>

<div class="modes" role="radiogroup" aria-label="Light pattern">
  {#each modes as option (option.code)}
    <label class="mode" class:selected={light.mode === option.code}>
      <input
        type="radio"
        name="mode"
        value={option.code}
        bind:group={light.mode}
      />
      {option.label}
    </label>
  {/each}
</div>

<style>
  .modes {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .mode {
    position: relative;
    display: inline-flex;
    align-items: center;
    min-height: 2.75rem;
    padding: 0.5rem 0.95rem;
    border: 1px solid var(--line);
    border-radius: 999px;
    cursor: pointer;
    background: var(--panel);
    -webkit-tap-highlight-color: transparent;
    transition:
      border-color 150ms ease,
      background-color 150ms ease,
      color 150ms ease,
      transform 160ms var(--ease-out);
  }

  .mode:active {
    transform: scale(0.96);
  }

  @media (hover: hover) and (pointer: fine) {
    .mode:hover:not(.selected) {
      border-color: var(--ink-soft);
    }
  }

  .mode input {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .mode.selected {
    border-color: var(--ink);
    background: var(--ink);
    color: var(--shell);
  }

  @media (prefers-reduced-motion: reduce) {
    .mode {
      transition: none;
    }
  }

  .mode:has(input:focus-visible) {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
