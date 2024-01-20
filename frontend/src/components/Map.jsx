import { MapContainer, TileLayer } from 'react-leaflet'
import { Marker } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import ChangeView from './ChangeView';
import MapClickHandler from './MapClickHandler';

const CONFIG = require("../config.json");

const TravelTimeMap = ({ addLoc, changeView, coordsList, CircleLayer }) => {
  const handleMapClick = (e) => {
    const { lat, lng } = e.latlng;
    const latLongString = `${lat},${lng}`;
    addLoc(latLongString, [lat, lng]);
  };

  return <MapContainer bounds={CONFIG.startingBounds} scrollWheelZoom zoomControl={false} preferCanvas={true}>
    <ChangeView locs={coordsList} changeView={changeView} />
    <TileLayer
      attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
      url="https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png"
    />
    {CircleLayer}
    {coordsList.map((c, i) => <Marker position={c} key={i.toString()} />)}
    <MapClickHandler onClick={handleMapClick} />
  </MapContainer>
}



export default TravelTimeMap;