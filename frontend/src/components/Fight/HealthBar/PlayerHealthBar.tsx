import { motion } from "framer-motion";
import { times } from "lodash";

import { cn } from "../../../utils/classNameMerge";

const containerVariants = {
  hidden: {},
  visible: {
    transition: {
      duration: 0.25,
      delay: 0.25,
      staggerChildren: 0.05,
    },
  },
  exit: {
    scaleX: 0.2,
    transition: {
      delay: 0.5,
    },
  },
};

const childVariants = {
  hidden: { scaleX: 0.2, opacity: 0.1 },
  visible: {
    scaleX: 1,
    opacity: 1,
    transition: {
      duration: 0.02,
    },
  },
};

interface Props {
  health: number;
  color: string;
  side: "left" | "right";
}

const PlayerHealthBar = ({ health, side, color }: Props) => {
  return (
    <motion.div
      key={`player-health-bar-${side}`}
      variants={containerVariants}
      initial="hidden"
      animate="visible"
      exit="exit"
      className={cn("gap-1", "flex w-full items-center justify-center", {
        "origin-left": side === "left",
        "origin-right flex-row-reverse": side === "right",
      })}
    >
      {times(5, (n) => {
        return (
          <motion.div
            key={n}
            variants={childVariants}
            className={cn("w-full bg-zinc-300 h-[8px] rounded-[2px]", {
              "origin-left": side === "left",
              "origin-right": side === "right",
            })}
            style={health > n ? { backgroundColor: color } : {}}
          />
        );
      })}
    </motion.div>
  );
};

export default PlayerHealthBar;
