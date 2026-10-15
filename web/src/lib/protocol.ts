// Loads the Rust/WebAssembly protocol module. main.ts awaits loadProtocol()
// before importing the app, so protocol() is safe to call anywhere after that.
import init, * as wasm from '@/wasm/xbox_led.js';

export type ProtocolType = typeof wasm;

let loaded: ProtocolType | null = null;

export async function loadProtocol(): Promise<ProtocolType> {
  if (!loaded) {
    await init();
    loaded = wasm;
  }
  return loaded;
}

export function protocol(): ProtocolType {
  if (!loaded) {
    throw new Error('protocol() called before loadProtocol() finished');
  }
  return loaded;
}
