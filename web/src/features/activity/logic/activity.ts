export type ActivityDirectionType = 'sent' | 'received' | 'info';

export type ActivityEntryType = {
  readonly id: number;
  readonly time: string;
  readonly direction: ActivityDirectionType;
  readonly text: string;
};

/** The log with `entry` added on top, keeping at most `limit` entries. */
export function withEntry(
  entries: readonly ActivityEntryType[],
  entry: ActivityEntryType,
  limit: number,
): ActivityEntryType[] {
  return [entry, ...entries].slice(0, limit);
}
