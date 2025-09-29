import { useMemo } from "react";
import { motion } from "framer-motion";

import { FIGHTERS } from "../../utils/fighters";
import { cn } from "../../utils/classNameMerge";

interface Props {
  winnerNumber: number;
  onBack: () => void;
}

const WinScreen = ({ winnerNumber: winNumber, onBack }: Props) => {
  const winner = useMemo(() => {
    const winner = FIGHTERS.find((f) => f.number === winNumber);
    return winner;
  }, [winNumber]);

  return (
    <div className={cn("size-full flex flex-col justify-center items-center")}>
      <div className="flex flex-col gap-0 w-full items-center">
        <motion.span
          className={cn(
            "font-family-shoulders md:text-[120px] md:leading-[114px] font-stretch-extra-condensed font-semibold z-10",
            "text-[90px] leading-[84px]",
          )}
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5, ease: "easeOut" }}
        >
          {winner?.name.toUpperCase()}
        </motion.span>

        <motion.span
          className={cn(
            "font-family-shoulders md:text-[120px] md:leading-[114px] font-stretch-extra-condensed font-semibold z-10",
            "text-[90px] leading-[84px]",
          )}
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5, ease: "easeOut", delay: 0.2 }}
        >
          WINS
        </motion.span>
      </div>

      <motion.div
        className="flex gap-4 text-lg mt-8"
        initial={{
          translateY: "100%",
          opacity: 0,
        }}
        animate={{
          translateY: "0%",
          opacity: 1,
        }}
        transition={{ duration: 0.3, ease: "easeOut", delay: 0.5 }}
      >
        <button
          className="bg-zinc-700 px-8 py-4 text-white rounded-[4px] relative overflow-hidden"
          onClick={onBack}
        >
          <div className="absolute top-0 left-0 size-full animate-blink bg-[#FF3131] z-0" />
          <span className="z-20 relative">NEW MATCH</span>
        </button>
      </motion.div>
    </div>
  );
};

export default WinScreen;
