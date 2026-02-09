import type { ReactNode } from "react";
import { useReadinessScan } from "../../hooks/useReadinessScan";
import { useReport } from "../../hooks/useReport";
import { useWizard } from "../../hooks/useWizard";
import { useAppStore } from "../../stores/appStore";
import { ResultsSummary } from "./ResultsSummary";
import { CheckListItem } from "./CheckListItem";
import { CheckDetail } from "./CheckDetail";
import type { CheckResult } from "../../stores/types";

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
        <aside className="border-r border-border overflow-y-auto scrollbar-thin">
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
                  onSelect={() => selectCheck(c.id)}
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
                  onSelect={() => selectCheck(c.id)}
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
                  onSelect={() => selectCheck(c.id)}
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
                  onSelect={() => selectCheck(c.id)}
                />
              ))}
            </Section>
          )}
        </aside>

        {/* Detail panel */}
        <div className="overflow-y-auto scrollbar-thin">
          <CheckDetail
            check={selectedCheck}
            issueCount={failedChecks.length}
            onStartGuide={
              selectedCheck && selectedCheck.status !== "pass" && selectedCheck.status !== "skipped"
                ? enterWizard
                : undefined
            }
          />
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
