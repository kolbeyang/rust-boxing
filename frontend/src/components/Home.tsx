import { useState } from "react";
import { isNil } from "lodash";

import Fight from "./Fight";
import SelectFighters from "./SelectFighters";

const Home = () => {
  const [selectedFighterNumbers, setSelectedFighterNumbers] = useState<
    [number, number] | null
  >(null);

  const goBack = () => {
    setSelectedFighterNumbers(null);
  };

  const startFight = (f0Num: number, f1Num: number) => {
    setSelectedFighterNumbers([f0Num, f1Num]);
  };

  return !isNil(selectedFighterNumbers) ? (
    <>
      <Fight
        f0Num={selectedFighterNumbers[0]}
        f1Num={selectedFighterNumbers[1]}
        goBack={goBack}
      />
    </>
  ) : (
    <SelectFighters startFight={startFight} />
  );
};

export default Home;
