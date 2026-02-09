import { ScanSearch } from "lucide-react";
import { Button } from "../ui/button";
import { useReadinessScan } from "../../hooks/useReadinessScan";

export function EmptyState() {
  const { startScan, scanError } = useReadinessScan();

  return (
    <div className="flex flex-col items-center gap-6 text-center max-w-md">
      <div className="rounded-full bg-secondary p-4">
        <ScanSearch className="h-10 w-10 text-muted-foreground" />
      </div>
      <div className="space-y-2">
        <h1 className="text-2xl font-semibold tracking-tight">
          OneNote to Joplin Readiness
        </h1>
        <p className="text-muted-foreground text-sm leading-relaxed">
          Validates your Windows environment before running the OneNote to
          Joplin migration. Checks Joplin installation, Office versions,
          desktop applications, and COM automation access.
        </p>
      </div>
      {scanError && (
        <p className="text-sm text-destructive-foreground">{scanError}</p>
      )}
      <Button size="lg" onClick={startScan} className="gap-2">
        <ScanSearch className="h-5 w-5" />
        Run Readiness Scan
      </Button>
      <p className="text-xs text-muted-foreground">
        Ctrl+R to scan &middot; Ctrl+S to save report
      </p>
    </div>
  );
}
