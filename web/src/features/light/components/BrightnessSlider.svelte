<script lang="ts">
  import { TextMorph } from 'torph/svelte';

  import { useLightOptions } from '../hooks/useLightOptions';
  import { useLightStore } from '../store/useLightStore.svelte';

  const light = useLightStore();
  const { offMode, maxBrightness } = useLightOptions();

  const MIN = 1;

  let isOff = $derived(light.mode === offMode);
  let fill = $derived(((light.brightness - MIN) / (maxBrightness - MIN)) * 100);
</script>

<label class="slider" class:muted={isOff}>
  <span class="heading">
    Brightness
    <output><TextMorph text={String(light.brightness)} duration={320} /></output
    >
  </span>
  <input
    type="range"
    min={MIN}
    max={maxBrightness}
    bind:value={light.brightness}
    disabled={isOff}
    style:--fill="{fill}%"
    aria-valuetext="{light.brightness} of {maxBrightness}"
  />
</label>

<style>
  .slider {
    display: grid;
    gap: 0.5rem;
    transition: opacity 200ms ease;
  }

  .heading {
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

  /* Custom track: filled up to the thumb, so the level reads at a glance. */
  input {
    --track: 0.375rem;
    --thumb: 1.25rem;
    width: 100%;
    height: 2.75rem;
    margin: 0;
    background: transparent;
    cursor: pointer;
    appearance: none;
    -webkit-tap-highlight-color: transparent;
  }

  input:disabled {
    cursor: not-allowed;
  }

  input::-webkit-slider-runnable-track {
    height: var(--track);
    border-radius: 999px;
    background: linear-gradient(
      to right,
      var(--accent) var(--fill),
      var(--well) var(--fill)
    );
  }

  input::-moz-range-track {
    height: var(--track);
    border-radius: 999px;
    background: var(--well);
  }

  input::-moz-range-progress {
    height: var(--track);
    border-radius: 999px;
    background: var(--accent);
  }

  input::-webkit-slider-thumb {
    appearance: none;
    width: var(--thumb);
    height: var(--thumb);
    margin-top: calc((var(--track) - var(--thumb)) / 2);
    border: 0;
    border-radius: 50%;
    background: #fff;
    box-shadow:
      0 0 0 1px rgb(0 0 0 / 0.12),
      0 1px 3px rgb(0 0 0 / 0.25);
    transition: transform 160ms var(--ease-out);
  }

  input::-moz-range-thumb {
    width: var(--thumb);
    height: var(--thumb);
    border: 0;
    border-radius: 50%;
    background: #fff;
    box-shadow:
      0 0 0 1px rgb(0 0 0 / 0.12),
      0 1px 3px rgb(0 0 0 / 0.25);
    transition: transform 160ms var(--ease-out);
  }

  /* The thumb swells while held, like picking it up. */
  input:active:not(:disabled)::-webkit-slider-thumb {
    transform: scale(1.2);
  }

  input:active:not(:disabled)::-moz-range-thumb {
    transform: scale(1.2);
  }

  input:focus-visible {
    outline: none;
  }

  input:focus-visible::-webkit-slider-thumb {
    box-shadow:
      0 0 0 2px var(--shell),
      0 0 0 4px var(--accent);
  }

  input:focus-visible::-moz-range-thumb {
    box-shadow:
      0 0 0 2px var(--shell),
      0 0 0 4px var(--accent);
  }

  @media (prefers-reduced-motion: reduce) {
    .slider,
    input::-webkit-slider-thumb,
    input::-moz-range-thumb {
      transition: none;
    }
  }
</style>
