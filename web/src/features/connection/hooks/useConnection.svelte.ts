import { useActivityStore } from '@/features/activity/store/useActivityStore.svelte';
import { errorMessage, pickerCancelled } from '@/lib/errors';
import { protocol } from '@/lib/protocol';

import { reportIdsLabel } from '../logic/reports';
import { openBluetoothController } from '../services/bluetooth';
import { openUsbController } from '../services/usb';
import { useConnectionStore } from '../store/useConnectionStore.svelte';
import { useUsbController } from './useUsbController.svelte';

/** Connect to and disconnect from a controller. */
export function useConnection() {
  const store = useConnectionStore();
  const activity = useActivityStore();
  const usb = useUsbController();
  const wasm = protocol();

  function handleLinkLost(message: string) {
    store.reset();
    activity.record('info', message);
  }

  async function runExclusively(task: () => Promise<void>) {
    store.error = '';
    store.isBusy = true;
    try {
      await task();
    } catch (error) {
      if (!pickerCancelled(error)) store.error = errorMessage(error);
    } finally {
      store.isBusy = false;
    }
  }

  async function closeLink() {
    const link = store.link;
    if (!link) return;
    store.reset();
    await link.close();
    activity.record('info', 'Disconnected.');
  }

  const connectUsb = () =>
    runExclusively(async () => {
      await closeLink();
      const link = await openUsbController({
        onPacket: bytes => {
          if (!wasm.noisyPacket(bytes)) {
            activity.record('received', wasm.packetSummary(bytes));
          }
        },
        onDisconnect: () => handleLinkLost('Controller unplugged.'),
      });
      store.link = link;
      store.session = new wasm.GipSession();
      activity.record('info', `Connected to ${link.name} over USB.`);
      await usb.sendPacket(session => session.powerOnPacket());
    });

  const connectBluetooth = () =>
    runExclusively(async () => {
      await closeLink();
      const link = await openBluetoothController({
        onReport: (reportId, payload) => {
          const status = wasm.batteryStatus(reportId, payload);
          if (!status) return;
          store.battery = {
            percent: status.percent,
            isCharging: status.charging,
          };
          status.free();
        },
        onDisconnect: () =>
          handleLinkLost('Bluetooth controller disconnected.'),
      });
      store.link = link;
      activity.record(
        'info',
        `Connected to ${link.name} over Bluetooth. Output reports: ${reportIdsLabel(link.outputReportIds)}.`,
      );
    });

  const disconnect = () => runExclusively(closeLink);

  return {
    get link() {
      return store.link;
    },
    get battery() {
      return store.battery;
    },
    get isBusy() {
      return store.isBusy;
    },
    get error() {
      return store.error;
    },
    connectUsb,
    connectBluetooth,
    disconnect,
  };
}
