<script lang="ts">
  import ActivityLog from '@/features/activity/components/ActivityLog.svelte';
  import ConnectionPanel from '@/features/connection/components/ConnectionPanel.svelte';
  import SupportBanner from '@/features/connection/components/SupportBanner.svelte';
  import Troubleshooting from '@/features/help/components/Troubleshooting.svelte';
  import LightPanel from '@/features/light/components/LightPanel.svelte';
  import LightPreview from '@/features/light/components/LightPreview.svelte';
</script>

<main>
  <header>
    <div>
      <h1>Guide light</h1>
      <p class="lede">
        Dim, blink or switch off the glowing Xbox button on your controller.
        Everything runs in your browser; nothing is sent to a server.
      </p>
    </div>
    <a
      class="source"
      href="https://github.com/kelvincesar/xbox-controller-config"
      rel="noreferrer"
      target="_blank"
      aria-label="Source code on GitHub"
    >
      <svg viewBox="0 0 16 16" aria-hidden="true">
        <path
          d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
        />
      </svg>
      <span>GitHub</span>
    </a>
  </header>

  <SupportBanner />

  <div class="stage">
    <div class="preview">
      <LightPreview />
    </div>
    <div class="controls">
      <ConnectionPanel />
      <LightPanel />
    </div>
  </div>

  <Troubleshooting />
  <ActivityLog />

  <footer>
    <p>
      Packet formats follow the open-source <a
        href="https://github.com/libsdl-org/SDL"
        rel="noreferrer">SDL</a
      >
      and <a href="https://github.com/medusalix/xone" rel="noreferrer">xone</a> drivers.
      Encoding runs in Rust compiled to WebAssembly. Not affiliated with Microsoft.
    </p>
  </footer>
</main>

<style>
  main {
    max-width: 62rem;
    margin: 0 auto;
    padding-block: clamp(2rem, 6vw, 4.5rem) 3rem;
    display: grid;
    gap: clamp(2.5rem, 6vw, 4rem);
  }

  header {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: start;
    gap: 1rem;
  }

  .source {
    display: inline-flex;
    align-items: center;
    gap: 0.45rem;
    padding: 0.5rem 0.8rem;
    border: 1px solid var(--line);
    border-radius: 999px;
    color: var(--ink);
    font-weight: 600;
    font-size: 0.925rem;
    text-decoration: none;
    -webkit-tap-highlight-color: transparent;
    transition:
      border-color 150ms ease,
      background-color 150ms ease,
      transform 160ms var(--ease-out);
  }

  .source:active {
    transform: scale(0.96);
  }

  @media (hover: hover) and (pointer: fine) {
    .source:hover {
      border-color: var(--ink);
      background: var(--panel);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .source {
      transition: none;
    }
  }

  .source svg {
    width: 1.15rem;
    height: 1.15rem;
    fill: currentColor;
  }

  @media (max-width: 30rem) {
    .source {
      padding: 0.55rem;
    }
    .source span {
      display: none;
    }
  }

  h1 {
    margin: 0;
    font-size: clamp(2.75rem, 9vw, 5.25rem);
    font-stretch: 125%;
    font-weight: 800;
    line-height: 0.95;
    letter-spacing: -0.02em;
  }

  .lede {
    margin: 1rem 0 0;
    max-width: 46ch;
    font-size: 1.125rem;
    color: var(--ink-soft);
  }

  .stage {
    display: grid;
    grid-template-columns: minmax(0, 0.9fr) minmax(0, 1.1fr);
    gap: clamp(2rem, 5vw, 4rem);
    align-items: start;
  }

  .preview {
    position: sticky;
    top: calc(env(safe-area-inset-top, 0px) + 2rem);
    padding-top: 1rem;
  }

  .controls {
    display: grid;
    gap: 2rem;
  }

  @media (max-width: 46rem) {
    .stage {
      grid-template-columns: minmax(0, 1fr);
    }
    .preview {
      position: static;
    }
  }

  footer {
    padding-top: 1.5rem;
    border-top: 1px solid var(--line);
    font-size: 0.875rem;
    color: var(--ink-soft);
  }

  footer p {
    margin: 0;
    max-width: 70ch;
  }
</style>
