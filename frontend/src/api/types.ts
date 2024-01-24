export interface XYPoint {
  x: number;
  y: number;
}

export interface Destination {
  id: string;
  location: XYPoint;
  name: string;
}

export interface Journey {
  minutes: number;
  destination: Destination;
  path: any[];
}
