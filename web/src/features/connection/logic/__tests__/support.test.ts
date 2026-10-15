import { describe, expect, it } from 'vitest';

import { browserSupport } from '../support';

describe('browserSupport', () => {
  it('when the page is not secure, it should allow nothing', () => {
    expect(
      browserSupport({ isSecure: false, hasUsb: true, hasHid: true }),
    ).toEqual({
      canUseUsb: false,
      canUseBluetooth: false,
      problem: 'insecure',
    });
  });

  it('when the browser has neither API, it should report unsupported', () => {
    expect(
      browserSupport({ isSecure: true, hasUsb: false, hasHid: false }).problem,
    ).toBe('unsupported');
  });

  it('when the browser has only WebHID, it should allow only Bluetooth', () => {
    expect(
      browserSupport({ isSecure: true, hasUsb: false, hasHid: true }),
    ).toEqual({ canUseUsb: false, canUseBluetooth: true, problem: null });
  });

  it('when the browser has both APIs, it should allow both', () => {
    expect(
      browserSupport({ isSecure: true, hasUsb: true, hasHid: true }),
    ).toEqual({ canUseUsb: true, canUseBluetooth: true, problem: null });
  });
});
