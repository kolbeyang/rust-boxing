import { times } from "lodash";
import { motion } from "framer-motion";

import { cn } from "../../../utils/classNameMerge";

interface Props {
  health: number;
  color: string;
  side: "left" | "right";
}

const PlayerHealthBar = ({ health, side, color }: Props) => {
  return (
    <motion.div
      initial={{ scaleX: "20%" }}
      animate={{
        scaleX: "100%",
      }}
      transition={{
        duration: 0.25,
        delay: 0.25,
      }}
      className={cn(
        "gap-1",
        "flex md:gap-2 w-full items-center justify-center",
        {
          "origin-left": side === "left",
          "origin-right": side === "right",
        },
      )}
    >
      {times(5, (n) => {
        const is_primary_color =
          side === "left" ? health >= n : health >= 5 - n;
        return (
          <motion.div
            initial={{ scaleX: "20%", opacity: "10%" }}
            animate={{
              scaleX: "100%",
              opacity: "100%",
            }}
            transition={{
              duration: 0.5,
              delay: 0.25,
            }}
            className={cn("w-full bg-zinc-300 h-[8px] rounded-[2px]", {
              "origin-left": side === "left",
              "origin-right": side === "right",
            })}
            style={is_primary_color ? { backgroundColor: color } : {}}
          />
        );
      })}
    </motion.div>
  );
};

export default PlayerHealthBar;
