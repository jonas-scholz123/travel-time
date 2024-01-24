import { useMap } from "react-leaflet";

import config from "../config.ts";
import Location from "./Location.ts";
import { ReactElement } from "react";

const ChangeMapBounds = (props: {
  locations: Location[];
  changeView: boolean;
}): ReactElement => {
  const map = useMap();

  if (!props.changeView) {
    return;
  }

  const coords = props.locations.map((l) => l.coords);

  if (coords.length == 0) {
    map.fitBounds(config.startingBounds);
    return;
  }

  if (coords.length == 1) {
    map.setView(coords[0], 14);
    return;
  }

  map.fitBounds(coords, { padding: [config.padding, config.padding] });
  return;
};

export default ChangeMapBounds;
