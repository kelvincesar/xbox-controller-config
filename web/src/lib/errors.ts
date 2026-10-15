/** Readable text for anything thrown. */
export function errorMessage(error: unknown): string {
  if (error instanceof Error) return error.message;
  return String(error);
}

/** The user closed the browser's device picker without choosing a device. */
export function pickerCancelled(error: unknown): boolean {
  return (
    error instanceof DOMException &&
    (error.name === 'NotFoundError' || error.name === 'AbortError')
  );
}
