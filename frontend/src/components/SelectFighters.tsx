import { FighterWeb } from "boxing-web";
import { isNil } from "lodash";
import { useState } from "react";

import { FIGHTERS } from "../utils/fighters";

import TopBar from "./Fight/TopBar";
import FighterCard from "./FighterCard";
import FighterIndicator from "./FighterIndicator";

interface Props {
  startFight: (fighter0Num: number, fighter1Num: number) => void;
}

const SelectFighters = ({ startFight: startFightProp }: Props) => {
  const [selectedFighter0, setSelectedFighter0] = useState<FighterWeb | null>(
    null,
  );
  const [selectedFighter1, setSelectedFighter1] = useState<FighterWeb | null>(
    null,
  );

  const [selectingState, setSelectingState] = useState<"p0" | "p1">("p0");
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
  const setSelectedFighter =
    selectingState === "p0" ? setSelectedFighter0 : setSelectedFighter1;

  return (
    <div className="size-full px-4 py-3 flex flex-col items-center gap-[60px]">
      <TopBar />
      <div className="flex items-center gap-5">
        <FighterIndicator
          fighter={selectedFighter0}
          isBlinking={selectingState === "p0"}
          onClick={() => setSelectingState("p0")}
        />
        <span>VS</span>
        <FighterIndicator
          fighter={selectedFighter1}
          isBlinking={selectingState === "p1"}
          onClick={() => setSelectingState("p1")}
        />
        <button disabled={isStartFightDisabled} onClick={startFight}>
          Fight
        </button>
      </div>
      <span>
        {selectingState === "p0" ? "Select Fighter 1" : "Select Fighter 2"}
      </span>
      <div className="grid grid-cols-3 w-full max-w-[960px] gap-8">
        {FIGHTERS.map((fighter) => (
          <FighterCard
            fighter={fighter}
            isSelected={selectedFighter?.number === fighter.number}
            onClick={() => setSelectedFighter(fighter)}
          />
        ))}
      </div>
    </div>
  );
};

export default SelectFighters;
