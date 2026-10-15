import { describe, expect, it } from 'vitest';

import { batteryLabel } from '../battery';

describe('batteryLabel', () => {
  it('when no battery report has arrived, it should be empty', () => {
    expect(batteryLabel(null)).toBe('');
  });

  it('when running on battery, it should show the level only', () => {
    expect(batteryLabel({ percent: 70, isCharging: false })).toBe(
      'battery about 70%',
    );
  });

  it('when charging, it should say so', () => {
    expect(batteryLabel({ percent: 40, isCharging: true })).toBe(
      'battery about 40%, charging',
    );
  });
});
