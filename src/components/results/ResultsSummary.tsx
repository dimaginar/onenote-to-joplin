import { RotateCcw, Download } from "lucide-react";
import { Badge } from "../ui/badge";
import { Button } from "../ui/button";
import type { ScanResult } from "../../stores/types";

interface ResultsSummaryProps {
  result: ScanResult;
  onRescan: () => void;
  onReport: () => void;
  canSaveReport: boolean;
}

export function ResultsSummary({
  result,
  onRescan,
  onReport,
  canSaveReport,
}: ResultsSummaryProps) {
  const passCount = result.checks.filter((c) => c.status === "pass").length;
  const failCount = result.checks.filter((c) => c.status === "fail").length;
  const warnCount = result.checks.filter((c) => c.status === "warning").length;
  const skippedCount = result.checks.filter((c) => c.status === "skipped").length;

  return (
    <div className="flex items-center justify-between px-4 py-3 border-b border-border">
      <div className="flex items-center gap-3">
        <h2 className="text-base font-semibold whitespace-nowrap">
          OneNote to Joplin Readiness
        </h2>
        <div className="flex items-center gap-1.5">
          {failCount > 0 && (
            <Badge variant="destructive">{failCount} Failed</Badge>
          )}
          {warnCount > 0 && (
            <Badge variant="warning">{warnCount} Warning</Badge>
          )}
          <Badge variant="success">{passCount} Ready</Badge>
          {skippedCount > 0 && (
            <Badge variant="secondary">{skippedCount} Skipped</Badge>
          )}
        </div>
      </div>
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onClick={onRescan}
          className="gap-1.5"
        >
          <RotateCcw className="h-3.5 w-3.5" />
          Re-Scan
        </Button>
        <Button
          variant="secondary"
          size="sm"
          onClick={onReport}
          disabled={!canSaveReport}
          className="gap-1.5"
        >
          <Download className="h-3.5 w-3.5" />
          Report
        </Button>
      </div>
    </div>
  );
}
