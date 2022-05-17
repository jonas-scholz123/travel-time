import LocationCard from './components/LocationCard';
import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { Marker, Popup, Circle, Pane } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import axios from 'axios';
import { useEffect, useState } from 'react';
import ChangeView from './components/ChangeView';
import TimeCard from './components/TimeCard';

import L from 'leaflet';
import BoundsCard from './components/BoundsCard';
delete L.Icon.Default.prototype._getIconUrl;

L.Icon.Default.mergeOptions({
  iconRetinaUrl: require('leaflet/dist/images/marker-icon-2x.png'),
  iconUrl: require('leaflet/dist/images/marker-icon.png'),
  shadowUrl: require('leaflet/dist/images/marker-shadow.png'),
});

const CONFIG = require("./config.json");

const colours = CONFIG.colours;

function App() {

  // If you move this into a separate file the app gets very slow.
  // Only the javascript gods know why.
  const CircleLayer = ({ circles }) =>
    <div>
      <Pane name="circles" style={{ zIndex: 500, opacity: CONFIG.opacity }}>
        {circles}
      </Pane>
    </div>

  const [coordsList, setCoordsList] = useState([]);
  const [circles, setCircles] = useState([]);
  const [allData, setAllData] = useState({});
  const [time, setTime] = useState("18:00");
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

  // When new data is loaded, we calculate the longest paths
  // to everywhere to colour the map in correctly.
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

  // When the longest path or the bounds change,
  // we need to re-render the circles.
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

  // When the longest paths to every station have been calculated,
  // we can determine what the lowest bound should look like (so
  // that there's always a green layer on the map).
  useEffect(() => {
    let shortestTravelTime = Math.min(...Object.values(longestPaths).map(p => p.minutes))
    setBounds(oldBounds => {
      let copy = [...oldBounds];
      copy[0] = Math.min(shortestTravelTime + CONFIG.minBoundSize, copy[1]);
      return copy
    });

  }, [longestPaths])

  useEffect(() => {
    updateAllData();
    setChangeView(true);
  }, [coordsList])

  useEffect(() => {
    updateAllData();
    setChangeView(false);
  }, [time])

  const updateAllData = () => {
    if (coordsList.length === 0) {
      setAllData({});
      return;
    }

    let newData = {};

    for (const coords of coordsList) {
      // declare the data fetching function
      let key = coords.join(",") + "/" + time;
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
  }

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
          </Marker>
        )}
      </MapContainer>

      <div className="absolute top-3 left-3 w-96 z-10000">
        <LocationCard addCoords={addCoords} deleteCoords={deleteCoords} changeCoords={changeCoords} />
        {circles.length > 0 && <BoundsCard colours={colours} setBounds={setBounds} bounds={bounds} />}
        {circles.length > 0 && <TimeCard time={time} setTime={setTime} />}
      </div>
    </div>
  );
}

export default App;
