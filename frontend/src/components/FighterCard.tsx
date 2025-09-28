import type { FighterWeb } from "boxing-web";

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
    <div
      className={cn(
        "flex flex-col bg-zinc-200 rounded-[4px] overflow-hidden",
        "hover:ring-1 hover:ring-zinc-400 hover:bg-zinc-100 hover:scale-105",
        { "ring-1 ring-offset-3 ring-zinc-700": isSelected },
        className,
      )}
      onClick={onClick}
    >
      <div
        className="flex justify-between text-white w-full h-[4px] shrink-0"
        style={{ background: color }}
      />
      <div className="flex w-full justify-between px-3 py-1 border-b border-b-zinc-700">
        <span className="">{name.toUpperCase()}</span>
        <span className="">{number.toString().padStart(3, "0")}</span>
      </div>
      <div className="px-3 py-2 shrink-0">{`"...${description}..."`}</div>
    </div>
  );
};

export default FighterCard;
