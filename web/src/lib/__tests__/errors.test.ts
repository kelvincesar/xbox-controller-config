import { describe, expect, it } from 'vitest';

import { errorMessage, pickerCancelled } from '../errors';

describe('errorMessage', () => {
  it('when given an Error, it should return its message', () => {
    expect(errorMessage(new Error('boom'))).toBe('boom');
  });

  it('when given a non-Error, it should stringify it', () => {
    expect(errorMessage('plain')).toBe('plain');
    expect(errorMessage(42)).toBe('42');
  });
});

describe('pickerCancelled', () => {
  it('when the picker reports NotFoundError, it should be a cancel', () => {
    expect(pickerCancelled(new DOMException('none', 'NotFoundError'))).toBe(
      true,
    );
  });

  it('when the picker reports AbortError, it should be a cancel', () => {
    expect(pickerCancelled(new DOMException('aborted', 'AbortError'))).toBe(
      true,
    );
  });

  it('when the error is anything else, it should not be a cancel', () => {
    expect(pickerCancelled(new DOMException('busy', 'NetworkError'))).toBe(
      false,
    );
    expect(pickerCancelled(new Error('NotFoundError'))).toBe(false);
  });
});
