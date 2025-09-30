import { useMemo, useState } from "react";
import { motion } from "framer-motion";

import { cn } from "../utils/classNameMerge";
import { FIGHTERS } from "../utils/fighters";

import TopBar from "./Fight/TopBar";
import FighterCard from "./FighterCard";
import FighterIndicator from "./FighterIndicator";

interface Props {
  onSelectFighterNumbers: (s: [number | null, number | null]) => void;
  selectedFighterNumbers: [number | null, number | null];
}

const SelectFighters = ({
  selectedFighterNumbers,
  onSelectFighterNumbers,
}: Props) => {
  const [isGridScrolled, setIsGridScrolled] = useState(false);

  const [selectingState, setSelectingState] = useState<0 | 1>(
    selectedFighterNumbers[0] === null ? 0 : 1,
  );

  const selectedFighters = useMemo(
    () =>
      selectedFighterNumbers.map(
        (number) => FIGHTERS.find((f) => f.number === number) ?? null,
      ),
    [selectedFighterNumbers],
  );

  const selectedFighter = selectedFighters[selectingState];

  const setSelectedFighterNumber = (fighterNumber: number) => {
    if (selectingState === 0) {
      setSelectingState(1);
    }

    const newSelectedFighterNumbers = [...selectedFighterNumbers] as [
      number | null,
      number | null,
    ];
    newSelectedFighterNumbers[selectingState] = fighterNumber;
    onSelectFighterNumbers(newSelectedFighterNumbers);
  };

  return (
    <div className="size-full px-2 md:px-4 pt-3 flex flex-col items-center relative">
      <TopBar />
      <motion.div
        initial={{ translateY: "-100px", opacity: 0 }}
        animate={{ translateY: "0", opacity: 1 }}
        transition={{ duration: 0.1 }}
        className={cn(
          "flex-col gap-3 pt-[30px] pb-[20px]",
          "flex md:flex-row items-center md:gap-5 md:pt-[60px] md:pb-[40px]",
        )}
      >
        <FighterIndicator
          fighter={selectedFighters[0]}
          isBlinking={selectingState === 0}
          onClick={() => setSelectingState(0)}
          className={cn("transition-all duration-100")}
        />
        <span className={cn("transition-all duration-100")}>VS</span>

        <FighterIndicator
          fighter={selectedFighters[1]}
          isBlinking={selectingState === 1}
          onClick={() => setSelectingState(1)}
          className={cn("transition-all duration-100")}
        />
      </motion.div>
      <div
        className={cn(
          "w-full flex-1 overflow-hidden flex flex-col items-center",
        )}
      >
        <div
          className={cn("h-full w-fit transition-all duration-200 relative")}
        >
          <div
            className={cn(
              "absolute w-0 border-t transition-all duration-200 border-t-zinc-400 translate-x-[-50%] left-1/2 z-20 h-[2px] top-0 opacity-0",
              {
                "w-full opacity-100": isGridScrolled,
              },
            )}
          />
          <motion.div
            className="grid grid-cols-2 md:grid-cols-3 w-full max-w-[960px] gap-x-0 gap-y-8 overflow-auto items-start min-h-0 auto-rows-min scrollbar-hide h-full pt-[40px] pb-[60px]"
            initial="hidden"
            animate="visible"
            variants={{
              visible: {
                transition: {
                  duration: 0.05,
                  delayChildren: 0.15,
                  staggerChildren: 0.05,
                },
              },
            }}
            onScroll={(e) =>
              setIsGridScrolled(e.currentTarget.scrollTop > 40.0)
            }
          >
            {FIGHTERS.map((fighter) => (
              <FighterCard
                key={fighter.number}
                fighter={fighter}
                isSelected={selectedFighter?.number === fighter.number}
                onClick={() => setSelectedFighterNumber(fighter.number)}
                className="shrink-0 m-2 md:m-4 h-full"
              />
            ))}
          </motion.div>
        </div>
      </div>
    </div>
  );
};

export default SelectFighters;
