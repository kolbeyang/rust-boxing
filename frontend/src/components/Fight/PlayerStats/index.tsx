import type { PlayerWeb } from "boxing-web";
import { cn } from "../../../utils/classNameMerge";
import PlayerControls from "./PlayerControls";
import NumericMetric from "./NumericMetric";
import RotationMetric from "./RotationMetric";

interface Props {
  className?: string;
  player: PlayerWeb;
}

const PlayerStats = ({ className, player }: Props) => {
  return (
    <div
      className={cn(
        "flex w-[220px] flex-col items-center justify-between gap-2",
        className,
      )}
    >
      <div className="flex flex-col gap-2 w-full">
        <NumericMetric label="X" value={player.position.x} />
        <NumericMetric label="Y" value={player.position.y} />
        <RotationMetric
          label="Velocity"
          angle={Math.atan2(player.velocity.y, player.velocity.x) - Math.PI / 2}
        />
        <RotationMetric label="Rotation" angle={player.rotation + Math.PI} />
      </div>
      <PlayerControls control={player.last_control} className="max-w-[160px]" />
    </div>
  );
};

export default PlayerStats;
