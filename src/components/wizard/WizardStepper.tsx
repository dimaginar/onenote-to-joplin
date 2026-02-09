import { cn } from "../../utils/cn";

interface WizardStepperProps {
  current: number;
  total: number;
  labels: string[];
}

export function WizardStepper({ current, total, labels }: WizardStepperProps) {
  return (
    <div className="flex items-center gap-2">
      {Array.from({ length: total }, (_, i) => (
        <div key={i} className="flex items-center gap-2">
          <div
            className={cn(
              "flex h-6 w-6 items-center justify-center rounded-full text-xs font-medium",
              i < current
                ? "bg-success/20 text-success"
                : i === current
                  ? "bg-primary text-primary-foreground"
                  : "bg-secondary text-muted-foreground"
            )}
          >
            {i + 1}
          </div>
          <span
            className={cn(
              "text-xs hidden sm:inline",
              i === current ? "text-foreground font-medium" : "text-muted-foreground"
            )}
          >
            {labels[i]}
          </span>
          {i < total - 1 && (
            <div className="h-px w-4 bg-border" />
          )}
        </div>
      ))}
    </div>
  );
}
