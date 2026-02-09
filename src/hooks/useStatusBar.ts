import { useAppStore } from "../stores/appStore";

export function useStatusBar() {
  const statusMessage = useAppStore((s) => s.statusMessage);
  const statusType = useAppStore((s) => s.statusType);
  return { statusMessage, statusType };
}
