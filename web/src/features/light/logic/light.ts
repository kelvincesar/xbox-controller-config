export type LightSettingType = {
  readonly mode: number;
  readonly brightness: number;
};

/** How lit the preview looks, from 0 (dark) to 1 (full). */
export function previewLevel(
  setting: LightSettingType,
  maxBrightness: number,
  offMode: number,
): number {
  if (setting.mode === offMode || maxBrightness <= 0) return 0;
  return (
    Math.min(Math.max(setting.brightness, 0), maxBrightness) / maxBrightness
  );
}

/**
 * Whether the controller is showing something other than `current`.
 * Brightness doesn't matter while the light is off.
 */
export function hasUnsentChanges(
  sent: LightSettingType | null,
  current: LightSettingType,
  offMode: number,
): boolean {
  if (!sent || sent.mode !== current.mode) return true;
  if (current.mode === offMode) return false;
  return sent.brightness !== current.brightness;
}
