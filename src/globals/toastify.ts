export function useToastification() {
  const TIMEOUT = 2000;
  const onSuccess = (message: string, timeout?: number) => {};

  const onError = (message: string, timeout?: number) => {};

  return { onSuccess, onError };
}
