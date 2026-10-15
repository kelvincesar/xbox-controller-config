import { describe, expect, it } from 'vitest';

import { withEntry, type ActivityEntryType } from '../activity';

function entry(id: number): ActivityEntryType {
  return { id, time: '12:00:00', direction: 'info', text: `entry ${id}` };
}

describe('withEntry', () => {
  it('when adding an entry, it should put it first', () => {
    const log = withEntry([entry(1)], entry(2), 10);
    expect(log.map(e => e.id)).toEqual([2, 1]);
  });

  it('when the log is full, it should drop the oldest entry', () => {
    const log = withEntry([entry(2), entry(1)], entry(3), 2);
    expect(log.map(e => e.id)).toEqual([3, 2]);
  });

  it('when adding, it should not mutate the original log', () => {
    const original = [entry(1)];
    withEntry(original, entry(2), 10);
    expect(original).toHaveLength(1);
  });
});
