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

const backendUrl = process.env.REACT_APP_DEV
  ? "http://localhost:3001/"
  : "https://tfl-travel-time-backend-polished-log-1447.fly.dev/";

const config: Config = {
  backendUrl: backendUrl,
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
