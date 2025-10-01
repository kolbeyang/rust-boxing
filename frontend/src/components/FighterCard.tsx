import type { FighterWeb } from "boxing-web";
import { motion } from "framer-motion";

import { cn } from "../utils/classNameMerge";

interface Props {
  fighter: FighterWeb;
  isSelected: boolean;
  className?: string;
  onClick: () => void;
}

const FighterCard = ({ fighter, isSelected, className, onClick }: Props) => {
  const { name, description, number, color } = fighter;
  return (
    <motion.div
      className={cn(
        "flex flex-col bg-zinc-200 rounded-[4px] overflow-hidden",
        "hover:ring-1 hover:ring-zinc-400 hover:bg-zinc-100 hover:scale-105",
        className,
      )}
      variants={{
        hidden: { opacity: 0, translateY: 100 },
        visible: {
          opacity: 1,
          translateY: 0,
          transition: {
            ease: "linear",
            duration: 0.05,
            delayChildren: 0.2,
          },
        },
      }}
      onClick={onClick}
    >
      <div className="flex justify-between bg-white w-full h-[4px] shrink-0">
        <div
          className={cn("size-full", { "animate-blink": isSelected })}
          style={{ background: color }}
        />
      </div>
      <div
        className={cn(
          "flex w-full justify-between px-3 py-1 border-b border-b-zinc-700",
        )}
      >
        <span className="">{name.toUpperCase()}</span>
        <span className="">{number.toString().padStart(3, "0")}</span>
      </div>
      <div className="px-3 py-2 shrink-0">
        <motion.span
          className="inline-block"
          variants={{
            hidden: { opacity: 1, translateY: "400px" },
            visible: {
              opacity: 1,
              translateY: 0,
              transition: { ease: "linear", duration: 0.05 },
            },
          }}
        >{`"...${description}..."`}</motion.span>
      </div>
    </motion.div>
  );
};

export default FighterCard;
