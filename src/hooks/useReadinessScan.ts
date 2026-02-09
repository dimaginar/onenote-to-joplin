import { useAppStore } from "../stores/appStore";

export function useReadinessScan() {
  const view = useAppStore((s) => s.view);
  const scanResult = useAppStore((s) => s.scanResult);
  const scanError = useAppStore((s) => s.scanError);
  const startScan = useAppStore((s) => s.startScan);
  const resetScan = useAppStore((s) => s.resetScan);
  return { view, scanResult, scanError, startScan, resetScan };
}
