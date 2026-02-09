import { ChevronLeft, ChevronRight, RotateCcw, X } from "lucide-react";
import { Button } from "../ui/button";

interface WizardNavProps {
  isFirst: boolean;
  isLast: boolean;
  onPrev: () => void;
  onNext: () => void;
  onExit: () => void;
  onRescan: () => void;
}

export function WizardNav({
  isFirst,
  isLast,
  onPrev,
  onNext,
  onExit,
  onRescan,
}: WizardNavProps) {
  return (
    <div className="flex items-center justify-between">
      <Button variant="ghost" size="sm" onClick={onExit} className="gap-1.5">
        <X className="h-3.5 w-3.5" />
        Back to Results
      </Button>
      <div className="flex gap-2">
        <Button
          variant="outline"
          size="sm"
          onClick={onPrev}
          disabled={isFirst}
          className="gap-1"
        >
          <ChevronLeft className="h-3.5 w-3.5" />
          Prev
        </Button>
        {isLast ? (
          <Button size="sm" onClick={onRescan} className="gap-1.5">
            <RotateCcw className="h-3.5 w-3.5" />
            Re-Scan
          </Button>
        ) : (
          <Button size="sm" onClick={onNext} className="gap-1">
            Next
            <ChevronRight className="h-3.5 w-3.5" />
          </Button>
        )}
      </div>
    </div>
  );
}
