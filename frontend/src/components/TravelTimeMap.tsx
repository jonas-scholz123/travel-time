import React from "react";
import { MapContainer, TileLayer } from "react-leaflet";
import { Marker } from "react-leaflet";
import "leaflet/dist/leaflet.css";
import Location from "./Location.ts";
import config from "../config.ts";
import MapClickHandler from "./MapClickHandler.tsx";
import ChangeMapBounds from "./ChangeView.tsx";

const TravelTimeMap = ({ addLoc, changeView, locations, CircleLayer }) => {
  const handleMapClick = (e) => {
    const { lat, lng } = e.latlng;
    const latLongString = `${lat},${lng}`;
    const loc = new Location(latLongString, [lat, lng]);
    addLoc(loc);
  };

  console.log("locations", locations);

  return (
    <MapContainer
      bounds={config.startingBounds}
      scrollWheelZoom={true}
      zoomControl={false}
      preferCanvas={true}
    >
      {<ChangeMapBounds locations={locations} changeView={changeView} />}
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png"
      />
      {CircleLayer}
      {locations.map((l, i) => (
        <Marker position={l.coords} key={i.toString()} />
      ))}
      {<MapClickHandler onClick={handleMapClick} />}
    </MapContainer>
  );
};

export default TravelTimeMap;
