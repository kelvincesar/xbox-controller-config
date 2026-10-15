import { browserSupport, type SupportType } from '../logic/support';
import { hidAvailable } from '../services/bluetooth';
import { usbAvailable } from '../services/usb';

/** What this browser lets the page connect with. Fixed for the page's life. */
export function useBrowserSupport(): SupportType {
  return browserSupport({
    isSecure: window.isSecureContext,
    hasUsb: usbAvailable(),
    hasHid: hidAvailable(),
  });
}
