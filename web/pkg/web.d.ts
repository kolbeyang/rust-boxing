/* tslint:disable */
/* eslint-disable */
export function greet(): void;
export function get_fighters(): FighterWeb[];
export enum FistStateWeb {
  Resting = 0,
  Extending = 1,
  Retracting = 2,
}
export type MoveX = "Left" | "None" | "Right";

export type MoveY = "Back" | "None";

export interface Control {
    move_x: MoveX;
    move_y: MoveY;
    left_punch: boolean;
    right_punch: boolean;
}

export class FighterWeb {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  number: number;
  readonly name: string;
  readonly description: string;
  readonly color: string;
}
export class FistWeb {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  position: Point;
  state: FistStateWeb;
}
export class Game {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static new(player0_number: number, player1_number: number): Promise<Game>;
  step(): GameStateWeb;
}
export class GameStateWeb {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  player_0: PlayerWeb;
  player_1: PlayerWeb;
  is_done: boolean;
}
export class PlayerWeb {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  position: Point;
  rotation: number;
  velocity: Point;
  health: number;
  energy: number;
  fist_0: FistWeb;
  fist_1: FistWeb;
  last_control: Control;
}
export class Point {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
}
