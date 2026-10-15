// WebHID I/O for a controller paired over Bluetooth.
//
// Web Bluetooth can't reach the controller: Chrome blocklists the
// HID-over-GATT service (0x1812). Once paired in the operating system the
// controller is a HID gamepad, which WebHID can open. Over this link the
// controller only accepts rumble output reports.

const MICROSOFT = 0x045e;
const GAMEPAD = { usagePage: 0x01, usage: 0x05 } as const;

export type BluetoothLink = {
  readonly kind: 'bluetooth';
  readonly name: string;
  /** Output report IDs the controller's HID descriptor accepts. */
  readonly outputReportIds: readonly number[];
  sendReport: (reportId: number, bytes: Uint8Array) => Promise<void>;
  close: () => Promise<void>;
};

type BluetoothHandlers = {
  onReport: (reportId: number, payload: Uint8Array) => void;
  onDisconnect: () => void;
};

export function hidAvailable(): boolean {
  return 'hid' in navigator;
}

function outputReportIds(device: HIDDevice): number[] {
  return (device.collections ?? []).flatMap(collection =>
    (collection.outputReports ?? []).flatMap(report =>
      report.reportId === undefined ? [] : [report.reportId],
    ),
  );
}

/** Ask the user to pick a paired controller, then open it. */
export async function openBluetoothController(
  handlers: BluetoothHandlers,
): Promise<BluetoothLink> {
  const [device] = await navigator.hid.requestDevice({
    filters: [{ vendorId: MICROSOFT, ...GAMEPAD }],
  });
  if (!device) throw new DOMException('No device selected', 'NotFoundError');

  if (!device.opened) await device.open();

  const handleReport = (event: HIDInputReportEvent) => {
    const data = event.data;
    handlers.onReport(
      event.reportId,
      new Uint8Array(data.buffer, data.byteOffset, data.byteLength),
    );
  };
  device.addEventListener('inputreport', handleReport);

  const handleDisconnect = (event: HIDConnectionEvent) => {
    if (event.device !== device) return;
    navigator.hid.removeEventListener('disconnect', handleDisconnect);
    handlers.onDisconnect();
  };
  navigator.hid.addEventListener('disconnect', handleDisconnect);

  return {
    kind: 'bluetooth',
    name: device.productName || 'Xbox Wireless Controller',
    outputReportIds: outputReportIds(device),
    async sendReport(reportId, bytes) {
      // Copy into an ArrayBuffer-backed array, which WebHID requires.
      await device.sendReport(reportId, new Uint8Array(bytes));
    },
    async close() {
      device.removeEventListener('inputreport', handleReport);
      navigator.hid.removeEventListener('disconnect', handleDisconnect);
      await device.close().catch(() => {});
    },
  };
}
