import { MapContainer, TileLayer } from 'react-leaflet'
import { Marker, Pane } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import ChangeView from './ChangeView';

const CONFIG = require("../config.json");

const TravelTimeMap = ({ changeView, coordsList, CircleLayer }) =>
    <MapContainer bounds={CONFIG.startingBounds} scrollWheelZoom zoomControl={false} preferCanvas={true}>
        <ChangeView locs={coordsList} changeView={changeView} />
        <TileLayer
            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png"
        />
        {CircleLayer}
        {coordsList.map((c, i) =>
            <Marker position={c} key={i.toString()}>
            </Marker>
        )}
    </MapContainer>

export default TravelTimeMap;