import type { FighterWeb } from "boxing-web";

import { cn } from "../utils/classNameMerge";

interface Props {
  fighter: FighterWeb | null;
  isBlinking: boolean;
  onClick: () => void;
}

const FighterIndicator = ({ fighter, isBlinking, onClick }: Props) => {
  return (
    <div
      className={cn(
        "flex justify-center w-[400px] h-[128px] items-center bg-zinc-200 rounded-[4px] overflow-hidden relative",
        "hover:scale-105 hover:ring-1 hover:ring-zinc-700",
      )}
      onClick={onClick}
    >
      <span className="font-family-shoulders font-stretch-extra-condensed font-semibold absolute text-transparent">
        FORCE FONT LOAD
      </span>
      {/*Blinker*/}
      {isBlinking && (
        <div className="bg-zinc-400/50 size-full top-0 left-0 absolute animate-blink" />
      )}
      {fighter ? (
        <div
          className={cn("size-full flex justify-between items-start px-3 py-2")}
        >
          <span className="font-family-shoulders text-[120px] leading-[114px] font-stretch-extra-condensed font-semibold z-10">
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
