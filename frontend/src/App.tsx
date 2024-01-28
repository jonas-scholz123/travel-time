import React from "react";
import LocationCard from "./components/LocationCard.tsx";
import { Pane } from "react-leaflet";
import "leaflet/dist/leaflet.css";
import axios from "axios";
import { useEffect, useState } from "react";
import TimeCard from "./components/TimeCard.tsx";
import BackendStatusCard from "./components/BackendStatusCard.tsx";
import makeCircles from "./components/Circles.tsx";
import "leaflet/dist/leaflet.css";
import { useSearchParams } from "react-router-dom";
import config from "./config.ts";

import L from "leaflet";
import BoundsCard from "./components/BoundsCard.tsx";
import TravelTimeMap from "./components/TravelTimeMap.tsx";
import Location from "./components/Location";
import { Journey } from "./api/types.ts";
import { queryJourneys } from "./api/journeyApi.ts";
import { BigScreenCards } from "./components/BigScreenCards.tsx";
import { SmallScreenCards } from "./components/SmallScreenCards.tsx";
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

  //TODO fetch from URL.
  const [locations, setLocations] = useState<Location[]>(
    getLocsFromUrl(searchParams)
  );
  const [circles, setCircles] = useState([]);
  const [journeys, setJourneys] = useState<Journey[]>([]);

  const current = new Date();
  const [time, setTime] = useState(
    current.getHours() + ":" + current.getMinutes()
  );
  // TODO: Use top N-th percentile bounds instead.
  const [bounds, setBounds] = useState([15, 30, 45, 60]);
  const [changeView, setChangeView] = useState(true);
  const [backendAwake, setBackendAwake] = useState<boolean>(false);
  const [isSmallScreen, setIsSmallScreen] = useState<boolean>(false);

  useEffect(() => {
    if (!backendAwake) {
      return;
    }
    refreshState();
    setChangeView(true);
  }, [locations, backendAwake]);

  useEffect(() => {
    if (!backendAwake) {
      return;
    }
    refreshState();
    setChangeView(false);
  }, [time, backendAwake]);

  useEffect(() => {
    console.log("Health check started, backendAwake: ", backendAwake);
    repeatHealthCheckUntilAwake(backendAwake);
  }, [backendAwake]);

  useEffect(() => {
    setArrayParam(LOCATIONS_SEARCH_PARAM, locations);
  }, [locations]);

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

  const isBackendAwake = async () => {
    try {
      await axios.get(encodeURI(config.backendUrl));
      return true;
    } catch (e) {
      return false;
    }
  };

  const repeatHealthCheckUntilAwake = async (backendAwake: boolean) => {
    const awake = await isBackendAwake();
    setBackendAwake(awake);
    if (!awake) {
      setTimeout(() => repeatHealthCheckUntilAwake(awake), 5000);
    }
  };

  const setArrayParam = (name: string, array: any[]) => {
    searchParams.delete(name);
    if (array.length > 0) {
      for (const el of array) {
        searchParams.append(name, el);
      }
    }
    setSearchParams(searchParams);
  };

  const addLoc = (location: Location) => {
    setLocations([...locations, location]);
  };

  const deleteLoc = (idx: number) => {
    setLocations(locations.filter((_, i) => i !== idx));
  };

  // When the longest paths to every station have been calculated,
  // we can determine what the lowest bound should look like (so
  // that there's always a green layer on the map).
  const computeBounds = (longestPaths: Journey[]): number[] => {
    let shortestTravelTime = Math.min(...longestPaths.map((p) => p.minutes));
    let boundsCopy = [...bounds];
    boundsCopy[0] = Math.min(
      shortestTravelTime + config.minBoundSize,
      boundsCopy[1]
    );
    return boundsCopy;
  };

  const refreshState = async () => {
    const journeys = await queryJourneys(locations, time);
    setJourneys(journeys);
    const newBounds = computeBounds(journeys);
    setBounds(newBounds);
    const newCircles = makeCircles(journeys, newBounds);
    setCircles(newCircles);
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

  const cardComponent = isSmallScreen ? SmallScreenCards : BigScreenCards;

  return (
    <div className="h-screen">
      <TravelTimeMap
        addLoc={addLoc}
        changeView={changeView}
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
