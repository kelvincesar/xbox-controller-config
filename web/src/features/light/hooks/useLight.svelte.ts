import { useUsbController } from '@/features/connection/hooks/useUsbController.svelte';
import { errorMessage } from '@/lib/errors';

import { hasUnsentChanges, type LightSettingType } from '../logic/light';
import { useLightStore } from '../store/useLightStore.svelte';
import { useLightOptions } from './useLightOptions';

/** Send the chosen light setting to a wired controller. */
export function useLight() {
  const store = useLightStore();
  const usb = useUsbController();
  const { offMode } = useLightOptions();

  async function send(setting: LightSettingType) {
    store.error = '';
    store.isSending = true;
    try {
      await usb.sendPacket(session =>
        session.ledPacket(setting.mode, setting.brightness),
      );
      if (usb.session) store.sent = { setting, session: usb.session };
    } catch (error) {
      store.error = `The controller didn’t accept the command: ${errorMessage(error)}`;
    } finally {
      store.isSending = false;
    }
  }

  function handleSend() {
    return send({ mode: store.mode, brightness: store.brightness });
  }

  function handleTurnOff() {
    store.mode = offMode;
    return send({ mode: offMode, brightness: 0 });
  }

  return {
    get canSend() {
      return usb.isLinked && !store.isSending;
    },
    get hasUnsentChanges() {
      const sentHere =
        store.sent && store.sent.session === usb.session
          ? store.sent.setting
          : null;
      return hasUnsentChanges(
        sentHere,
        { mode: store.mode, brightness: store.brightness },
        offMode,
      );
    },
    get error() {
      return store.error;
    },
    handleSend,
    handleTurnOff,
  };
}
