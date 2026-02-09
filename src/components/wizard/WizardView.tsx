import { useWizard } from "../../hooks/useWizard";
import { useReadinessScan } from "../../hooks/useReadinessScan";
import { WizardStepper } from "./WizardStepper";
import { WizardStep } from "./WizardStep";
import { WizardNav } from "./WizardNav";

export function WizardView() {
  const {
    wizardStep,
    failedChecks,
    currentCheck,
    isFirst,
    isLast,
    totalSteps,
    exitWizard,
    nextWizardStep,
    prevWizardStep,
  } = useWizard();
  const { startScan } = useReadinessScan();

  if (!currentCheck) return null;

  return (
    <div className="w-full max-w-lg flex flex-col gap-3 h-full">
      <WizardStepper
        current={wizardStep}
        total={totalSteps}
        labels={failedChecks.map((c) => c.label)}
      />
      <div className="flex-1 min-h-0 overflow-y-auto scrollbar-thin">
        <WizardStep
          check={currentCheck}
          stepNumber={wizardStep + 1}
          totalSteps={totalSteps}
        />
      </div>
      <WizardNav
        isFirst={isFirst}
        isLast={isLast}
        onPrev={prevWizardStep}
        onNext={nextWizardStep}
        onExit={exitWizard}
        onRescan={startScan}
      />
    </div>
  );
}
