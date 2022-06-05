import LocationCard from './components/LocationCard';
import { Pane } from 'react-leaflet';
import 'leaflet/dist/leaflet.css'
import axios from 'axios';
import { useEffect, useState } from 'react';
import TimeCard from './components/TimeCard';
import BackendStatusCard from './components/BackendStatusCard';
import makeCircles from './components/Circles';
import 'leaflet/dist/leaflet.css'

import L from 'leaflet';
import BoundsCard from './components/BoundsCard';
import TravelTimeMap from './components/Map';
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

  const handleBoundsChange = async (newBounds) => {
    setBounds(newBounds);
    const circles = makeCircles(allData, newBounds);
    setCircles(circles);
  }

  useEffect(() => {
    console.log("Waking up the backend...");
    axios.get(encodeURI(CONFIG.backendUrl))
      .then(_ => setBackendOk(true))
      .catch(e => console.log("Health check failed: ", e));
  }, [])

  // When the longest paths to every station have been calculated,
  // we can determine what the lowest bound should look like (so
  // that there's always a green layer on the map).
  const computeBounds = (longestPaths) => {
    let shortestTravelTime = Math.min(...longestPaths.map(p => p.minutes));
    let copy = [...bounds];
    copy[0] = Math.min(shortestTravelTime + CONFIG.minBoundSize, copy[1]);
    return copy
  }

  const fetchAllData = async () => {
    if (coordsList.length === 0) {
      return [];
    }

    let key = coordsList.map(coords => coords.join(",")).join("_");
    // declare the data fetching function
    const url = CONFIG.backendUrl + "traveltime/" + key + "/" + time;
    const response = await axios.get(encodeURI(url));
    return response.data;
  }

  const refreshState = async () => {
    const newData = await fetchAllData();
    // Cache the data.
    setAllData(newData);
    const newBounds = computeBounds(newData);
    setBounds(newBounds);
    const newCircles = makeCircles(newData, newBounds);
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


  return (
    <div className="h-screen">
      <TravelTimeMap changeView={changeView} coordsList={coordsList} CircleLayer={<CircleLayer circles={circles} />} />
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
