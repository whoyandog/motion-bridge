export interface Point3D {
  id: number;
  x: number;
  y: number;
  z: number;
  visibility: number;
}

export interface MocapFrame {
  timestamp: number;
  skeleton_type: string;
  landmarks: Point3D[];
}
