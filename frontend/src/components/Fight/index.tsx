import * as wasm from "boxing-web";
import { useEffect, useMemo, useRef, useState } from "react";

import { FIGHTERS } from "../../utils/fighters";

import HealthBar from "./HealthBar";
import PlayerStats from "./PlayerStats";
import { renderGame } from "./render";
import TopBar from "./TopBar";

const targetFPS = 24;

interface Props {
  f0Num: number;
  f1Num: number;
  endFight: () => void;
}

const Fight = ({ f0Num, f1Num, endFight }: Props) => {
  const [game, setGame] = useState<wasm.Game | null>(null);
  const [gameState, setGameState] = useState<wasm.GameStateWeb | null>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const [fighter0, fighter1] = useMemo(() => {
    const fighter0 = FIGHTERS.find((f) => f.number === f0Num);
    const fighter1 = FIGHTERS.find((f) => f.number === f1Num);

    return [fighter0, fighter1];
  }, []);

  useEffect(() => {
    wasm.Game.new(f0Num, f1Num).then((game) => setGame(game));
  }, []);

  useEffect(() => {
    if (!game || !canvasRef.current) return;
    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Set canvas size
    canvas.width = 440;
    canvas.height = 440;

    let animationId: number;
    let lastTime = 0;
    let last_inference_time = 0;
    const frameTime = 1000 / targetFPS;

    const animate = (currentTime: number) => {
      const time_since_last_frame = currentTime - lastTime;
      last_inference_time = time_since_last_frame - frameTime;
      if (time_since_last_frame >= frameTime - last_inference_time) {
        const newState = game.step();
        if (newState.is_done) {
          endFight();
        }
        renderGame(
          ctx,
          newState,
          fighter0?.color ?? "#000",
          fighter1?.color ?? "#000",
        );
        setGameState(newState);
        lastTime = currentTime;
      }

      animationId = requestAnimationFrame(animate);
    };

    animationId = requestAnimationFrame(animate);

    return () => {
      cancelAnimationFrame(animationId);
    };
  }, [game]);

  return (
    <div className="size-full px-4 py-3 flex flex-col items-center gap-[60px]">
      <TopBar goBack={endFight} />
      <HealthBar
        p0Name={`${fighter0?.name.toUpperCase()}_${fighter0?.number.toString().padStart(3, "0")}`}
        p1Name={`${fighter1?.name.toUpperCase()}_${fighter1?.number.toString().padStart(3, "0")}`}
        p0Color={fighter0?.color}
        p1Color={fighter1?.color}
        p0Health={gameState?.player_0.health}
        p1Health={gameState?.player_1.health}
        p0Energy={gameState?.player_0.energy}
        p1Energy={gameState?.player_1.energy}
        className="max-w-[750px]"
      />
      <div className="w-full flex gap-[80px] justify-center">
        {gameState && <PlayerStats player={gameState?.player_0} />}
        <canvas
          ref={canvasRef}
          className="size-[440px] bg-zinc-200 rounded-[40px]"
        />
        {gameState && <PlayerStats player={gameState?.player_1} />}
      </div>
    </div>
  );
};

export default Fight;
