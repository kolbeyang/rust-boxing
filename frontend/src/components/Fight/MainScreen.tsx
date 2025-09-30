import * as wasm from "boxing-web";
import { motion, useAnimationControls } from "framer-motion";
import { useEffect, useMemo, useRef, useState } from "react";

import { FIGHTERS } from "../../utils/fighters";

import HealthBar from "./HealthBar";
import PlayerStats from "./PlayerStats";
import { renderGame } from "./render";

const TARGET_FPS = 48;

const ENDGAME_DELAY = 2000; // ms

interface Props {
  f0Num: number;
  f1Num: number;
  endFight: (winnerNum: number) => void;
}

const MainScreen = ({ f0Num, f1Num, endFight: endFightProp }: Props) => {
  const [game, setGame] = useState<wasm.Game | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [gameState, setGameState] = useState<wasm.GameStateWeb | null>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const controls = useAnimationControls();

  const [fighter0, fighter1] = useMemo(() => {
    const fighter0 = FIGHTERS.find((f) => f.number === f0Num);
    const fighter1 = FIGHTERS.find((f) => f.number === f1Num);

    return [fighter0, fighter1];
  }, []);

  const endFight = (winnerNum: number) => {
    setTimeout(() => endFightProp(winnerNum), ENDGAME_DELAY);
  };

  useEffect(() => {
    wasm.Game.new(f0Num, f1Num).then((game) => {
      setGame(game);
      // TODO: find a way to get initial state
      setGameState(game.step());
    });
  }, []);

  const showHit = async (x: number, y: number, color: string) => {
    console.log("showHit!");
    await controls.start({
      scale: [0.25, 1, 0.75],
      left: [x],
      top: [y],
      background: [`radial-gradient(circle, transparent 50% ,${color}20 70%)`],
      borderWidth: [5, 40, 0],
      borderColor: [color],
      opacity: [0.2, 0.25, 0],
      transition: {
        duration: 0.5,
        ease: "easeInOut",
      },
    });
  };

  useEffect(() => {
    if (!game || !canvasRef.current || isLoading) return;
    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Set canvas size
    canvas.width = 440;
    canvas.height = 440;

    let animationId: number;
    let lastTime = 0;
    let last_inference_time = 0;
    const frameTime = 1000 / TARGET_FPS;

    const animate = (currentTime: number) => {
      const time_since_last_frame = currentTime - lastTime;
      last_inference_time = time_since_last_frame - frameTime;
      if (time_since_last_frame >= frameTime - last_inference_time) {
        const newState = game.step();
        renderGame(
          ctx,
          newState,
          fighter0?.color ?? "#000",
          fighter1?.color ?? "#000",
        );

        setGameState((prev) => {
          // Handle players getting hit effect
          if (newState.player_0.health < (prev?.player_0.health ?? 0)) {
            console.log(newState.player_0.health, prev?.player_0.health);
            const position = newState.player_0.position;

            showHit(position.x, position.y, fighter0?.color ?? "#000");
          }
          if (newState.player_1.health < (prev?.player_1.health ?? 0)) {
            console.log(newState.player_1.health, prev?.player_1.health);
            const position = newState.player_1.position;
            showHit(position.x, position.y, fighter1?.color ?? "#000");
          }

          return newState;
        });
        lastTime = currentTime;

        if (newState.is_done) {
          const winnerNum =
            newState.player_0.health < newState.player_1.health ? f1Num : f0Num;
          endFight(winnerNum);
          return;
        }
      }

      animationId = requestAnimationFrame(animate);
    };

    animationId = requestAnimationFrame(animate);

    return () => {
      cancelAnimationFrame(animationId);
    };
  }, [game, isLoading]);

  return (
    <>
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
        {gameState && (
          <PlayerStats
            player={gameState?.player_0}
            className="hidden lg:flex"
            side="left"
          />
        )}
        <motion.div
          key="ring-wrapper"
          className="size-[380px] md:size-[440px] rounded-[44px] relative"
          initial={{ backgroundColor: "#00000000" }}
          animate={{ backgroundColor: "#e4e4e7" }}
          exit={{ backgroundColor: "#00000000" }}
          transition={{ delay: 0.5 }}
        >
          <motion.div
            className="size-[500px] opacity-0 absolute -translate-[50%] rounded-full"
            animate={controls}
          />

          <motion.svg
            className="size-full absolute -z-10"
            viewBox="0 0 400 400"
            initial="hidden"
            animate="visible"
          >
            <motion.rect
              key="outline-animator"
              className="size-[calc(100%-1px)]"
              x="0.5"
              y="0.5"
              rx="42"
              onAnimationComplete={() => setIsLoading(false)}
              custom={3}
              style={{
                strokeWidth: 1,
                strokeLinecap: "square",
                fill: "transparent",
              }}
              initial={{ pathLength: 0, opacity: 0, stroke: "#3f3f46" }}
              animate={{
                pathLength: 1,
                opacity: 1,
                stroke: "#a1a1aa",
              }}
              exit={{ pathLength: 0, opacity: 0, stroke: "#3f3f46" }}
              transition={{ duration: 0.5 }}
            />
          </motion.svg>
          <motion.canvas
            initial={{ visibility: "hidden" }}
            animate={{ visibility: "visible" }}
            transition={{ delay: 0.5 }}
            ref={canvasRef}
            className="size-full bg-zinc-200 rounded-[40px]"
          />
        </motion.div>
        {gameState && (
          <PlayerStats
            player={gameState?.player_1}
            className="hidden lg:flex"
            side="right"
          />
        )}
      </div>
    </>
  );
};

export default MainScreen;
