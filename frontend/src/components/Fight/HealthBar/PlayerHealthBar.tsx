import { times } from "lodash";

import { cn } from "../../../utils/classNameMerge";

interface Props {
  health: number;
  color: string;
  side: "left" | "right";
}

const PlayerHealthBar = ({ health, side, color }: Props) => {
  return (
    <div className="flex gap-2 w-full items-center justify-center">
      {times(5, (n) => {
        const is_primary_color =
          side === "left" ? health >= n : health >= 5 - n;
        return (
          <div
            className={cn("w-full bg-zinc-300 h-[8px] rounded-[2px]")}
            style={is_primary_color ? { backgroundColor: color } : {}}
          />
        );
      })}
    </div>
  );
};

export default PlayerHealthBar;
