import {
  Rocket,
  ExternalLink,
  AlertTriangle,
  BookOpen,
} from "lucide-react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { cn } from "../../utils/cn";

interface MigrationGuideProps {
  allPassed: boolean;
  issueCount: number;
}

export function MigrationGuide({ allPassed, issueCount }: MigrationGuideProps) {
  return (
    <div className="p-4 space-y-3">
      {/* Header */}
      <div className="flex items-start gap-3">
        <div
          className={cn(
            "rounded-full p-2",
            allPassed ? "bg-success/10" : "bg-warning/10"
          )}
        >
          <Rocket
            className={cn(
              "h-5 w-5",
              allPassed ? "text-success" : "text-warning"
            )}
          />
        </div>
        <div>
          <h3 className="text-base font-semibold">
            {allPassed ? "Ready to Migrate" : "Migration Guide"}
          </h3>
          <p className="text-sm text-muted-foreground mt-0.5">
            {allPassed
              ? "All checks passed \u2014 follow these steps to migrate your notebooks"
              : `${issueCount} issue${issueCount !== 1 ? "s" : ""} remaining \u2014 you can preview the steps ahead`}
          </p>
        </div>
      </div>

      {/* Warning when issues remain */}
      {!allPassed && (
        <div className="rounded-lg border border-warning/20 bg-warning/5 p-3 flex items-start gap-2.5">
          <AlertTriangle className="h-4 w-4 text-warning shrink-0 mt-0.5" />
          <p className="text-sm text-warning">
            Resolve all failed checks before starting the migration to avoid
            errors during export.
          </p>
        </div>
      )}

      {/* Steps */}
      <div className="space-y-2">
        <h4 className="text-sm font-medium">Migration Steps</h4>

        <ol className="space-y-1.5">
          <Step
            number={1}
            title="Download OneNote Md Exporter"
            description="Get the exporter tool from GitHub."
            link={{
              label: "onenote-md-exporter",
              url: "https://github.com/alxnbl/onenote-md-exporter",
            }}
          />
          <Step
            number={2}
            title='Export using "Joplin Raw Directory" format'
            description="Run the exporter, choose Joplin Raw Directory output, and note the export folder path."
          />
          <Step
            number={3}
            title="Import into Joplin"
            description='In Joplin, click File &gt; Import &gt; "RAW - Joplin Export Directory" and select the export folder.'
          />
        </ol>
      </div>

      {/* Detailed guide link */}
      <div className="rounded-lg border border-border bg-secondary/30 p-3 flex items-start gap-2.5">
        <BookOpen className="h-5 w-5 text-muted-foreground shrink-0 mt-0.5" />
        <div>
          <p className="text-sm font-medium">Detailed Migration Guide</p>
          <p className="text-sm text-muted-foreground mt-0.5">
            Covers supported features, known limitations, and advanced options.
          </p>
          <a
            href="https://github.com/alxnbl/onenote-md-exporter/blob/main/doc/migration-to-joplin.md"
            onClick={(e) => {
              e.preventDefault();
              openUrl("https://github.com/alxnbl/onenote-md-exporter/blob/main/doc/migration-to-joplin.md");
            }}
            className="inline-flex items-center gap-1.5 text-sm font-medium text-primary hover:underline mt-1.5"
          >
            View on GitHub
            <ExternalLink className="h-3.5 w-3.5" />
          </a>
        </div>
      </div>
    </div>
  );
}

function Step({
  number,
  title,
  description,
  link,
}: {
  number: number;
  title: string;
  description: string;
  link?: { label: string; url: string };
}) {
  return (
    <li className="rounded-lg border border-border bg-secondary/30 px-3 py-2 flex items-start gap-2.5">
      <div className="flex items-center justify-center h-5 w-5 rounded-full bg-primary text-primary-foreground text-[11px] font-bold shrink-0 mt-px">
        {number}
      </div>
      <div className="min-w-0 flex-1">
        <p className="text-sm font-medium">{title}</p>
        <p className="text-sm text-muted-foreground mt-0.5">
          {description}
        </p>
        {link && (
          <a
            href={link.url}
            onClick={(e) => {
              e.preventDefault();
              openUrl(link.url);
            }}
            className="inline-flex items-center gap-1 text-sm text-primary hover:underline mt-1"
          >
            {link.label}
            <ExternalLink className="h-3 w-3" />
          </a>
        )}
      </div>
    </li>
  );
}
