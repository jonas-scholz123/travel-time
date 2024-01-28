import React, { useRef, useState } from "react";
import { MapContainer, Popup, TileLayer } from "react-leaflet";
import "leaflet/dist/leaflet.css";
import Location from "./Location.ts";
import config from "../config.ts";
import MapClickHandler from "./MapClickHandler.tsx";
import ChangeMapBounds from "./ChangeView.tsx";
import MarkerClickedPopup from "./MarkerClickedPopup.tsx";
import { ColouredMarker, MarkerColour } from "./ColouredMarker.tsx";

const TravelTimeMap = ({ addLoc, changeView, locations, CircleLayer }) => {
  const [clickedLoc, setClickedLoc] = useState<Location | null>(null);

  const popupRef = useRef(null);

  const handleMapClick = (e) => {
    var { lat, lng } = e.latlng;
    const latLongString = `(${lat.toFixed(4)}, ${lng.toFixed(4)})`;
    const loc = new Location(latLongString, [lat, lng]);
    setClickedLoc(loc);
  };

  const handleMarkerClick = (e) => {
    // Need a delay here because the popup has some internal onclick logic that breaks
    // when we remove the popup from the dom.
    setTimeout(() => {
      setClickedLoc(null);
    }, 20);
  };

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
        <ColouredMarker
          position={l.coords}
          key={i.toString()}
          colour={MarkerColour.blue}
        />
      ))}

      {clickedLoc && (
        <ColouredMarker
          position={clickedLoc.coords}
          colour={MarkerColour.grey}
          eventHandlers={{ click: handleMarkerClick }}
        >
          <Popup closeOnClick={true} closeButton={false} ref={popupRef}>
            <MarkerClickedPopup
              location={clickedLoc}
              addLoc={() => addLoc(clickedLoc)}
              onPopupClick={handleMarkerClick}
              className="!bg-green-500"
            />
          </Popup>
        </ColouredMarker>
      )}
      {<MapClickHandler onClick={handleMapClick} />}
    </MapContainer>
  );
};

export default TravelTimeMap;
