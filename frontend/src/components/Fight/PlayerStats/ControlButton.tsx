import type { ReactNode } from "react";
import { cn } from "../../../utils/classNameMerge";

interface Props {
  children?: ReactNode;
  isPressed: boolean;
  className?: string;
}

const ControlButton = ({ children, isPressed, className }: Props) => {
  return (
    <div
      className={cn(
        "flex aspect-square w-full justify-center items-center rounded-[4px] ring-1 ring-inset  ring-zinc-300",
        {
          "bg-zinc-200 ring-transparent": isPressed,
        },
        className,
      )}
    >
      {children}
    </div>
  );
};

export default ControlButton;
