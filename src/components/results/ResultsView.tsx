import type { ReactNode } from "react";
import { Rocket } from "lucide-react";
import { useReadinessScan } from "../../hooks/useReadinessScan";
import { useReport } from "../../hooks/useReport";
import { useWizard } from "../../hooks/useWizard";
import { useAppStore } from "../../stores/appStore";
import { ResultsSummary } from "./ResultsSummary";
import { CheckListItem } from "./CheckListItem";
import { CheckDetail } from "./CheckDetail";
import { MigrationGuide } from "./MigrationGuide";
import type { CheckResult } from "../../stores/types";
import { cn } from "../../utils/cn";

const NEXT_STEPS_ID = "__next_steps__";

export function ResultsView() {
  const { scanResult, startScan } = useReadinessScan();
  const { canSave, saveReport } = useReport();
  const { failedChecks, enterWizard } = useWizard();
  const selectedCheckId = useAppStore((s) => s.selectedCheckId);
  const selectCheck = useAppStore((s) => s.selectCheck);

  if (!scanResult) return null;

  const grouped = groupByStatus(scanResult.checks);
  const selectedCheck =
    scanResult.checks.find((c) => c.id === selectedCheckId) ?? null;
  const allPassed = scanResult.overall === "pass";
  const showMigrationGuide =
    selectedCheckId === NEXT_STEPS_ID || selectedCheckId === null;

  return (
    <div className="w-full h-full flex flex-col rounded-xl border border-border bg-card overflow-hidden">
      <ResultsSummary
        result={scanResult}
        onRescan={startScan}
        onReport={saveReport}
        canSaveReport={canSave}
      />

      <div className="flex-1 min-h-0 grid grid-cols-[280px_1fr]">
        {/* Master: check list */}
        <aside className="border-r border-border overflow-y-auto scrollbar-thin flex flex-col">
          <div className="flex-1">
            {grouped.fail.length > 0 && (
              <Section
                label="Failed"
                count={grouped.fail.length}
                variant="fail"
              >
                {grouped.fail.map((c) => (
                  <CheckListItem
                    key={c.id}
                    check={c}
                    selected={c.id === selectedCheckId}
                    onSelect={() =>
                      selectCheck(c.id === selectedCheckId ? null : c.id)
                    }
                  />
                ))}
              </Section>
            )}
            {grouped.warning.length > 0 && (
              <Section
                label="Warnings"
                count={grouped.warning.length}
                variant="warning"
              >
                {grouped.warning.map((c) => (
                  <CheckListItem
                    key={c.id}
                    check={c}
                    selected={c.id === selectedCheckId}
                    onSelect={() =>
                      selectCheck(c.id === selectedCheckId ? null : c.id)
                    }
                  />
                ))}
              </Section>
            )}
            {grouped.pass.length > 0 && (
              <Section
                label="Passed"
                count={grouped.pass.length}
                variant="pass"
              >
                {grouped.pass.map((c) => (
                  <CheckListItem
                    key={c.id}
                    check={c}
                    selected={c.id === selectedCheckId}
                    onSelect={() =>
                      selectCheck(c.id === selectedCheckId ? null : c.id)
                    }
                  />
                ))}
              </Section>
            )}
            {grouped.skipped.length > 0 && (
              <Section
                label="Skipped"
                count={grouped.skipped.length}
                variant="skipped"
              >
                {grouped.skipped.map((c) => (
                  <CheckListItem
                    key={c.id}
                    check={c}
                    selected={c.id === selectedCheckId}
                    onSelect={() =>
                      selectCheck(c.id === selectedCheckId ? null : c.id)
                    }
                  />
                ))}
              </Section>
            )}
          </div>

          {/* Next Steps - pinned to bottom */}
          <div className="border-t border-border">
            <button
              onClick={() =>
                selectCheck(
                  selectedCheckId === NEXT_STEPS_ID ? null : NEXT_STEPS_ID
                )
              }
              className={cn(
                "relative flex items-center gap-2.5 px-3 py-2.5 w-full text-left transition-colors",
                "hover:bg-accent/50",
                showMigrationGuide &&
                  "bg-accent before:absolute before:left-0 before:top-0 before:bottom-0 before:w-1 before:rounded-r-sm before:bg-primary"
              )}
            >
              <Rocket
                className={cn(
                  "h-4 w-4 shrink-0",
                  allPassed ? "text-success" : "text-muted-foreground"
                )}
              />
              <div className="min-w-0 flex-1">
                <span className="text-sm font-medium block truncate">
                  Next Steps
                </span>
                <p className="text-xs text-muted-foreground truncate mt-0.5">
                  Migration guide
                </p>
              </div>
            </button>
          </div>
        </aside>

        {/* Detail panel */}
        <div className="overflow-y-auto scrollbar-thin">
          {showMigrationGuide ? (
            <MigrationGuide
              allPassed={allPassed}
              issueCount={failedChecks.length}
            />
          ) : (
            <CheckDetail
              check={selectedCheck}
              issueCount={failedChecks.length}
              onStartGuide={
                selectedCheck &&
                selectedCheck.status !== "pass" &&
                selectedCheck.status !== "skipped"
                  ? enterWizard
                  : undefined
              }
            />
          )}
        </div>
      </div>
    </div>
  );
}

function groupByStatus(checks: CheckResult[]) {
  return {
    fail: checks.filter((c) => c.status === "fail"),
    warning: checks.filter((c) => c.status === "warning"),
    pass: checks.filter((c) => c.status === "pass"),
    skipped: checks.filter((c) => c.status === "skipped"),
  };
}

const sectionColors = {
  fail: "text-destructive-foreground",
  warning: "text-warning",
  pass: "text-muted-foreground",
  skipped: "text-muted-foreground",
};

function Section({
  label,
  count,
  variant,
  children,
}: {
  label: string;
  count: number;
  variant: "fail" | "warning" | "pass" | "skipped";
  children: ReactNode;
}) {
  return (
    <section>
      <div className="px-3 py-2 text-xs font-semibold uppercase tracking-wider border-b border-border/50">
        <span className={sectionColors[variant]}>{label}</span>
        <span className="text-muted-foreground ml-1.5">({count})</span>
      </div>
      {children}
    </section>
  );
}
