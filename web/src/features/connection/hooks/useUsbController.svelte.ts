import { useActivityStore } from '@/features/activity/store/useActivityStore.svelte';
import { protocol } from '@/lib/protocol';
import type { GipSession } from '@/wasm/xbox_led.js';

import { useConnectionStore } from '../store/useConnectionStore.svelte';

/** Send GIP packets to the controller on the current USB link. */
export function useUsbController() {
  const store = useConnectionStore();
  const activity = useActivityStore();
  const wasm = protocol();

  /** Build a packet with this link's session, then send it. */
  async function sendPacket(build: (session: GipSession) => Uint8Array) {
    const { link, session } = store;
    if (link?.kind !== 'usb' || !session) {
      throw new Error('Connect a controller with a USB cable first.');
    }
    const packet = build(session);
    await link.send(packet);
    activity.record('sent', wasm.packetSummary(packet));
  }

  return {
    get isLinked() {
      return store.link?.kind === 'usb';
    },
    /** Changes whenever a new USB connection starts. */
    get session() {
      return store.session;
    },
    sendPacket,
  };
}
