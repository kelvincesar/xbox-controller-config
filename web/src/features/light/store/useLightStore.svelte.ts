import type { GipSession } from '@/wasm/xbox_led.js';

import { useLightOptions } from '../hooks/useLightOptions';
import type { LightSettingType } from '../logic/light';

type SentLightType = {
  readonly setting: LightSettingType;
  /** The USB connection it was sent on; a reconnect resets the controller. */
  readonly session: GipSession;
};

const { defaultMode, defaultBrightness } = useLightOptions();

class LightStore {
  mode = $state(defaultMode);
  brightness = $state(defaultBrightness);
  sent = $state.raw<SentLightType | null>(null);
  isSending = $state(false);
  error = $state('');
}

const store = new LightStore();

export function useLightStore() {
  return store;
}
