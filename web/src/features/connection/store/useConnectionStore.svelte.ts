import type { GipSession } from '@/wasm/xbox_led.js';

import type { BatteryType } from '../logic/battery';
import type { BluetoothLink } from '../services/bluetooth';
import type { UsbLink } from '../services/usb';

export type LinkType = UsbLink | BluetoothLink;

class ConnectionStore {
  // Device handles and wasm objects must not be wrapped in reactive proxies.
  link = $state.raw<LinkType | null>(null);
  /** Packet sequence state for the current USB link. */
  session = $state.raw<GipSession | null>(null);
  battery = $state<BatteryType | null>(null);
  isBusy = $state(false);
  error = $state('');

  reset = () => {
    this.session?.free();
    this.link = null;
    this.session = null;
    this.battery = null;
  };
}

const store = new ConnectionStore();

export function useConnectionStore() {
  return store;
}
