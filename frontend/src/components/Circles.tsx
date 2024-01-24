import React from "react";
import { Circle } from "react-leaflet";
import { Journey } from "../api/types";

import CONFIG from "../config.ts";
const colours = CONFIG.colours;

// When the longest path or the bounds change,
// we need to re-render the circles.
const makeCircles = (
  journeys: Journey[],
  bounds: number[]
): React.ReactElement[] => {
  let circles = [];
  for (const [idx, bound] of bounds.entries()) {
    circles.push(
      ...journeys
        .filter((p) => p.minutes < bound)
        .map((journey) => makeCircle(journey, bounds[idx], colours[idx]))
    );
  }

  // TODO: improve performance using: https://github.com/domoritz/leaflet-maskcanvas;
  return circles.reverse();
};

const makeCircle = (
  journey: Journey,
  boundMins: number,
  colour: string
): React.ReactElement => {
  const minutes = journey.minutes;
  const walkingMinutes = boundMins - minutes;
  const center: [number, number] = [
    journey.destination.location.x,
    journey.destination.location.y,
  ];
  return (
    <Circle
      center={center}
      pathOptions={{ fillColor: colour, weight: 0, fillOpacity: 1 }}
      radius={CONFIG.walkingSpeed * (walkingMinutes + 0.5)}
      key={journey.destination.id + colour}
    />
  );
};

export default makeCircles;
