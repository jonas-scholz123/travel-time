import React from "react";
import { Pane } from "react-leaflet";
import "leaflet/dist/leaflet.css";
import { useEffect, useState } from "react";
import "leaflet/dist/leaflet.css";
import { useSearchParams } from "react-router-dom";
import config from "./config.ts";

import L from "leaflet";
import TravelTimeMap from "./components/TravelTimeMap.tsx";
import Location from "./components/Location";
import { Journey } from "./api/types.ts";
import { queryJourneys } from "./api/journeyApi.ts";
import { BigScreenCards } from "./components/BigScreenCards.tsx";
import { SmallScreenCards } from "./components/SmallScreenCards.tsx";
import makeCircles from "./components/Circles.tsx";
import { HealthChecker } from "./components/HealthChecker.tsx";
delete (L.Icon.Default.prototype as any)._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl: require("leaflet/dist/images/marker-icon-2x.png"),
  iconUrl: require("leaflet/dist/images/marker-icon.png"),
  shadowUrl: require("leaflet/dist/images/marker-shadow.png"),
});

const colours = config.colours;
const LOCATIONS_SEARCH_PARAM = "locations";

const getLocsFromUrl = (searchParams: URLSearchParams): Location[] => {
  return searchParams
    .getAll(LOCATIONS_SEARCH_PARAM)
    .map((locString) => Location.fromString(locString));
};

// When the longest paths to every station have been calculated,
// we can determine what the lowest bound should look like (so
// that there's always a green layer on the map).
const computeBounds = (oldBounds: number[], journeys: Journey[]): number[] => {
  if (journeys.length === 0) {
    return oldBounds;
  }

  let shortestTravelTime = Math.min(...journeys.map((p) => p.minutes));
  let newBounds = [...oldBounds];
  newBounds[0] = shortestTravelTime + config.minBoundSize;

  // Iterate over the bounds and make sure they're all at least
  // minBoundSize apart.
  //
  for (let i = 1; i < newBounds.length; i++) {
    if (newBounds[i] - newBounds[i - 1] < config.minBoundSize) {
      newBounds[i] = newBounds[i - 1] + config.minBoundSize;
    }
  }

  return newBounds;
};

function App() {
  // If you move this into a separate file the app gets slower.
  // Only the javascript gods know why.
  const CircleLayer = ({ circles }) => (
    <div>
      <Pane name="circles" style={{ zIndex: 500, opacity: config.opacity }}>
        {circles}
      </Pane>
    </div>
  );

  const [searchParams, setSearchParams] = useSearchParams();
  const [locations, setLocations] = useState<Location[]>(
    getLocsFromUrl(searchParams),
  );
  const [circles, setCircles] = useState([]);
  const [journeys, setJourneys] = useState<Journey[]>([]);

  const current = new Date();
  const [time, setTime] = useState(
    current.getHours() + ":" + current.getMinutes(),
  );
  // TODO: Use top N-th percentile bounds instead.
  const [bounds, setBounds] = useState([15, 30, 45, 60]);
  const [changeView, setChangeView] = useState(true);
  const [backendAwake, setBackendAwake] = useState<boolean>(false);
  const [isSmallScreen, setIsSmallScreen] = useState<boolean>(false);

  useEffect(() => {
    setChangeView(true);
  }, [locations, backendAwake]);

  useEffect(() => {
    const abortController = new AbortController();

    const refreshJourneys = async () => {
      try {
        const journeys = await queryJourneys(
          locations,
          time,
          abortController.signal,
        );
        setJourneys(journeys);
      } catch (e) {
        if (e.name === "AbortError") {
          //Aborted request, do nothing.
        }
      }
    };

    refreshJourneys();
    return () => {
      // Clean up stale requests.
      abortController.abort();
    };
  }, [locations, time]);

  useEffect(() => {
    // We do both in the same function to avoid re-rendering the circles twice, which a
    // separate useEffect would do because it depends on both journeys and bounds.
    const computeBoundsAndCircles = (
      oldBounds: number[],
      journeys: Journey[],
    ): number[] => {
      const newBounds = computeBounds(oldBounds, journeys);
      const newCircles = makeCircles(journeys, newBounds);
      setCircles(newCircles);
      return newBounds;
    };

    setBounds((oldBounds) => computeBoundsAndCircles(oldBounds, journeys));
  }, [journeys]);

  useEffect(() => {
    searchParams.delete(LOCATIONS_SEARCH_PARAM);
    if (locations.length > 0) {
      for (const loc of locations) {
        searchParams.append(LOCATIONS_SEARCH_PARAM, loc.toString());
      }
    }
    setSearchParams(searchParams);
  }, [locations, searchParams, setSearchParams]);

  useEffect(() => {
    const checkScreenSize = () => {
      const smallScreenThreshold = 420;
      setIsSmallScreen(window.innerWidth < smallScreenThreshold);
    };
    checkScreenSize();
    window.addEventListener("resize", checkScreenSize);
    return () => {
      window.removeEventListener("resize", checkScreenSize);
    };
  }, []);

  const handleBoundsChange = async (newBounds: number[]) => {
    setBounds(newBounds);
    const circles = makeCircles(journeys, newBounds);
    setCircles(circles);
  };

  const addLoc = (location: Location) => {
    setLocations([...locations, location]);
  };

  const deleteLoc = (idx: number) => {
    setLocations(locations.filter((_, i) => i !== idx));
  };

  const cardProps = {
    locations,
    circles,
    colours,
    bounds,
    time,
    backendAwake,
    addLoc,
    deleteLoc,
    handleBoundsChange,
    setTime,
  };

  return (
    <div className="h-screen">
      <HealthChecker setBackendAwake={setBackendAwake} />
      <TravelTimeMap
        addLoc={addLoc}
        changeView={changeView}
        setChangeView={setChangeView}
        locations={locations}
        CircleLayer={<CircleLayer circles={circles} />}
      />

      {isSmallScreen ? (
        <SmallScreenCards {...cardProps} />
      ) : (
        <BigScreenCards {...cardProps} />
      )}
    </div>
  );
}

export default App;
