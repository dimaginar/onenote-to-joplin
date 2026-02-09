import { useEffect } from "react";
import { useAppStore } from "../stores/appStore";

export function useKeyboard() {
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const state = useAppStore.getState();

      // Ctrl+R: Re-scan (only when not already scanning)
      if (e.ctrlKey && e.key === "r") {
        e.preventDefault();
        if (state.view !== "scanning") {
          state.startScan();
        }
      }

      // Ctrl+S: Save report (only when results available)
      if (e.ctrlKey && e.key === "s") {
        e.preventDefault();
        if (state.scanResult) {
          state.saveReport();
        }
      }

      // Escape: Exit wizard
      if (e.key === "Escape" && state.view === "wizard") {
        state.exitWizard();
      }
    };

    window.addEventListener("keydown", handler);
    return () => window.removeEventListener("keydown", handler);
  }, []);
}
