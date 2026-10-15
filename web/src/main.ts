import { mount } from 'svelte';

import '@fontsource-variable/archivo/wdth.css';

import { errorMessage } from '@/lib/errors';
import { loadProtocol } from '@/lib/protocol';

import './app.css';

const target = document.getElementById('app');

if (target) {
  try {
    await loadProtocol();
    // Imported only after the protocol module is ready, so every store and
    // hook can read from it synchronously at module load.
    const { default: App } = await import('./App.svelte');
    mount(App, { target });
  } catch (error) {
    target.textContent = `This page couldn’t start: ${errorMessage(error)}. Reload to try again.`;
  }
}
