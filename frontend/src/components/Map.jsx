import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { Marker, Popup, Circle, Pane } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import ChangeView from './ChangeView';

import L from 'leaflet';
delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
    iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
    iconUrl: require('leaflet/dist/images/marker-icon.png'),
    shadowUrl: require('leaflet/dist/images/marker-shadow.png'),
});

const CONFIG = require("../config.json");

// If you move this into a separate file the app gets slower.
// Only the javascript gods know why.
const CircleLayer = ({ circles }) =>
    <div>
        <Pane name="circles" style={{ zIndex: 500, opacity: CONFIG.opacity }}>
            {circles}
        </Pane>
    </div>

export default ({ changeView, coordsList, CircleLayer }) =>
    <MapContainer bounds={CONFIG.startingBounds} scrollWheelZoom zoomControl={false} preferCanvas>
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