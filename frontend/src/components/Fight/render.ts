import type { GameStateWeb } from "boxing-web";

export const renderGame = (
  ctx: CanvasRenderingContext2D,
  gameState: GameStateWeb,
  p0Color: string,
  p1Color: string,
) => {
  ctx.clearRect(0, 0, 440, 440);

  // Scale factor to fit the game world (400x400) into canvas (440x440)
  const scale = 440 / 400;

  // Draw Player 0 (red)
  ctx.fillStyle = p0Color;
  ctx.beginPath();
  ctx.arc(
    gameState.player_0.position.x * scale,
    gameState.player_0.position.y * scale,
    26 * scale, // Player radius from your code
    0,
    2 * Math.PI,
  );
  ctx.fill();

  // Left fist
  ctx.beginPath();
  ctx.arc(
    gameState.player_0.fist_0.position.x * scale,
    gameState.player_0.fist_0.position.y * scale,
    15 * scale, // Fist radius from your code
    0,
    2 * Math.PI,
  );
  ctx.fill();

  // Right fist
  ctx.beginPath();
  ctx.arc(
    gameState.player_0.fist_1.position.x * scale,
    gameState.player_0.fist_1.position.y * scale,
    15 * scale,
    0,
    2 * Math.PI,
  );
  ctx.fill();

  // Draw Player 1 (blue)
  ctx.fillStyle = p1Color;
  ctx.beginPath();
  ctx.arc(
    gameState.player_1.position.x * scale,
    gameState.player_1.position.y * scale,
    26 * scale,
    0,
    2 * Math.PI,
  );
  ctx.fill();

  // Left fist
  ctx.beginPath();
  ctx.arc(
    gameState.player_1.fist_0.position.x * scale,
    gameState.player_1.fist_0.position.y * scale,
    15 * scale,
    0,
    2 * Math.PI,
  );
  ctx.fill();

  // Right fist
  ctx.beginPath();
  ctx.arc(
    gameState.player_1.fist_1.position.x * scale,
    gameState.player_1.fist_1.position.y * scale,
    15 * scale,
    0,
    2 * Math.PI,
  );
  ctx.fill();
};
