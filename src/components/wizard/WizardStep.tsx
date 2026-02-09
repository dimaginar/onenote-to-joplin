import { AlertTriangle, CircleX } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "../ui/card";
import type { CheckResult } from "../../stores/types";
import { cn } from "../../utils/cn";

interface WizardStepProps {
  check: CheckResult;
  stepNumber: number;
  totalSteps: number;
}

export function WizardStep({ check, stepNumber, totalSteps }: WizardStepProps) {
  const Icon = check.status === "fail" ? CircleX : AlertTriangle;
  const iconColor =
    check.status === "fail" ? "text-destructive-foreground" : "text-warning";

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center gap-2">
          <Icon className={cn("h-5 w-5", iconColor)} />
          <CardTitle className="text-base">
            Step {stepNumber} of {totalSteps}: {check.label}
          </CardTitle>
        </div>
        <p className="text-sm text-muted-foreground mt-1">{check.message}</p>
      </CardHeader>
      <CardContent>
        {check.remediation && (
          <div className="rounded-lg bg-secondary p-4 text-sm leading-relaxed">
            <p className="font-medium mb-2">How to fix:</p>
            <p>{check.remediation}</p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
