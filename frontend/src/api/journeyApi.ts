import axios from "axios";
import Location from "../components/Location";
import { Journey } from "./types";

import CONFIG from "../config.ts";

export const queryJourneys = async (
  locations: Location[],
  time: string,
  signal: AbortSignal,
): Promise<Journey[]> => {
  if (locations.length === 0) {
    return [];
  }

  let key = toKey(locations);

  const url = CONFIG.backendUrl + "traveltime/" + key + "/" + time;
  const response = await axios.get(encodeURI(url), { signal: signal });
  const journeys: Journey[] = response.data;
  return journeys;
};

const toCoordString = (location: Location): string => {
  return location.coords.join(",");
};

const toKey = (locations: Location[]): string => {
  return locations.map(toCoordString).join("_");
};
