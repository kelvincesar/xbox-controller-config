import { useActivityStore } from '@/features/activity/store/useActivityStore.svelte';
import { errorMessage } from '@/lib/errors';
import { protocol } from '@/lib/protocol';

import { reportIdLabel } from '../logic/reports';
import { useConnectionStore } from '../store/useConnectionStore.svelte';

const PULSE_MS = 350;

/** A short rumble over Bluetooth, to prove the link works. */
export function useRumble() {
  const store = useConnectionStore();
  const activity = useActivityStore();
  const wasm = protocol();

  async function testRumble() {
    const link = store.link;
    if (link?.kind !== 'bluetooth') return;
    store.error = '';
    const reportId = wasm.rumbleReportId();
    try {
      await link.sendReport(reportId, wasm.testRumbleReport());
      activity.record('sent', `rumble on (report ${reportIdLabel(reportId)})`);
      await new Promise(resolve => setTimeout(resolve, PULSE_MS));
      await link.sendReport(reportId, wasm.stopRumbleReport());
      activity.record('sent', `rumble off (report ${reportIdLabel(reportId)})`);
    } catch (error) {
      store.error = `Rumble failed: ${errorMessage(error)}`;
    }
  }

  return { testRumble };
}
