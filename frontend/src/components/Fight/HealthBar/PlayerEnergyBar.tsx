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
      <div
        style={{ flexBasis: `${(energy * 100) / 10}%` }}
        className="h-full bg-zinc-400"
      />
    </div>
  );
};

export default PlayerEnergyBar;
