<script lang="ts">
  import { useLightOptions } from '../hooks/useLightOptions';
  import { useLightStore } from '../store/useLightStore.svelte';

  const light = useLightStore();
  const { offMode, maxBrightness } = useLightOptions();

  let isOff = $derived(light.mode === offMode);
</script>

<label class="slider" class:muted={isOff}>
  <span>Brightness <output>{light.brightness}</output></span>
  <input
    type="range"
    min="1"
    max={maxBrightness}
    bind:value={light.brightness}
    disabled={isOff}
    aria-valuetext="{light.brightness} of {maxBrightness}"
  />
</label>

<style>
  .slider {
    display: grid;
    gap: 0.5rem;
  }

  span {
    display: flex;
    justify-content: space-between;
    font-weight: 600;
  }

  output {
    font-variant-numeric: tabular-nums;
    font-weight: 500;
    color: var(--ink-soft);
  }

  .muted {
    opacity: 0.45;
  }

  input {
    width: 100%;
    height: 2rem;
    accent-color: var(--accent);
  }
</style>
