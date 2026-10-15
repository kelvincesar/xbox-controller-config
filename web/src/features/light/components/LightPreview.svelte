<script lang="ts">
  import { useLightOptions } from '../hooks/useLightOptions';
  import { previewLevel } from '../logic/light';
  import { useLightStore } from '../store/useLightStore.svelte';

  const light = useLightStore();
  const { modes, offMode, maxBrightness } = useLightOptions();

  let option = $derived(modes.find(m => m.code === light.mode));
  let level = $derived(
    previewLevel(
      { mode: light.mode, brightness: light.brightness },
      maxBrightness,
      offMode,
    ),
  );
  let caption = $derived(
    light.mode === offMode
      ? 'Off'
      : `${option?.label ?? ''}, brightness ${light.brightness} of ${maxBrightness}`,
  );
</script>

<figure class="guide" aria-label="Preview: {caption}">
  <div class="button" style:--level={level}>
    <div class="light {option?.key ?? 'steady'}">
      <svg viewBox="0 0 100 100" aria-hidden="true">
        <circle class="ring" cx="50" cy="50" r="34" />
        <circle class="core" cx="50" cy="50" r="14" />
      </svg>
    </div>
  </div>
  <figcaption>{caption}</figcaption>
</figure>

<style>
  .guide {
    margin: 0;
    display: grid;
    justify-items: center;
    gap: 1.25rem;
  }

  .button {
    --lit: calc(0.06 + var(--level) * 0.94);
    width: min(15rem, 62vw);
    aspect-ratio: 1;
    border-radius: 50%;
    background: radial-gradient(circle at 38% 30%, #3a3e44, #1f2125 70%);
    box-shadow:
      inset 0 -0.4rem 0.8rem rgb(0 0 0 / 0.45),
      inset 0 0.3rem 0.6rem rgb(255 255 255 / 0.06),
      0 0.15rem 0 var(--line),
      0 0 0 0.55rem var(--bezel);
    display: grid;
    place-items: center;
  }

  .light {
    width: 72%;
    aspect-ratio: 1;
    filter: drop-shadow(
      0 0 calc(var(--level) * 1.1rem)
        rgb(225 236 255 / calc(var(--level) * 0.9))
    );
  }

  svg {
    width: 100%;
    height: 100%;
    display: block;
  }

  .ring {
    fill: none;
    stroke: #f4f8ff;
    stroke-width: 7;
    opacity: var(--lit);
  }

  .core {
    fill: #f4f8ff;
    opacity: var(--lit);
  }

  .light,
  .ring,
  .core {
    transition:
      opacity 180ms ease-out,
      filter 180ms ease-out;
  }

  /* Approximations of the controller's patterns. */
  .blink-fast {
    animation: blink 0.3s steps(1, end) infinite;
  }
  .blink {
    animation: blink 0.7s steps(1, end) infinite;
  }
  .blink-slow {
    animation: blink 1.4s steps(1, end) infinite;
  }
  .fade-fast {
    animation: fade 1.2s ease-in-out infinite alternate;
  }
  .fade-slow {
    animation: fade 3s ease-in-out infinite alternate;
  }

  @keyframes blink {
    50% {
      opacity: 0.08;
    }
  }

  @keyframes fade {
    from {
      opacity: 1;
    }
    to {
      opacity: 0.08;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .light {
      animation: none;
    }
  }

  figcaption {
    font-size: 0.95rem;
    color: var(--ink-soft);
  }
</style>
