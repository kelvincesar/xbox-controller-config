import { protocol } from '@/lib/protocol';

export type LightModeOptionType = {
  readonly code: number;
  readonly label: string;
  /** Stable name for the preview animation, e.g. `blink-fast`. */
  readonly key: string;
};

/** The modes and brightness range the controller accepts. Never changes. */
export function useLightOptions() {
  const wasm = protocol();
  const modes: LightModeOptionType[] = Array.from(wasm.ledModes(), code => ({
    code,
    label: wasm.ledModeLabel(code),
    key: wasm.ledModeKey(code),
  }));
  return {
    modes,
    offMode: wasm.ledModeOff(),
    defaultMode: wasm.ledModeDefault(),
    maxBrightness: wasm.brightnessMax(),
    defaultBrightness: wasm.brightnessDefault(),
  };
}
