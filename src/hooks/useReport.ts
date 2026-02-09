import { useAppStore } from "../stores/appStore";

export function useReport() {
  const scanResult = useAppStore((s) => s.scanResult);
  const saveReport = useAppStore((s) => s.saveReport);
  const canSave = scanResult !== null;
  return { canSave, saveReport };
}
