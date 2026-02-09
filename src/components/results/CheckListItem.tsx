import { CircleCheck, CircleX, AlertTriangle, MinusCircle } from "lucide-react";
import type { CheckResult } from "../../stores/types";
import { cn } from "../../utils/cn";

const statusIcon = {
  pass: CircleCheck,
  fail: CircleX,
  warning: AlertTriangle,
  skipped: MinusCircle,
};

const statusColor = {
  pass: "text-success",
  fail: "text-destructive-foreground",
  warning: "text-warning",
  skipped: "text-muted-foreground",
};

interface CheckListItemProps {
  check: CheckResult;
  selected: boolean;
  onSelect: () => void;
}

export function CheckListItem({ check, selected, onSelect }: CheckListItemProps) {
  const Icon = statusIcon[check.status];
  const isCompact = check.status === "pass" || check.status === "skipped";

  return (
    <button
      onClick={onSelect}
      className={cn(
        "relative flex items-center gap-2.5 px-3 w-full text-left transition-colors",
        isCompact ? "py-1.5" : "py-2.5",
        "hover:bg-accent/50",
        selected &&
          "bg-accent before:absolute before:left-0 before:top-0 before:bottom-0 before:w-1 before:rounded-r-sm before:bg-primary"
      )}
    >
      <Icon className={cn("h-4 w-4 shrink-0", statusColor[check.status])} />
      <div className="min-w-0 flex-1">
        <span
          className={cn(
            "text-sm block truncate",
            isCompact ? "text-muted-foreground" : "font-medium"
          )}
        >
          {check.label}
        </span>
        {!isCompact && (
          <p className="text-xs text-muted-foreground truncate mt-0.5">
            {check.message}
          </p>
        )}
      </div>
    </button>
  );
}
