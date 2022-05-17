import LocationCard from './components/LocationCard';
import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { Marker, Popup, Circle, Pane } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import axios from 'axios';
import { useEffect, useState } from 'react';
import ChangeView from './components/ChangeView';

import L from 'leaflet';
import useAxiosGet from './utils';
import BoundsCard from './components/BoundsCard';
delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png'),
});

const CONFIG = require("./config.json");

const colours = ["green", "yellow", "orange", "red"]




function App() {
  const [coordsList, setCoordsList] = useState([]);
  const [circles, setCircles] = useState([]);
  const [allData, setAllData] = useState({});
  // TODO: Use top N-th percentile bounds instead.
  const [bounds, setBounds] = useState([15, 30, 45, 60]);
  const [changeView, setChangeView] = useState(true);
  const [longestPaths, setLongestPaths] = useState({});

  const addCoords = (coord) => {
    setCoordsList([...coordsList, coord]);
  }

  const changeCoords = (idx, coord) => {
    let copy = [...coordsList];
    copy[idx] = coord;
    setCoordsList(copy);
  }

  function CircleLayer({ circles }) {
    return (
      <div>
        <Pane name="circles" style={{ zIndex: 500, opacity: CONFIG.opacity }}>
          {circles}
        </Pane>
      </div>
    )
  }

  const deleteCoords = (idx) => {
    setCoordsList(coordsList.filter((_, i) => i !== idx));
  }

  const makeCircle = (path, tier) => {
    const minutes = path.minutes
    const walkingMinutes = bounds[tier] - minutes;
    const center = [path.destination.location.x, path.destination.location.y];
    return <Circle
      center={center}
      pathOptions={{ fillColor: colours[tier], weight: 0, fillOpacity: 1 }}
      radius={CONFIG.walkingSpeed * (walkingMinutes + 0.5)}
      key={path.destination.id + colours[tier]}
    />;
  }

  useEffect(() => {

    let longestPathsDict = {};

    for (const data of Object.values(allData)) {
      for (const path of data) {
        let key = path.destination.id;
        if (!(key in longestPathsDict) || longestPathsDict[key].minutes < path.minutes) {
          longestPathsDict[path.destination.id] = path;
        }
      }
    }

    setLongestPaths(longestPathsDict);

  }, [allData])

  useEffect(() => {
    let circles = []
    for (const [idx, bound] of bounds.entries()) {
      circles.push(
        ...Object.values(longestPaths)
          .filter(p => p.minutes < bound)
          .map((d, i) => makeCircle(d, idx)));
    }

    // TODO: improve performance using: https://github.com/domoritz/leaflet-maskcanvas;
    setCircles(circles.reverse());
    setChangeView(false);
  }, [longestPaths, bounds])

  useEffect(() => {
    let shortestTravelTime = Math.min(...Object.values(longestPaths).map(p => p.minutes))
    setBounds(oldBounds => {
      let copy = [...oldBounds];
      copy[0] = shortestTravelTime + CONFIG.minBoundSize;
      return copy
    });

  }, [longestPaths])

  useEffect(() => {
    if (coordsList.length === 0) {
      setAllData({});
      return;
    }

    let newData = {};

    for (const coords of coordsList) {
      // declare the data fetching function
      let key = coords.join(",") + "/18:00";
      if (key in allData) {
        newData[key] = allData[key]
      }
      else {
        const url = CONFIG.backendUrl + "traveltime/" + key;
        axios.get(encodeURI(url))
          .then(resp => {
            newData[key] = resp.data;
            setAllData(newData);
          });
      }
    }
    if (Object.keys(allData).length > coordsList.length)
      setAllData(newData);
    setChangeView(true);
  }, [coordsList])

  return (
    <div className="h-screen">
      <MapContainer bounds={CONFIG.startingBounds} scrollWheelZoom zoomControl={false} preferCanvas>
        <ChangeView locs={coordsList} changeView={changeView} />
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.fr/hot/{z}/{x}/{y}.png"
        />
        <CircleLayer circles={circles} />
        {coordsList.map((c, i) =>
          <Marker position={c} key={i.toString()}>
            <Popup>
              A pretty CSS3 popup. <br /> Easily customizable.
            </Popup>
          </Marker>
        )}
      </MapContainer>

      <div className="absolute top-3 left-3 w-96 z-10000">
        <LocationCard addCoords={addCoords} deleteCoords={deleteCoords} changeCoords={changeCoords} />
        {circles.length > 0 && <BoundsCard colours={colours} setBounds={setBounds} bounds={bounds} />}
      </div>
    </div>
  );
}

export default App;
