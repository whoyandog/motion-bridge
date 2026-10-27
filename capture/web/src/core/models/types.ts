export interface IRLandmark {
  x: number;
  y: number;
  z: number;
  visibility?: number;
}

export interface IModelProvider {
  readonly name: string;

  initialize(): Promise<void>;

  predict(
    video: HTMLVideoElement,
    timestamp: number,
  ): Promise<IRLandmark[] | null>;

  draw(ctx: CanvasRenderingContext2D): void;

  dispose(): Promise<void>;
}
