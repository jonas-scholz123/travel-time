import L from "leaflet";
import React, { useEffect, useRef } from "react";
import { Marker } from "react-leaflet";

export enum MarkerColour {
  blue = "blue",
  gold = "gold",
  red = "red",
  green = "green",
  orange = "orange",
  yellow = "yellow",
  violet = "violet",
  grey = "grey",
  black = "black",
}

export const ColouredMarker = (props: {
  [x: string]: any;
  position: [number, number];
  colour?: MarkerColour;
}) => {
  const leafletRef = useRef();

  useEffect(() => {
    (leafletRef as any)?.current?.openPopup();
  }, [props.position]);

  const { colour, ...rest } = props;

  var icon = new L.Icon({
    iconUrl: `https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-${colour}.png`,
    iconSize: [25, 41],
    iconAnchor: [12, 41],
    popupAnchor: [1, -34],
  });

  return <Marker ref={leafletRef} icon={icon} {...rest} />;
};

ColouredMarker.defaultProps = { colour: MarkerColour.blue };
