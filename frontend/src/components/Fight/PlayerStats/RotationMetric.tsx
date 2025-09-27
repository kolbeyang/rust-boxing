import type { ReactNode } from "react";

interface Props {
  label: ReactNode;
  angle: number;
}

const RotationMetric = ({ label, angle }: Props) => {
  // Convert radians to degrees for CSS transform
  const angleDegrees = (angle * 180) / Math.PI;

  return (
    <div className="flex ring ring-inset ring-zinc-200 justify-between rounded-[4px] overflow-hidden">
      <div className="px-4 py-3">{label}</div>
      <div className="bg-zinc-200 text-right h-full aspect-square relative  justify-center items-center flex p-2">
        <div className="bg-zinc-200 text-right h-full aspect-square relative ring ring-[#414141] rounded-full">
          {/* Dial line */}
          <div
            className="absolute left-[50%] top-[50%]"
            style={{
              width: 1,
              height: "50%", // Half the dial size minus some padding
              backgroundColor: "#414141",
              transformOrigin: "top center",
              transform: `rotate(${angleDegrees}deg)`,
            }}
          />
        </div>
      </div>
    </div>
  );
};

export default RotationMetric;
