import { Loader2 } from "lucide-react";

export function ScanningState() {
  return (
    <div className="flex flex-col items-center gap-6 text-center">
      <Loader2 className="h-12 w-12 animate-spin text-muted-foreground" />
      <div className="space-y-2">
        <h2 className="text-xl font-semibold">Scanning Environment</h2>
        <p className="text-sm text-muted-foreground">
          Checking Joplin installation, Windows version, Office apps, and COM access...
        </p>
      </div>
    </div>
  );
}
