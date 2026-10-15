/** `3` → `0x03`. */
export function reportIdLabel(reportId: number): string {
  return `0x${reportId.toString(16).padStart(2, '0').toUpperCase()}`;
}

/** `[3, 5]` → `0x03, 0x05`; an empty list reads `none`. */
export function reportIdsLabel(reportIds: readonly number[]): string {
  return reportIds.length === 0
    ? 'none'
    : reportIds.map(reportIdLabel).join(', ');
}
