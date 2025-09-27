import type { ReactNode } from "react";

interface Props {
  label: ReactNode;
  value: number;
}

const NumericMetric = ({ label, value }: Props) => {
  return (
    <div className="flex ring ring-inset ring-zinc-200 justify-between rounded-[4px] overflow-hidden">
      <div className="px-4 py-3">{label}</div>
      <div className="px-4 py-3 min-w-[100px] bg-zinc-200 text-right">
        {value.toFixed(2)}
      </div>
    </div>
  );
};

export default NumericMetric;
