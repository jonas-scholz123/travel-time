import Card from './components/Card';
import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { Marker, Popup, Circle, Pane } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
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
const colours = ["green", "yellow", "orange", "red"]
const minutesBounds = [15, 30, 45, 60]

function App() {
  const [coordsList, setCoordsList] = useState([])
  const [circles, setCircles] = useState([])

  const addCoords = (coord) => {
    setCoordsList([...coordsList, coord])
  }

  const deleteCoords = (idx) => {
    setCoordsList(coordsList.filter((_, i) => i !== idx))
  }

  const makeCircle = (path, key) => {
    const minutes = path.minutes

    if (minutes > minutesBounds[minutesBounds.length - 1]) {
      return null;
    }

    var tier = 0;

    //console.log("bounds", minutesBounds);
    for (const [i, bound] of minutesBounds.entries()) {
      tier = i;
      if (bound > minutes) {
        break;
      }
    }

    const walkingMinutes = minutesBounds[tier] - minutes;

    const center = [path.destination.location.x, path.destination.location.y];

    return <Circle
      center={center}
      pathOptions={{ fillColor: colours[tier], weight: 0, fillOpacity: 1 }}
      radius={80 * walkingMinutes}
      key={key}
    />;
  }

  useEffect(() => {
    if (coordsList.length === 0) {
      return;
    }

    const url = "http://localhost:3001/traveltime/" + coordsList[0].join(",") + "/10:00";

    axios.get(encodeURI(url))
      .then(resp => {
        let data = resp.data;
        data.sort((a, b) => b.minutes - a.minutes);
        setCircles(data.map((d, i) => makeCircle(d, i.toString())));
      })
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
      <MapContainer bounds={startingBounds} scrollWheelZoom zoomControl={false} preferCanvas fillOpacity={0.5}>
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
        <Pane name="circles" style={{ zIndex: 500, opacity: 0.5 }}>
          {circles}
        </Pane>
      </MapContainer>
      <Card addCoords={addCoords} deleteCoords={deleteCoords} />
    </div>
  );
}

export default App;
