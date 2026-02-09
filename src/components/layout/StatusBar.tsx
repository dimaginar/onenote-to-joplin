import { useStatusBar } from "../../hooks/useStatusBar";
import { cn } from "../../utils/cn";

const statusColors = {
  info: "text-muted-foreground",
  error: "text-destructive-foreground",
  success: "text-success",
};

export function StatusBar() {
  const { statusMessage, statusType } = useStatusBar();

  return (
    <div className="flex items-center justify-between border-t border-border bg-card/50 px-4 py-1.5">
      <span className={cn("text-xs", statusColors[statusType])}>
        {statusMessage}
      </span>
      <span className="text-xs text-muted-foreground">v0.1.0</span>
    </div>
  );
}
