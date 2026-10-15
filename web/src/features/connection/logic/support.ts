export type SupportProblemType = 'insecure' | 'unsupported' | null;

export type SupportType = {
  readonly canUseUsb: boolean;
  readonly canUseBluetooth: boolean;
  readonly problem: SupportProblemType;
};

type BrowserFactsType = {
  isSecure: boolean;
  hasUsb: boolean;
  hasHid: boolean;
};

/** Which connections this browser allows, and why when it allows none. */
export function browserSupport({
  isSecure,
  hasUsb,
  hasHid,
}: BrowserFactsType): SupportType {
  if (!isSecure) {
    return { canUseUsb: false, canUseBluetooth: false, problem: 'insecure' };
  }
  return {
    canUseUsb: hasUsb,
    canUseBluetooth: hasHid,
    problem: hasUsb || hasHid ? null : 'unsupported',
  };
}
