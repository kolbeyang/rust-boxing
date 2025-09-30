import { motion } from "framer-motion";
import { useMemo } from "react";

import { FIGHTERS } from "../../utils/fighters";
import StaticFighterIndicator from "../StaticFighterIndicator";

interface Props {
  f0Num: number;
  f1Num: number;
  onBack: () => void;
  onFight: () => void;
}

const FightStartScreen = ({ f0Num, f1Num, onBack, onFight }: Props) => {
  const [fighter0, fighter1] = useMemo(() => {
    const fighter0 = FIGHTERS.find((f) => f.number === f0Num);
    const fighter1 = FIGHTERS.find((f) => f.number === f1Num);

    return [fighter0, fighter1];
  }, [f0Num, f1Num]);

  return (
    <motion.div
      key="fight-start"
      className="flex flex-col absolute top-1/2 left-1/2 -translate-x-[50%] -translate-y-[50%] gap-12 items-center"
    >
      <div className="flex flex-col gap-4 w-full items-center">
        <motion.div
          key="p0-indicator"
          initial={{ x: "-100vw", opacity: 0 }}
          animate={{ x: 0, opacity: 1 }}
          exit={{ x: "-100vw", opacity: 0 }}
          transition={{ duration: 0.2 }}
        >
          <StaticFighterIndicator fighter={fighter0 ?? null} />
        </motion.div>

        <motion.span
          key="vs"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.1 }}
        >
          VS
        </motion.span>

        <motion.div
          key="p1-indicator"
          initial={{ x: "100vw", opacity: 0 }}
          animate={{ x: 0, opacity: 1 }}
          exit={{ x: "100vw", opacity: 0 }}
          transition={{ duration: 0.2 }}
        >
          <StaticFighterIndicator fighter={fighter1 ?? null} />
        </motion.div>
      </div>

      <motion.div
        key="button-box"
        className="flex gap-4 text-lg"
        initial={{ y: "100%", opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        exit={{ y: "100%", opacity: 0 }}
        transition={{ duration: 0.2 }}
      >
        <button
          className="ring-zinc-700 ring-1 px-8 py-4 rounded-[4px] hover:bg-zinc-200"
          onClick={onBack}
        >
          BACK
        </button>
        <button
          className="bg-zinc-700 px-8 py-4 text-white rounded-[4px] relative overflow-hidden"
          onClick={onFight}
        >
          <div className="absolute top-0 left-0 size-full animate-blink bg-[#FF3131] z-0" />
          <span className="z-20 relative">FIGHT</span>
        </button>
      </motion.div>
    </motion.div>
  );
};

export default FightStartScreen;
