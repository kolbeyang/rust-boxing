import { useState } from "react";

import MainScreen from "./MainScreen";
import TopBar from "./TopBar";
import WinScreen from "./WinScreen";

interface Props {
  f0Num: number;
  f1Num: number;
  goBack: () => void;
}

const Fight = ({ goBack, ...props }: Props) => {
  const [screen, setScreen] = useState<"main" | "end">("main");
  const [winnerNum, setWinnerNum] = useState<number | null>(null);

  const endFight = (winnerNum: number) => {
    setWinnerNum(winnerNum);
    setScreen("end");
  };

  return (
    <>
      {screen === "end" && (
        <WinScreen winnerNumber={winnerNum!} onBack={goBack} />
      )}

      {screen === "main" && (
        <div className="size-full px-4 py-3 flex flex-col items-center gap-[60px]">
          <TopBar goBack={goBack} />
          <MainScreen {...props} endFight={endFight} />
        </div>
      )}
    </>
  );
};

export default Fight;
