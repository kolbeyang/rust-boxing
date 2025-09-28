import { FighterWeb } from "boxing-web";
import { isNil } from "lodash";
import { useState } from "react";

import { FIGHTERS } from "../utils/fighters";
import { cn } from "../utils/classNameMerge";

import TopBar from "./Fight/TopBar";
import FighterCard from "./FighterCard";
import FighterIndicator from "./FighterIndicator";
import StaticFighterIndicator from "./StaticFighterIndicator";

interface Props {
  startFight: (fighter0Num: number, fighter1Num: number) => void;
}

const SelectFighters = ({ startFight: startFightProp }: Props) => {
  const [isGridScrolled, setIsGridScrolled] = useState(false);
  const [selectedFighter0, setSelectedFighter0] = useState<FighterWeb | null>(
    null,
  );
  const [selectedFighter1, setSelectedFighter1] = useState<FighterWeb | null>(
    null,
  );

  const [selectingState, setSelectingState] = useState<"p0" | "p1" | "fight">(
    "p0",
  );
  const isStartFightDisabled =
    isNil(selectedFighter0) || isNil(selectedFighter1);

  const startFight = () => {
    if (!selectedFighter0 || !selectedFighter1) {
      return;
    }
    startFightProp(selectedFighter0.number, selectedFighter1.number);
  };

  const selectedFighter =
    selectingState === "p0" ? selectedFighter0 : selectedFighter1;
  const setSelectedFighter = (fighter: FighterWeb) => {
    if (selectingState === "p0") {
      setSelectedFighter0(fighter);
      setSelectingState("p1");
    } else {
      setSelectedFighter1(fighter);
      setSelectingState("fight");
    }
  };

  return (
    <div className="size-full px-2 md:px-4 pt-3 flex flex-col items-center relative">
      <TopBar />
      <div
        className={cn(
          "flex flex-col absolute top-1/2 left-1/2 -translate-x-[50%] -translate-y-[50%] gap-12 opacity-0 items-center pointer-events-none",
          { "opacity-100 pointer-events-auto": selectingState === "fight" },
        )}
      >
        <div className="flex flex-col gap-4 w-full items-center ">
          <StaticFighterIndicator
            fighter={selectedFighter0}
            className={cn(
              "translate-x-[-100vw] transition-all duration-300 opacity-0",
              {
                "translate-x-0 opacity-100": selectingState === "fight",
              },
            )}
          />
          <span className={cn("transition-all duration-100")}>VS</span>
          <StaticFighterIndicator
            fighter={selectedFighter1}
            className={cn(
              "translate-x-[100vw] transition-all duration-300 opacity-0",
              {
                "translate-x-0 opacity-100": selectingState === "fight",
              },
            )}
          />
        </div>
        <div
          className={cn(
            "flex gap-4 text-lg",
            "translate-y-[100%] transition-all duration-300 opacity-0",
            {
              "translate-y-0 opacity-100": selectingState === "fight",
            },
          )}
        >
          <button
            className="ring-zinc-700 ring-1 px-8 py-4 rounded-[4px] hover:bg-zinc-200"
            onClick={() => setSelectingState("p1")}
          >
            BACK
          </button>
          <button
            className="bg-zinc-700 px-8 py-4 text-white rounded-[4px] relative overflow-hidden"
            disabled={isStartFightDisabled}
            onClick={startFight}
          >
            <div className="absolute top-0 left-0 size-full animate-blink bg-[#FF3131] z-0" />
            <span className="z-20 relative">FIGHT</span>
          </button>
        </div>
      </div>
      <div
        className={cn(
          "flex-col gap-3 pt-[30px] pb-[20px]",
          "flex md:flex-row items-center md:gap-5 md:pt-[60px] md:pb-[40px]",
          {
            "pointer-events-none": selectingState === "fight",
          },
        )}
      >
        <FighterIndicator
          fighter={selectedFighter0}
          isBlinking={selectingState === "p0"}
          onClick={() => setSelectingState("p0")}
          className={cn("transition-all duration-100", {
            "scale-95 opacity-0": selectingState === "fight",
          })}
        />
        <span
          className={cn("transition-all duration-100", {
            "scale-95 opacity-0": selectingState === "fight",
          })}
        >
          VS
        </span>

        <FighterIndicator
          fighter={selectedFighter1}
          isBlinking={selectingState === "p1"}
          onClick={() => setSelectingState("p1")}
          className={cn("transition-all duration-100", {
            "scale-95 opacity-0": selectingState === "fight",
          })}
        />
      </div>
      <div
        className={cn(
          "w-full flex-1 overflow-hidden flex flex-col items-center",
        )}
      >
        <div
          className={cn(
            "h-full w-fit transition-all duration-200 border-t border-transparent relative",
            {
              "translate-y-[100vh]": selectingState === "fight",
              "border-t-zinc-700": isGridScrolled,
            },
          )}
        >
          <div
            className="grid grid-cols-2 md:grid-cols-3 w-full max-w-[960px] gap-x-0 gap-y-8 overflow-auto items-start min-h-0 auto-rows-min scrollbar-hide h-full pt-[40px] pb-[60px]"
            onScroll={(e) =>
              setIsGridScrolled(e.currentTarget.scrollTop > 40.0)
            }
          >
            {FIGHTERS.map((fighter) => (
              <FighterCard
                fighter={fighter}
                isSelected={selectedFighter?.number === fighter.number}
                onClick={() => setSelectedFighter(fighter)}
                className="shrink-0 m-2 md:m-4 h-full"
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default SelectFighters;
