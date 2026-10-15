import { describe, expect, it } from 'vitest';

import { reportIdLabel, reportIdsLabel } from '../reports';

describe('reportIdLabel', () => {
  it('when the id is a single hex digit, it should pad to two', () => {
    expect(reportIdLabel(3)).toBe('0x03');
  });

  it('when the id has letters, it should use upper case', () => {
    expect(reportIdLabel(0xab)).toBe('0xAB');
  });
});

describe('reportIdsLabel', () => {
  it('when there are no ids, it should say none', () => {
    expect(reportIdsLabel([])).toBe('none');
  });

  it('when there are several ids, it should join them', () => {
    expect(reportIdsLabel([3, 5])).toBe('0x03, 0x05');
  });
});
