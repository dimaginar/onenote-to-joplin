import { create } from "zustand";
import type { AppState, ScanResult } from "./types";
import { isTauri } from "../utils/tauri";

const mockScanResult: ScanResult = {
  checks: [
    { id: "joplin", label: "Joplin", status: "pass", message: "Joplin 3.1.24 found at C:\\Users\\User\\AppData\\Local\\Programs\\joplin", remediation: null },
    { id: "windows_os", label: "Windows OS", status: "pass", message: "Windows 11 Enterprise 23H2 (Build 22631.4890)", remediation: null },
    { id: "onenote", label: "OneNote (Desktop)", status: "pass", message: "Version 16.0 found", remediation: null },
    { id: "word", label: "Word", status: "fail", message: "Word desktop not found", remediation: "Install Microsoft Office with Word included." },
    { id: "com_bridge", label: "COM Automation", status: "warning", message: "OneNote.Application OK. Word.Application failed.", remediation: "Repair your Office installation." },
    { id: "sync_auto", label: "OneNote Auto-Sync", status: "skipped", message: "Skipped \u2014 OneNote Desktop not installed", remediation: null },
    { id: "sync_download", label: "OneNote Full Download", status: "skipped", message: "Skipped \u2014 OneNote Desktop not installed", remediation: null },
  ],
  timestamp: new Date().toISOString(),
  osInfo: "Windows 11 Enterprise 23H2 (Build 22631.4890)",
  overall: "fail",
};

export const useAppStore = create<AppState>((set, get) => ({
  view: "empty",
  scanResult: null,
  scanError: null,
  wizardStep: 0,
  failedChecks: [],
  selectedCheckId: null,
  statusMessage: "Ready",
  statusType: "info",

  startScan: async () => {
    set({
      view: "scanning",
      scanError: null,
      statusMessage: "Scanning...",
      statusType: "info",
    });

    try {
      let result: ScanResult;

      if (isTauri()) {
        const { invoke } = await import("@tauri-apps/api/core");
        result = await invoke<ScanResult>("run_readiness_scan");
      } else {
        // Mock for browser dev
        await new Promise((r) => setTimeout(r, 1500));
        result = mockScanResult;
      }

      const failed = result.checks.filter((c) => c.status !== "pass" && c.status !== "skipped");
      const firstIssue = failed.length > 0 ? failed[0].id : null;
      set({
        view: "results",
        scanResult: result,
        failedChecks: failed,
        selectedCheckId: firstIssue,
        statusMessage:
          result.overall === "pass"
            ? "All checks passed"
            : `${failed.length} issue(s) found`,
        statusType: result.overall === "pass" ? "success" : "error",
      });
    } catch (err) {
      set({
        view: "empty",
        scanError: String(err),
        statusMessage: `Scan failed: ${err}`,
        statusType: "error",
      });
    }
  },

  resetScan: () =>
    set({
      view: "empty",
      scanResult: null,
      scanError: null,
      selectedCheckId: null,
      statusMessage: "Ready",
      statusType: "info",
    }),

  selectCheck: (id) => set({ selectedCheckId: id }),

  enterWizard: () => set({ view: "wizard", wizardStep: 0 }),

  exitWizard: () => set({ view: "results" }),

  nextWizardStep: () => {
    const { wizardStep, failedChecks } = get();
    if (wizardStep < failedChecks.length - 1) {
      set({ wizardStep: wizardStep + 1 });
    }
  },

  prevWizardStep: () => {
    const { wizardStep } = get();
    if (wizardStep > 0) {
      set({ wizardStep: wizardStep - 1 });
    }
  },

  generateReport: async () => {
    const { scanResult } = get();
    if (!scanResult) throw new Error("No scan result");

    if (isTauri()) {
      const { invoke } = await import("@tauri-apps/api/core");
      return invoke<string>("generate_report", { results: scanResult });
    }

    // Mock for browser dev
    return `# Readiness Report\n\nGenerated: ${scanResult.timestamp}\n\n${scanResult.checks
      .map((c) => `- ${c.label}: ${c.status.toUpperCase()} - ${c.message}`)
      .join("\n")}`;
  },

  saveReport: async () => {
    const markdown = await get().generateReport();

    if (isTauri()) {
      const { save } = await import("@tauri-apps/plugin-dialog");
      const { invoke } = await import("@tauri-apps/api/core");
      const path = await save({
        defaultPath: "readiness-report.md",
        filters: [{ name: "Markdown", extensions: ["md"] }],
      });
      if (path) {
        await invoke("save_report", { markdown, path });
        set({ statusMessage: "Report saved", statusType: "success" });
      }
    } else {
      // Browser fallback: download as file
      const blob = new Blob([markdown], { type: "text/markdown" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = "readiness-report.md";
      a.click();
      URL.revokeObjectURL(url);
      set({ statusMessage: "Report downloaded", statusType: "success" });
    }
  },
}));
