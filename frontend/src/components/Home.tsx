import { useState } from "react";
import { isNil } from "lodash";
import { AnimatePresence } from "framer-motion";

import Fight from "./Fight";
import SelectFighters from "./SelectFighters";

const Home = () => {
  const [mode, setMode] = useState<"select" | "fight">("select");
  const [selectedFighterNumbers, setSelectedFighterNumbers] = useState<
    [number | null, number | null]
  >([null, null]);

  const onSelectFighterNumbers = (numbers: [number | null, number | null]) => {
    setSelectedFighterNumbers(numbers);
    if (!numbers.some(isNil)) {
      setMode("fight");
    }
  };

  const isBothFightersSelected =
    !isNil(selectedFighterNumbers[0]) && !isNil(selectedFighterNumbers[1]);

  return (
    <AnimatePresence mode="wait">
      {mode === "fight" && isBothFightersSelected ? (
        <Fight
          key="fight"
          f0Num={selectedFighterNumbers[0]!}
          f1Num={selectedFighterNumbers[1]!}
          goBack={() => setMode("select")}
        />
      ) : (
        <SelectFighters
          selectedFighterNumbers={selectedFighterNumbers}
          onSelectFighterNumbers={onSelectFighterNumbers}
        />
      )}
    </AnimatePresence>
  );
};

export default Home;
