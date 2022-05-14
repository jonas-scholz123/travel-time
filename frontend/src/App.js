import Card from './components/Card';
import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { Marker, Popup } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import icon from 'leaflet/dist/images/marker-icon.png';
import iconShadow from 'leaflet/dist/images/marker-shadow.png';
import axios from 'axios';
import { useEffect, useState } from 'react';

import L from 'leaflet';
import useAxiosGet from './utils';
delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png'),
});

const startingBounds = [[51.564956, -0.263222], [51.452705, 0.022491]];

function App() {
  const [coordsList, setCoordsList] = useState([])
  const [data, setData] = useState(null)

  const addCoords = (coord) => {
    setCoordsList([...coordsList, coord])
  }

  const deleteCoords = (idx) => {
    setCoordsList(coordsList.filter((_, i) => i !== idx))
  }

  useEffect(() => {
    if (coordsList.length === 0) {
      return;
    }
    console.log("locs", coordsList[0]);
    axios.get("https://still-sierra-43714.herokuapp.com/traveltime/" + coordsList[0].join(",") + "/10:00")
      .then(resp => setData(resp.data))
  }, [coordsList])

  function ChangeView({ locs }) {

    const map = useMap();
    if (locs.length == 0) {
      map.fitBounds(startingBounds);
      return null;
    }

    if (locs.length == 1) {
      map.fitBounds(locs);
      map.setZoom(15);
      return null;
    }

    map.fitBounds(locs);
    return null;
  }

  return (
    <div class="h-screen">
      <MapContainer bounds={startingBounds} scrollWheelZoom={true} zoomControl={false}>
        <ChangeView locs={coordsList} />
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png"
        />
        {coordsList.map((c, i) => {
          return (<Marker position={c} key={i.toString()}>
            <Popup>
              A pretty CSS3 popup. <br /> Easily customizable.
            </Popup>
          </Marker>)
        })}
      </MapContainer>
      <Card addCoords={addCoords} deleteCoords={deleteCoords} />
    </div>
  );
}

export default App;
