import { useAppStore } from "../stores/appStore";

export function useWizard() {
  const wizardStep = useAppStore((s) => s.wizardStep);
  const failedChecks = useAppStore((s) => s.failedChecks);
  const enterWizard = useAppStore((s) => s.enterWizard);
  const exitWizard = useAppStore((s) => s.exitWizard);
  const nextWizardStep = useAppStore((s) => s.nextWizardStep);
  const prevWizardStep = useAppStore((s) => s.prevWizardStep);

  const currentCheck = failedChecks[wizardStep] ?? null;
  const isFirst = wizardStep === 0;
  const isLast = wizardStep >= failedChecks.length - 1;
  const totalSteps = failedChecks.length;

  return {
    wizardStep,
    failedChecks,
    currentCheck,
    isFirst,
    isLast,
    totalSteps,
    enterWizard,
    exitWizard,
    nextWizardStep,
    prevWizardStep,
  };
}
