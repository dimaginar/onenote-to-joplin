import {
  CircleCheck,
  CircleX,
  AlertTriangle,
  CircleHelp,
  MinusCircle,
  Wrench,
} from "lucide-react";
import type { CheckResult } from "../../stores/types";
import { cn } from "../../utils/cn";
import { Button } from "../ui/button";

const statusConfig = {
  pass: {
    icon: CircleCheck,
    color: "text-success",
    bg: "bg-success/10",
    label: "Passed",
  },
  fail: {
    icon: CircleX,
    color: "text-destructive-foreground",
    bg: "bg-destructive/10",
    label: "Failed",
  },
  warning: {
    icon: AlertTriangle,
    color: "text-warning",
    bg: "bg-warning/10",
    label: "Warning",
  },
  skipped: {
    icon: MinusCircle,
    color: "text-muted-foreground",
    bg: "bg-muted/10",
    label: "Skipped",
  },
};

interface CheckDetailProps {
  check: CheckResult | null;
  issueCount: number;
  onStartGuide?: () => void;
}

export function CheckDetail({
  check,
  issueCount,
  onStartGuide,
}: CheckDetailProps) {
  if (!check) {
    return (
      <div className="flex flex-col items-center justify-center h-full px-8 text-center">
        <CircleHelp className="h-12 w-12 mb-4 text-muted-foreground/40" />
        <h3 className="text-base font-medium mb-1">
          Select a check to review
        </h3>
        <p className="text-sm text-muted-foreground max-w-xs">
          {issueCount > 0
            ? `${issueCount} item${issueCount !== 1 ? "s" : ""} need attention before migration`
            : "All checks passed \u2014 you're ready to migrate"}
        </p>
      </div>
    );
  }

  const config = statusConfig[check.status];
  const Icon = config.icon;
  const hasRemediation = check.remediation && check.status !== "pass";

  return (
    <div className="p-5 space-y-4">
      <div className="flex items-start gap-3">
        <div className={cn("rounded-full p-2", config.bg)}>
          <Icon className={cn("h-5 w-5", config.color)} />
        </div>
        <div>
          <h3 className="text-base font-semibold">{check.label}</h3>
          <p className="text-sm text-muted-foreground mt-0.5">
            {config.label}
          </p>
        </div>
      </div>

      <div className="rounded-lg border border-border bg-secondary/30 p-4">
        <p className="text-sm leading-relaxed">{check.message}</p>
      </div>

      {hasRemediation && (
        <div className="space-y-3">
          <h4 className="text-sm font-medium">How to fix</h4>
          <div className="rounded-lg border border-warning/20 bg-warning/5 p-4">
            <p className="text-sm leading-relaxed">{check.remediation}</p>
          </div>
          {onStartGuide && (
            <Button size="sm" onClick={onStartGuide} className="gap-1.5">
              <Wrench className="h-3.5 w-3.5" />
              Start Guided Fix
            </Button>
          )}
        </div>
      )}

      {check.status === "pass" && (
        <p className="text-sm text-muted-foreground">
          No action required. This check passed successfully.
        </p>
      )}

      {check.status === "skipped" && (
        <p className="text-sm text-muted-foreground">
          This check was skipped because OneNote Desktop is not installed, so it does not apply. No action is required.
        </p>
      )}
    </div>
  );
}
