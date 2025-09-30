import type { FighterWeb } from "boxing-web";

import { cn } from "../utils/classNameMerge";

interface Props {
  fighter: FighterWeb | null;
  isBlinking: boolean;
  onClick: () => void;
  className?: string;
}

const FighterIndicator = ({
  fighter,
  isBlinking,
  onClick,
  className,
}: Props) => {
  return (
    <div
      className={cn(
        "w-[300px] h-[96px]",
        "flex justify-center md:w-[360px] md:h-[116px] items-center bg-zinc-200 rounded-[4px] overflow-hidden relative",
        "hover:scale-105 hover:ring-1 hover:ring-zinc-700",
        className,
      )}
      onClick={onClick}
    >
      <span className="font-family-shoulders font-stretch-extra-condensed font-semibold absolute text-transparent">
        FORCE FONT LOAD
      </span>
      {/*Blinker*/}
      {isBlinking && (
        <div className="bg-zinc-400/50 size-full top-0 left-0 absolute animate-blink ring-inset ring-1 ring-zinc-400 rounded-[4px]" />
      )}
      {fighter ? (
        <div
          className={cn(
            "px-2",
            "size-full flex justify-between items-start md:px-3 py-2",
          )}
        >
          <span
            className={cn(
              "text-[90px] leading-[84px]",
              "font-family-shoulders md:text-[110px] md:leading-[104px] font-stretch-extra-condensed font-semibold z-10",
            )}
          >
            {fighter.name.toUpperCase()}
          </span>
          <span className="z-10">
            {fighter.number.toString().padStart(3, "0")}
          </span>
        </div>
      ) : (
        <span className="flex justify-center items-center z-10">
          Select fighter
        </span>
      )}
    </div>
  );
};

export default FighterIndicator;
