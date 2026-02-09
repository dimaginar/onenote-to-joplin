export type CheckStatus = "pass" | "fail" | "warning" | "skipped";

export interface CheckResult {
  id: string;
  label: string;
  status: CheckStatus;
  message: string;
  remediation: string | null;
}

export interface ScanResult {
  checks: CheckResult[];
  timestamp: string;
  osInfo: string;
  overall: CheckStatus;
}

export type AppView = "empty" | "scanning" | "results" | "wizard";

export type StatusType = "info" | "error" | "success";

export interface AppState {
  view: AppView;
  scanResult: ScanResult | null;
  scanError: string | null;
  wizardStep: number;
  failedChecks: CheckResult[];
  selectedCheckId: string | null;
  statusMessage: string;
  statusType: StatusType;

  startScan: () => Promise<void>;
  resetScan: () => void;
  selectCheck: (id: string | null) => void;
  enterWizard: () => void;
  exitWizard: () => void;
  nextWizardStep: () => void;
  prevWizardStep: () => void;
  generateReport: () => Promise<string>;
  saveReport: () => Promise<void>;
}
