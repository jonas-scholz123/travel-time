interface Config {
  backendUrl: string;
  walkingSpeed: number;
  startingBounds: [[number, number], [number, number]];
  opacity: number;
  minBoundSize: number;
  padding: number;
  maxTravelMins: number;
  colours: string[];
}

const config: Config = {
  backendUrl: "http://localhost:3001/",
  walkingSpeed: 80,
  startingBounds: [
    [51.564956, -0.263222],
    [51.452705, 0.022491],
  ],
  opacity: 0.4,
  minBoundSize: 5,
  padding: 300,
  maxTravelMins: 100,
  colours: ["green", "yellow", "orange", "red"],
};

export default config;
