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
    <h1>Guide light</h1>
    <p class="lede">
      Dim, blink or switch off the glowing Xbox button on your controller.
      Everything runs in your browser; nothing is sent to a server.
    </p>
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
