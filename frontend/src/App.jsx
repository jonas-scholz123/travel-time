import LocationCard from './components/LocationCard';
import { MapContainer, TileLayer, useMap } from 'react-leaflet'
import { Marker, Popup, Circle, Pane } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import axios from 'axios';
import { useEffect, useState } from 'react';
import ChangeView from './components/ChangeView';
import TimeCard from './components/TimeCard';
import BackendStatusCard from './components/BackendStatusCard';

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

  // If you move this into a separate file the app gets slower.
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
  const [backendAwake, setBackendOk] = useState(false);

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

  const handleBoundsChange = (newBounds) => {
    setBounds(newBounds);
    const circles = makeCircles(longestPaths, newBounds);
    setCircles(circles);
  }

  const makeCircle = (path, boundMins, colour) => {
    const minutes = path.minutes
    const walkingMinutes = boundMins - minutes;
    const center = [path.destination.location.x, path.destination.location.y];
    return <Circle
      center={center}
      pathOptions={{ fillColor: colour, weight: 0, fillOpacity: 1 }}
      radius={CONFIG.walkingSpeed * (walkingMinutes + 0.5)}
      key={path.destination.id + colour}
    />;
  }

  useEffect(() => {
    console.log("Waking up the backend...");
    axios.get(encodeURI(CONFIG.backendUrl))
      .then(_ => setBackendOk(true))
      .catch(e => console.log("Health check failed: ", e));

  }, [])

  // When new data is loaded, we calculate the longest paths
  // to everywhere to colour the map in correctly.
  const determineLongestPath = async (allData) => {
    let longestPathsDict = {};
    for (const data of Object.values(allData)) {
      for (const path of data) {
        let key = path.destination.id;
        if (!(key in longestPathsDict) || longestPathsDict[key].minutes < path.minutes) {
          longestPathsDict[path.destination.id] = path;
        }
      }
    }

    return longestPathsDict;
  }

  // When the longest path or the bounds change,
  // we need to re-render the circles.
  const makeCircles = (longestPaths, bounds) => {
    let circles = []
    for (const [idx, bound] of bounds.entries()) {
      circles.push(
        ...Object.values(longestPaths)
          .filter(p => p.minutes < bound)
          .map(d => makeCircle(d, bounds[idx], colours[idx])));
    }

    // TODO: improve performance using: https://github.com/domoritz/leaflet-maskcanvas;
    return circles.reverse();
  }

  // When the longest paths to every station have been calculated,
  // we can determine what the lowest bound should look like (so
  // that there's always a green layer on the map).
  const computeBounds = (longestPaths) => {
    let shortestTravelTime = Math.min(...Object.values(longestPaths).map(p => p.minutes))
    let copy = [...bounds];
    copy[0] = Math.min(shortestTravelTime + CONFIG.minBoundSize, copy[1]);
    return copy
  }

  const refreshState = async () => {
    const allData = await fetchAllData();
    // Cache the data.
    setAllData(allData);
    const longestPathsDict = await determineLongestPath(allData);
    setLongestPaths(longestPathsDict);
    const newBounds = computeBounds(longestPathsDict);
    setBounds(newBounds);
    const newCircles = makeCircles(longestPathsDict, newBounds);
    setCircles(newCircles);
  }

  useEffect(() => {
    refreshState()
    setChangeView(true);
  }, [coordsList])

  useEffect(() => {
    refreshState();
    setChangeView(false);
  }, [time])

  const fetchAllData = async () => {
    if (coordsList.length === 0) {
      return {};
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
        const response = await axios.get(encodeURI(url));
        newData[key] = response.data;
        return newData;
      }
    }
    if (Object.keys(allData).length > coordsList.length)
      return newData;
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

      <div className="absolute top-0 left-0 md:top-3 md:left-3 md:w-96 w-full z-10000">
        <LocationCard addCoords={addCoords} deleteCoords={deleteCoords} changeCoords={changeCoords} />
        {circles.length > 0 && <BoundsCard colours={colours} setBounds={handleBoundsChange} bounds={bounds} />}
        {circles.length > 0 && <TimeCard time={time} setTime={setTime} />}
        {!backendAwake && <BackendStatusCard addCoords={addCoords} deleteCoords={deleteCoords} changeCoords={changeCoords} />}
      </div>
    </div>
  );
}

export default App;