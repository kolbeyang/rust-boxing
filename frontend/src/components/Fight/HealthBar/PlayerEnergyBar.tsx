import { motion } from "framer-motion";

import { cn } from "../../../utils/classNameMerge";

interface Props {
  energy: number;
  side: "left" | "right";
  className?: string;
}

const PlayerEnergyBar = ({ energy, side, className }: Props) => {
  return (
    <div
      // TODO: max energy should come from Rust
      className={cn(
        "flex gap-2 w-full items-center h-[4px] bg-zinc-300",
        side === "left" ? "justify-start" : "justify-end",
        className,
      )}
    >
      <motion.div
        initial={{ scaleX: "20%" }}
        animate={{
          scaleX: "100%",
        }}
        transition={{
          duration: 0.5,
        }}
        style={{ flexBasis: `${(energy * 100) / 10}%` }}
        className="h-full bg-zinc-400 origin-left"
      />
    </div>
  );
};

export default PlayerEnergyBar;
