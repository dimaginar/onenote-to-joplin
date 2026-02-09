import { useReadinessScan } from "./hooks/useReadinessScan";
import { useKeyboard } from "./hooks/useKeyboard";
import { EmptyState } from "./components/scan/EmptyState";
import { ScanningState } from "./components/scan/ScanningState";
import { ResultsView } from "./components/results/ResultsView";
import { WizardView } from "./components/wizard/WizardView";
import { StatusBar } from "./components/layout/StatusBar";
import { cn } from "./utils/cn";

function App() {
  const { view } = useReadinessScan();
  useKeyboard();

  const isList = view === "results" || view === "wizard";

  return (
    <div className="h-screen flex flex-col bg-background text-foreground overflow-hidden">
      <main
        className={cn(
          "flex-1 flex justify-center overflow-hidden p-4",
          isList ? "items-start pt-6" : "items-center"
        )}
      >
        {view === "empty" && <EmptyState />}
        {view === "scanning" && <ScanningState />}
        {view === "results" && <ResultsView />}
        {view === "wizard" && <WizardView />}
      </main>
      <StatusBar />
    </div>
  );
}

export default App;
