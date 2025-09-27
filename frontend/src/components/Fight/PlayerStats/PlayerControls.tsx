import {
  IconCaretDown,
  IconCaretLeft,
  IconCaretRight,
} from "@tabler/icons-react";
import type { Control } from "boxing-web";
import ControlButton from "./ControlButton";
import { cn } from "../../../utils/classNameMerge";

interface Props {
  control: Control;
  className?: string;
}

const PlayerControls = ({ control, className }: Props) => {
  return (
    <div className={cn("flex flex-col w-full gap-[4px]", className)}>
      <div className="flex gap-[4px]">
        <ControlButton className="flex-1" isPressed={control.left_punch}>
          L
        </ControlButton>

        <div
          className={"flex-1 aspect-square w-full justify-center items-center"}
        />
        <ControlButton className="flex-1" isPressed={control.right_punch}>
          R
        </ControlButton>
      </div>
      <div className="flex gap-[4px]">
        <ControlButton className="flex-1" isPressed={control.move_x === "Left"}>
          <IconCaretLeft size={28} stroke={1.25} />
        </ControlButton>
        <ControlButton className="flex-1" isPressed={control.move_y === "Back"}>
          <IconCaretDown size={28} stroke={1.25} />
        </ControlButton>

        <ControlButton
          className="flex-1"
          isPressed={control.move_x === "Right"}
        >
          <IconCaretRight size={28} stroke={1.25} />
        </ControlButton>
      </div>
    </div>
  );
};

export default PlayerControls;
