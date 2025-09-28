import type { FighterWeb } from "boxing-web";

import { cn } from "../utils/classNameMerge";

interface Props {
  fighter: FighterWeb | null;
  className?: string;
}

const StaticFighterIndicator = ({ fighter, className }: Props) => {
  return (
    <div
      className={cn(
        "w-[300px] h-[96px]",
        "flex justify-center md:w-[400px] md:h-[128px] items-center bg-zinc-200 rounded-[4px] overflow-hidden relative",

        className,
      )}
    >
      <span className="font-family-shoulders font-stretch-extra-condensed font-semibold absolute text-transparent">
        FORCE FONT LOAD
      </span>
      <div
        className={cn("size-full flex justify-between items-start px-3 py-2")}
      >
        <span
          className={cn(
            "font-family-shoulders md:text-[120px] md:leading-[114px] font-stretch-extra-condensed font-semibold z-10",
            "text-[90px] leading-[84px]",
          )}
        >
          {fighter?.name.toUpperCase()}
        </span>
        <span className="z-10">
          {fighter?.number.toString().padStart(3, "0")}
        </span>
      </div>
    </div>
  );
};

export default StaticFighterIndicator;
