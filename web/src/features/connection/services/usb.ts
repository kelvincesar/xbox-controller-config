// WebUSB I/O for a wired controller. The controller's GIP interface is
// vendor-specific (class 0xFF, subclass 0x47, protocol 0xD0), which is why
// WebUSB can reach it at all: HID-class devices are off limits to WebUSB.

const GIP_INTERFACE = {
  classCode: 0xff,
  subclassCode: 0x47,
  protocolCode: 0xd0,
} as const;
const MICROSOFT = 0x045e;
const PACKET_SIZE = 64;

export type UsbLink = {
  readonly kind: 'usb';
  readonly name: string;
  send: (bytes: Uint8Array) => Promise<void>;
  close: () => Promise<void>;
};

type UsbHandlers = {
  onPacket: (bytes: Uint8Array) => void;
  onDisconnect: () => void;
};

type GipEndpoints = { interfaceNumber: number; out: number; in: number };

export function usbAvailable(): boolean {
  return 'usb' in navigator;
}

function gipEndpoints(configuration: USBConfiguration): GipEndpoints | null {
  for (const iface of configuration.interfaces) {
    const alt = iface.alternates[0];
    if (
      alt?.interfaceClass !== GIP_INTERFACE.classCode ||
      alt.interfaceSubclass !== GIP_INTERFACE.subclassCode ||
      alt.interfaceProtocol !== GIP_INTERFACE.protocolCode
    ) {
      continue;
    }
    const endpointOut = alt.endpoints.find(e => e.direction === 'out');
    const endpointIn = alt.endpoints.find(e => e.direction === 'in');
    if (endpointOut && endpointIn) {
      return {
        interfaceNumber: iface.interfaceNumber,
        out: endpointOut.endpointNumber,
        in: endpointIn.endpointNumber,
      };
    }
  }
  return null;
}

async function readPackets(
  device: USBDevice,
  endpoint: number,
  isOpen: () => boolean,
  onPacket: (bytes: Uint8Array) => void,
) {
  while (isOpen()) {
    try {
      const result = await device.transferIn(endpoint, PACKET_SIZE);
      const data = result.data;
      if (data && data.byteLength > 0) {
        onPacket(new Uint8Array(data.buffer, data.byteOffset, data.byteLength));
      }
    } catch {
      // transferIn rejects once the device is closed or unplugged.
      return;
    }
  }
}

/** Ask the user to pick a controller, then claim its GIP interface. */
export async function openUsbController(
  handlers: UsbHandlers,
): Promise<UsbLink> {
  const device = await navigator.usb.requestDevice({
    filters: [{ vendorId: MICROSOFT, ...GIP_INTERFACE }, GIP_INTERFACE],
  });

  await device.open();
  // Chrome reports null (typed as undefined) until a configuration is chosen.
  if (!device.configuration) await device.selectConfiguration(1);

  const endpoints = device.configuration
    ? gipEndpoints(device.configuration)
    : null;
  if (!endpoints) {
    await device.close();
    throw new Error(
      'This device has no Xbox controller interface. Pick an Xbox One or Series controller.',
    );
  }

  try {
    await device.claimInterface(endpoints.interfaceNumber);
  } catch {
    await device.close().catch(() => {});
    throw new Error(
      'Your operating system is already using this controller. Follow “Can’t connect?” below to release it, then try again.',
    );
  }

  let isOpen = true;

  const handleDisconnect = (event: USBConnectionEvent) => {
    if (event.device !== device) return;
    isOpen = false;
    navigator.usb.removeEventListener('disconnect', handleDisconnect);
    handlers.onDisconnect();
  };
  navigator.usb.addEventListener('disconnect', handleDisconnect);

  void readPackets(device, endpoints.in, () => isOpen, handlers.onPacket);

  return {
    kind: 'usb',
    name: device.productName || 'Xbox controller',
    async send(bytes) {
      // Copy into an ArrayBuffer-backed array, which WebUSB requires.
      const result = await device.transferOut(
        endpoints.out,
        new Uint8Array(bytes),
      );
      if (result.status !== 'ok') {
        throw new Error(`USB transfer failed (${result.status})`);
      }
    },
    async close() {
      isOpen = false;
      navigator.usb.removeEventListener('disconnect', handleDisconnect);
      await device.releaseInterface(endpoints.interfaceNumber).catch(() => {});
      await device.close().catch(() => {});
    },
  };
}
