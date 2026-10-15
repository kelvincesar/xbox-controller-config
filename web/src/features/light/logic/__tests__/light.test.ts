import { describe, expect, it } from 'vitest';

import { hasUnsentChanges, previewLevel } from '../light';

const OFF = 0x00;
const STEADY = 0x01;
const BLINK = 0x03;

describe('previewLevel', () => {
  it('when the mode is off, it should be dark whatever the brightness', () => {
    expect(previewLevel({ mode: OFF, brightness: 50 }, 50, OFF)).toBe(0);
  });

  it('when on, it should scale brightness to the maximum', () => {
    expect(previewLevel({ mode: STEADY, brightness: 25 }, 50, OFF)).toBe(0.5);
    expect(previewLevel({ mode: STEADY, brightness: 50 }, 50, OFF)).toBe(1);
  });

  it('when brightness is beyond the maximum, it should cap at full', () => {
    expect(previewLevel({ mode: STEADY, brightness: 80 }, 50, OFF)).toBe(1);
  });

  it('when the maximum is zero, it should be dark instead of dividing by zero', () => {
    expect(previewLevel({ mode: STEADY, brightness: 10 }, 0, OFF)).toBe(0);
  });
});

describe('hasUnsentChanges', () => {
  it('when nothing was sent on this connection, it should have changes', () => {
    expect(hasUnsentChanges(null, { mode: STEADY, brightness: 20 }, OFF)).toBe(
      true,
    );
  });

  it('when the mode differs, it should have changes', () => {
    expect(
      hasUnsentChanges(
        { mode: STEADY, brightness: 20 },
        { mode: BLINK, brightness: 20 },
        OFF,
      ),
    ).toBe(true);
  });

  it('when on and brightness differs, it should have changes', () => {
    expect(
      hasUnsentChanges(
        { mode: STEADY, brightness: 20 },
        { mode: STEADY, brightness: 30 },
        OFF,
      ),
    ).toBe(true);
  });

  it('when off and only brightness differs, it should have no changes', () => {
    expect(
      hasUnsentChanges(
        { mode: OFF, brightness: 0 },
        { mode: OFF, brightness: 30 },
        OFF,
      ),
    ).toBe(false);
  });

  it('when everything matches, it should have no changes', () => {
    expect(
      hasUnsentChanges(
        { mode: BLINK, brightness: 12 },
        { mode: BLINK, brightness: 12 },
        OFF,
      ),
    ).toBe(false);
  });
});
