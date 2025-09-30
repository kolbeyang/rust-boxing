import { motion } from "framer-motion";
import { useState } from "react";

import FightEndScreen from "./FightEndScreen";
import FightStartScreen from "./FightStartScreen";
import MainScreen from "./MainScreen";
import TopBar from "./TopBar";

interface Props {
  f0Num: number;
  f1Num: number;
  goBack: () => void;
  reset: () => void;
}

const Fight = ({ goBack, reset, ...props }: Props) => {
  const [screen, setScreen] = useState<"start" | "main" | "end">("start");
  const [winnerNum, setWinnerNum] = useState<number | null>(null);

  const endFight = (winnerNum: number) => {
    setWinnerNum(winnerNum);
    setScreen("end");
  };

  return (
    <>
      {screen === "start" && (
        <FightStartScreen
          f0Num={props.f0Num}
          f1Num={props.f1Num}
          key="win-screen"
          onBack={goBack}
          onFight={() => setScreen("main")}
        />
      )}

      {screen === "end" && (
        <FightEndScreen
          key="win-screen"
          winnerNumber={winnerNum!}
          onBack={reset}
        />
      )}

      {screen === "main" && (
        <motion.div
          key="main"
          className="size-full px-4 py-3 flex flex-col items-center gap-[60px]"
        >
          <TopBar goBack={goBack} />
          <MainScreen {...props} endFight={endFight} />
        </motion.div>
      )}
    </>
  );
};

export default Fight;
