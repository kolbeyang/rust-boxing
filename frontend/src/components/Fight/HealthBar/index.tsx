import { IconHeart, IconHeartFilled } from "@tabler/icons-react";
import { cn } from "../../../utils/classNameMerge";
import PlayerHealthBar from "./PlayerHealthBar";
import PlayerEnergyBar from "./PlayerEnergyBar";

interface Props {
  p0Name?: string;
  p1Name?: string;
  p0Health?: number;
  p1Health?: number;
  p0Energy?: number;
  p1Energy?: number;
  className?: string;
}

const HealthBar = ({
  p0Name,
  p1Name,
  p0Energy,
  p1Health,
  p0Health,
  p1Energy,
  className,
}: Props) => {
  return (
    <div className={cn("flex w-full flex-col gap-1", className)}>
      <div className="flex justify-between w-full">
        <span>{p0Name}</span>
        <span>{p1Name}</span>
      </div>
      <div className="flex flex-col gap-[8px]">
        <div className="flex gap-8">
          <PlayerHealthBar
            health={p0Health ?? 0}
            color={"bg-[#FF3131]"}
            side={"left"}
          />
          <IconHeartFilled className="text-zinc-300 shrink-0" />
          <PlayerHealthBar
            health={p1Health ?? 0}
            color={"bg-[#414141]"}
            side={"right"}
          />
        </div>
        <div className="flex w-full justify-between">
          <PlayerEnergyBar
            energy={p0Energy ?? 0}
            side="left"
            className="basis-[30%]"
          />
          <PlayerEnergyBar
            energy={p1Energy ?? 0}
            side="right"
            className="basis-[30%]"
          />
        </div>
      </div>
    </div>
  );
};

export default HealthBar;
