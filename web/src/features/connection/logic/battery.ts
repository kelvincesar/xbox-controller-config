export type BatteryType = {
  readonly percent: number;
  readonly isCharging: boolean;
};

/** `battery about 70%, charging` — or empty when the level is unknown yet. */
export function batteryLabel(battery: BatteryType | null): string {
  if (!battery) return '';
  const charging = battery.isCharging ? ', charging' : '';
  return `battery about ${battery.percent}%${charging}`;
}
