import { Circle } from "react-leaflet";

const CONFIG = require("../config.json");
const colours = CONFIG.colours;

// When the longest path or the bounds change,
// we need to re-render the circles.
const makeCircles = (longestPaths, bounds) => {
  let circles = []
  for (const [idx, bound] of bounds.entries()) {
    circles.push(
      ...longestPaths
        .filter(p => p.minutes < bound)
        .map(d => makeCircle(d, bounds[idx], colours[idx])));
  }

  // TODO: improve performance using: https://github.com/domoritz/leaflet-maskcanvas;
  return circles.reverse();
}

const makeCircle = (path, boundMins, colour) => {
  const minutes = path.minutes
  const walkingMinutes = boundMins - minutes;
  const center = [path.destination.location.x, path.destination.location.y];
  return <Circle
    center={center}
    pathOptions={{ fillColor: colour, weight: 0, fillOpacity: 1 }}
    radius={CONFIG.walkingSpeed * (walkingMinutes + 0.5)}
    key={path.destination.id + colour}
  />;
}

export default makeCircles;